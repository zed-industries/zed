#[cfg(feature = "Win32_System_Com")]
pub trait IDynamicPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RemoteHost(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExternalPort(&self) -> windows_core::Result<i32>;
    fn Protocol(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InternalPort(&self) -> windows_core::Result<i32>;
    fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LeaseDuration(&self) -> windows_core::Result<i32>;
    fn RenewLease(&self, lleasedurationdesired: i32) -> windows_core::Result<i32>;
    fn EditInternalClient(&self, bstrinternalclient: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enable(&self, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EditDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDynamicPortMapping {}
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMapping_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>() -> IDynamicPortMapping_Vtbl {
        unsafe extern "system" fn ExternalIPAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::ExternalIPAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHost<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::RemoteHost(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::ExternalPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::Protocol(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::InternalPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::InternalClient(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseDuration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::LeaseDuration(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewLease<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMapping_Impl::RenewLease(this, core::mem::transmute_copy(&lleasedurationdesired)) {
                Ok(ok__) => {
                    core::ptr::write(pleasedurationreturned, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDynamicPortMapping_Impl::EditInternalClient(this, core::mem::transmute(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDynamicPortMapping_Impl::Enable(this, core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDynamicPortMapping_Impl::EditDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linternalport: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDynamicPortMapping_Impl::EditInternalPort(this, core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, Impl, OFFSET>,
            RemoteHost: RemoteHost::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            InternalClient: InternalClient::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            LeaseDuration: LeaseDuration::<Identity, Impl, OFFSET>,
            RenewLease: RenewLease::<Identity, Impl, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            EditDescription: EditDescription::<Identity, Impl, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicPortMapping as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDynamicPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<IDynamicPortMapping>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Add(&self, bstrremotehost: &windows_core::BSTR, lexternalport: i32, bstrprotocol: &windows_core::BSTR, linternalport: i32, bstrinternalclient: &windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &windows_core::BSTR, lleaseduration: i32) -> windows_core::Result<IDynamicPortMapping>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDynamicPortMappingCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IDynamicPortMappingCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>() -> IDynamicPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMappingCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: core::mem::MaybeUninit<windows_core::BSTR>, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, ppdpm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMappingCollection_Impl::get_Item(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)) {
                Ok(ok__) => {
                    core::ptr::write(ppdpm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMappingCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: core::mem::MaybeUninit<windows_core::BSTR>, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDynamicPortMappingCollection_Impl::Remove(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDynamicPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: core::mem::MaybeUninit<windows_core::BSTR>, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, linternalport: i32, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, lleaseduration: i32, ppdpm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDynamicPortMappingCollection_Impl::Add(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&linternalport), core::mem::transmute(&bstrinternalclient), core::mem::transmute_copy(&benabled), core::mem::transmute(&bstrdescription), core::mem::transmute_copy(&lleaseduration)) {
                Ok(ok__) => {
                    core::ptr::write(ppdpm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDynamicPortMappingCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEnumNetConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<INetConnection>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetConnection>;
}
impl windows_core::RuntimeName for IEnumNetConnection {}
impl IEnumNetConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: isize>() -> IEnumNetConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetConnection as windows_core::Interface>::IID
    }
}
pub trait IEnumNetSharingEveryConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut windows_core::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingEveryConnection>;
}
impl windows_core::RuntimeName for IEnumNetSharingEveryConnection {}
impl IEnumNetSharingEveryConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>() -> IEnumNetSharingEveryConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingEveryConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingEveryConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingEveryConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingEveryConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetSharingEveryConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingEveryConnection as windows_core::Interface>::IID
    }
}
pub trait IEnumNetSharingPortMapping_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut windows_core::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingPortMapping>;
}
impl windows_core::RuntimeName for IEnumNetSharingPortMapping {}
impl IEnumNetSharingPortMapping_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>() -> IEnumNetSharingPortMapping_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPortMapping_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPortMapping_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPortMapping_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetSharingPortMapping_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingPortMapping as windows_core::Interface>::IID
    }
}
pub trait IEnumNetSharingPrivateConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut windows_core::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingPrivateConnection>;
}
impl windows_core::RuntimeName for IEnumNetSharingPrivateConnection {}
impl IEnumNetSharingPrivateConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>() -> IEnumNetSharingPrivateConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPrivateConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPrivateConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPrivateConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPrivateConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetSharingPrivateConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingPrivateConnection as windows_core::Interface>::IID
    }
}
pub trait IEnumNetSharingPublicConnection_Impl: Sized {
    fn Next(&self, celt: u32, rgvar: *mut windows_core::VARIANT, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumNetSharingPublicConnection>;
}
impl windows_core::RuntimeName for IEnumNetSharingPublicConnection {}
impl IEnumNetSharingPublicConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>() -> IEnumNetSharingPublicConnection_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPublicConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPublicConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumNetSharingPublicConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetSharingPublicConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumNetSharingPublicConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumNetSharingPublicConnection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INATEventManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetExternalIPAddressCallback(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetNumberOfEntriesCallback(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INATEventManager {}
#[cfg(feature = "Win32_System_Com")]
impl INATEventManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATEventManager_Impl, const OFFSET: isize>() -> INATEventManager_Vtbl {
        unsafe extern "system" fn SetExternalIPAddressCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATEventManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INATEventManager_Impl::SetExternalIPAddressCallback(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATEventManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INATEventManager_Impl::SetNumberOfEntriesCallback(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetExternalIPAddressCallback: SetExternalIPAddressCallback::<Identity, Impl, OFFSET>,
            SetNumberOfEntriesCallback: SetNumberOfEntriesCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INATEventManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait INATExternalIPAddressCallback_Impl: Sized {
    fn NewExternalIPAddress(&self, bstrnewexternalipaddress: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INATExternalIPAddressCallback {}
impl INATExternalIPAddressCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATExternalIPAddressCallback_Impl, const OFFSET: isize>() -> INATExternalIPAddressCallback_Vtbl {
        unsafe extern "system" fn NewExternalIPAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATExternalIPAddressCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewexternalipaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INATExternalIPAddressCallback_Impl::NewExternalIPAddress(this, core::mem::transmute(&bstrnewexternalipaddress)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewExternalIPAddress: NewExternalIPAddress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INATExternalIPAddressCallback as windows_core::Interface>::IID
    }
}
pub trait INATNumberOfEntriesCallback_Impl: Sized {
    fn NewNumberOfEntries(&self, lnewnumberofentries: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INATNumberOfEntriesCallback {}
impl INATNumberOfEntriesCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>() -> INATNumberOfEntriesCallback_Vtbl {
        unsafe extern "system" fn NewNumberOfEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INATNumberOfEntriesCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewnumberofentries: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INATNumberOfEntriesCallback_Impl::NewNumberOfEntries(this, core::mem::transmute_copy(&lnewnumberofentries)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewNumberOfEntries: NewNumberOfEntries::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INATNumberOfEntriesCallback as windows_core::Interface>::IID
    }
}
pub trait INetConnection_Impl: Sized {
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Duplicate(&self, pszwduplicatename: &windows_core::PCWSTR) -> windows_core::Result<INetConnection>;
    fn GetProperties(&self) -> windows_core::Result<*mut NETCON_PROPERTIES>;
    fn GetUiObjectClassId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Rename(&self, pszwnewname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetConnection {}
impl INetConnection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>() -> INetConnection_Vtbl {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnection_Impl::Connect(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnection_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnection_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Duplicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwduplicatename: windows_core::PCWSTR, ppcon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnection_Impl::Duplicate(this, core::mem::transmute(&pszwduplicatename)) {
                Ok(ok__) => {
                    core::ptr::write(ppcon, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnection_Impl::GetProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(ppprops, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiObjectClassId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnection_Impl::GetUiObjectClassId(this) {
                Ok(ok__) => {
                    core::ptr::write(pclsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwnewname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnection_Impl::Rename(this, core::mem::transmute(&pszwnewname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Duplicate: Duplicate::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetUiObjectClassId: GetUiObjectClassId::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnection as windows_core::Interface>::IID
    }
}
pub trait INetConnectionConnectUi_Impl: Sized {
    fn SetConnection(&self, pcon: Option<&INetConnection>) -> windows_core::Result<()>;
    fn Connect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>;
    fn Disconnect(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INetConnectionConnectUi {}
impl INetConnectionConnectUi_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>() -> INetConnectionConnectUi_Vtbl {
        unsafe extern "system" fn SetConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcon: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnectionConnectUi_Impl::SetConnection(this, windows_core::from_raw_borrowed(&pcon)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnectionConnectUi_Impl::Connect(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionConnectUi_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetConnectionConnectUi_Impl::Disconnect(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetConnection: SetConnection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnectionConnectUi as windows_core::Interface>::IID
    }
}
pub trait INetConnectionManager_Impl: Sized {
    fn EnumConnections(&self, flags: NETCONMGR_ENUM_FLAGS) -> windows_core::Result<IEnumNetConnection>;
}
impl windows_core::RuntimeName for INetConnectionManager {}
impl INetConnectionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionManager_Impl, const OFFSET: isize>() -> INetConnectionManager_Vtbl {
        unsafe extern "system" fn EnumConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionManager_Impl::EnumConnections(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumConnections: EnumConnections::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnectionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetConnectionProps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Guid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<NETCON_STATUS>;
    fn MediaType(&self) -> windows_core::Result<NETCON_MEDIATYPE>;
    fn Characteristics(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetConnectionProps {}
#[cfg(feature = "Win32_System_Com")]
impl INetConnectionProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>() -> INetConnectionProps_Vtbl {
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionProps_Impl::Guid(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrguid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionProps_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionProps_Impl::DeviceName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdevicename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionProps_Impl::Status(this) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionProps_Impl::MediaType(this) {
                Ok(ok__) => {
                    core::ptr::write(pmediatype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetConnectionProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetConnectionProps_Impl::Characteristics(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Guid: Guid::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            DeviceName: DeviceName::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Characteristics: Characteristics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetConnectionProps as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwAuthorizedApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProcessImageFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProcessImageFileName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwAuthorizedApplication {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>() -> INetFwAuthorizedApplication_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplication_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplication_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn ProcessImageFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplication_Impl::ProcessImageFileName(this) {
                Ok(ok__) => {
                    core::ptr::write(imagefilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplication_Impl::SetProcessImageFileName(this, core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplication_Impl::IpVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(ipversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplication_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplication_Impl::Scope(this) {
                Ok(ok__) => {
                    core::ptr::write(scope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplication_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplication_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(remoteaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplication_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplication_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplication_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Identity, Impl, OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplication as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwAuthorizedApplications_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, app: Option<&INetFwAuthorizedApplication>) -> windows_core::Result<()>;
    fn Remove(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<INetFwAuthorizedApplication>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwAuthorizedApplications {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwAuthorizedApplications_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>() -> INetFwAuthorizedApplications_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplications_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, app: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplications_Impl::Add(this, windows_core::from_raw_borrowed(&app)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwAuthorizedApplications_Impl::Remove(this, core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>, app: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplications_Impl::Item(this, core::mem::transmute(&imagefilename)) {
                Ok(ok__) => {
                    core::ptr::write(app, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwAuthorizedApplications_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwAuthorizedApplications_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(newenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwAuthorizedApplications as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwIcmpSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AllowOutboundDestinationUnreachable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundDestinationUnreachable(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowRedirect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowRedirect(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundEchoRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundEchoRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundTimeExceeded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundTimeExceeded(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundParameterProblem(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundParameterProblem(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundSourceQuench(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundSourceQuench(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundRouterRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundRouterRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundTimestampRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundTimestampRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowInboundMaskRequest(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowInboundMaskRequest(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowOutboundPacketTooBig(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAllowOutboundPacketTooBig(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwIcmpSettings {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwIcmpSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>() -> INetFwIcmpSettings_Vtbl {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowOutboundDestinationUnreachable(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowOutboundDestinationUnreachable(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowRedirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowRedirect(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowRedirect(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowInboundEchoRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowInboundEchoRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowOutboundTimeExceeded(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowOutboundTimeExceeded(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowOutboundParameterProblem(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowOutboundParameterProblem(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowOutboundSourceQuench(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowOutboundSourceQuench(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowInboundRouterRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowInboundRouterRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowInboundTimestampRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowInboundTimestampRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowInboundMaskRequest(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowInboundMaskRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwIcmpSettings_Impl::AllowOutboundPacketTooBig(this) {
                Ok(ok__) => {
                    core::ptr::write(allow, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwIcmpSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwIcmpSettings_Impl::SetAllowOutboundPacketTooBig(this, core::mem::transmute_copy(&allow)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Identity, Impl, OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Identity, Impl, OFFSET>,
            AllowRedirect: AllowRedirect::<Identity, Impl, OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Identity, Impl, OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Identity, Impl, OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Identity, Impl, OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Identity, Impl, OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Identity, Impl, OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Identity, Impl, OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Identity, Impl, OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Identity, Impl, OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Identity, Impl, OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Identity, Impl, OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Identity, Impl, OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Identity, Impl, OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Identity, Impl, OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwIcmpSettings as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwMgr_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LocalPolicy(&self) -> windows_core::Result<INetFwPolicy>;
    fn CurrentProfileType(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE>;
    fn RestoreDefaults(&self) -> windows_core::Result<()>;
    fn IsPortAllowed(&self, imagefilename: &windows_core::BSTR, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: &windows_core::BSTR, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut windows_core::VARIANT, restricted: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn IsIcmpTypeAllowed(&self, ipversion: NET_FW_IP_VERSION, localaddress: &windows_core::BSTR, r#type: u8, allowed: *mut windows_core::VARIANT, restricted: *mut windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwMgr {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: isize>() -> INetFwMgr_Vtbl {
        unsafe extern "system" fn LocalPolicy<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwMgr_Impl::LocalPolicy(this) {
                Ok(ok__) => {
                    core::ptr::write(localpolicy, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfileType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwMgr_Impl::CurrentProfileType(this) {
                Ok(ok__) => {
                    core::ptr::write(profiletype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDefaults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwMgr_Impl::RestoreDefaults(this).into()
        }
        unsafe extern "system" fn IsPortAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: core::mem::MaybeUninit<windows_core::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut core::mem::MaybeUninit<windows_core::VARIANT>, restricted: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwMgr_Impl::IsPortAllowed(this, core::mem::transmute(&imagefilename), core::mem::transmute_copy(&ipversion), core::mem::transmute_copy(&portnumber), core::mem::transmute(&localaddress), core::mem::transmute_copy(&ipprotocol), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwMgr_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: core::mem::MaybeUninit<windows_core::BSTR>, r#type: u8, allowed: *mut core::mem::MaybeUninit<windows_core::VARIANT>, restricted: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwMgr_Impl::IsIcmpTypeAllowed(this, core::mem::transmute_copy(&ipversion), core::mem::transmute(&localaddress), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LocalPolicy: LocalPolicy::<Identity, Impl, OFFSET>,
            CurrentProfileType: CurrentProfileType::<Identity, Impl, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, Impl, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, Impl, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwMgr as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwOpenPort_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Protocol(&self) -> windows_core::Result<NET_FW_IP_PROTOCOL>;
    fn SetProtocol(&self, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Port(&self) -> windows_core::Result<i32>;
    fn SetPort(&self, portnumber: i32) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BuiltIn(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwOpenPort {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>() -> INetFwOpenPort_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::IpVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(ipversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::Protocol(this) {
                Ok(ok__) => {
                    core::ptr::write(ipprotocol, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetProtocol(this, core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Port<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::Port(this) {
                Ok(ok__) => {
                    core::ptr::write(portnumber, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetPort(this, core::mem::transmute_copy(&portnumber)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::Scope(this) {
                Ok(ok__) => {
                    core::ptr::write(scope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(remoteaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPort_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn BuiltIn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPort_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, builtin: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPort_Impl::BuiltIn(this) {
                Ok(ok__) => {
                    core::ptr::write(builtin, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            SetProtocol: SetProtocol::<Identity, Impl, OFFSET>,
            Port: Port::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            BuiltIn: BuiltIn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPort as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwOpenPorts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, port: Option<&INetFwOpenPort>) -> windows_core::Result<()>;
    fn Remove(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<()>;
    fn Item(&self, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::Result<INetFwOpenPort>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwOpenPorts {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwOpenPorts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>() -> INetFwOpenPorts_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPorts_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, port: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPorts_Impl::Add(this, windows_core::from_raw_borrowed(&port)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwOpenPorts_Impl::Remove(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPorts_Impl::Item(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)) {
                Ok(ok__) => {
                    core::ptr::write(openport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwOpenPorts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwOpenPorts_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(newenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwOpenPorts as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwPolicy_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfile(&self) -> windows_core::Result<INetFwProfile>;
    fn GetProfileByType(&self, profiletype: NET_FW_PROFILE_TYPE) -> windows_core::Result<INetFwProfile>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy_Impl, const OFFSET: isize>() -> INetFwPolicy_Vtbl {
        unsafe extern "system" fn CurrentProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy_Impl::CurrentProfile(this) {
                Ok(ok__) => {
                    core::ptr::write(profile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileByType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy_Impl::GetProfileByType(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(profile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentProfile: CurrentProfile::<Identity, Impl, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwPolicy2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentProfileTypes(&self) -> windows_core::Result<i32>;
    fn get_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_FirewallEnabled(&self, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<windows_core::VARIANT>;
    fn put_ExcludedInterfaces(&self, profiletype: NET_FW_PROFILE_TYPE2, interfaces: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn get_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_BlockAllInboundTraffic(&self, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_NotificationsDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_UnicastResponsesToMulticastBroadcastDisabled(&self, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Rules(&self) -> windows_core::Result<INetFwRules>;
    fn ServiceRestriction(&self) -> windows_core::Result<INetFwServiceRestriction>;
    fn EnableRuleGroup(&self, profiletypesbitmask: i32, group: &windows_core::BSTR, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsRuleGroupEnabled(&self, profiletypesbitmask: i32, group: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RestoreLocalFirewallDefaults(&self) -> windows_core::Result<()>;
    fn get_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION>;
    fn put_DefaultInboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()>;
    fn get_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2) -> windows_core::Result<NET_FW_ACTION>;
    fn put_DefaultOutboundAction(&self, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::Result<()>;
    fn get_IsRuleGroupCurrentlyEnabled(&self, group: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LocalPolicyModifyState(&self) -> windows_core::Result<NET_FW_MODIFY_STATE>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwPolicy2 {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwPolicy2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>() -> INetFwPolicy2_Vtbl {
        unsafe extern "system" fn CurrentProfileTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::CurrentProfileTypes(this) {
                Ok(ok__) => {
                    core::ptr::write(profiletypesbitmask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_FirewallEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_FirewallEnabled(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_FirewallEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_FirewallEnabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn get_ExcludedInterfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(interfaces, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ExcludedInterfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype), core::mem::transmute(&interfaces)).into()
        }
        unsafe extern "system" fn get_BlockAllInboundTraffic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(block, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_BlockAllInboundTraffic<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&block)).into()
        }
        unsafe extern "system" fn get_NotificationsDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(disabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_NotificationsDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn get_UnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(disabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_UnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn Rules<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::Rules(this) {
                Ok(ok__) => {
                    core::ptr::write(rules, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceRestriction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicerestriction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::ServiceRestriction(this) {
                Ok(ok__) => {
                    core::ptr::write(servicerestriction, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: core::mem::MaybeUninit<windows_core::BSTR>, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::EnableRuleGroup(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group), core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: core::mem::MaybeUninit<windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::IsRuleGroupEnabled(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group)) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::RestoreLocalFirewallDefaults(this).into()
        }
        unsafe extern "system" fn get_DefaultInboundAction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(action, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_DefaultInboundAction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn get_DefaultOutboundAction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    core::ptr::write(action, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_DefaultOutboundAction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwPolicy2_Impl::put_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn get_IsRuleGroupCurrentlyEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: core::mem::MaybeUninit<windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::get_IsRuleGroupCurrentlyEnabled(this, core::mem::transmute(&group)) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwPolicy2_Impl::LocalPolicyModifyState(this) {
                Ok(ok__) => {
                    core::ptr::write(modifystate, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentProfileTypes: CurrentProfileTypes::<Identity, Impl, OFFSET>,
            get_FirewallEnabled: get_FirewallEnabled::<Identity, Impl, OFFSET>,
            put_FirewallEnabled: put_FirewallEnabled::<Identity, Impl, OFFSET>,
            get_ExcludedInterfaces: get_ExcludedInterfaces::<Identity, Impl, OFFSET>,
            put_ExcludedInterfaces: put_ExcludedInterfaces::<Identity, Impl, OFFSET>,
            get_BlockAllInboundTraffic: get_BlockAllInboundTraffic::<Identity, Impl, OFFSET>,
            put_BlockAllInboundTraffic: put_BlockAllInboundTraffic::<Identity, Impl, OFFSET>,
            get_NotificationsDisabled: get_NotificationsDisabled::<Identity, Impl, OFFSET>,
            put_NotificationsDisabled: put_NotificationsDisabled::<Identity, Impl, OFFSET>,
            get_UnicastResponsesToMulticastBroadcastDisabled: get_UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            put_UnicastResponsesToMulticastBroadcastDisabled: put_UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            ServiceRestriction: ServiceRestriction::<Identity, Impl, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, Impl, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, Impl, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, Impl, OFFSET>,
            get_DefaultInboundAction: get_DefaultInboundAction::<Identity, Impl, OFFSET>,
            put_DefaultInboundAction: put_DefaultInboundAction::<Identity, Impl, OFFSET>,
            get_DefaultOutboundAction: get_DefaultOutboundAction::<Identity, Impl, OFFSET>,
            put_DefaultOutboundAction: put_DefaultOutboundAction::<Identity, Impl, OFFSET>,
            get_IsRuleGroupCurrentlyEnabled: get_IsRuleGroupCurrentlyEnabled::<Identity, Impl, OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwPolicy2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProduct_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RuleCategories(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetRuleCategories(&self, rulecategories: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathToSignedProductExe(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwProduct {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwProduct_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: isize>() -> INetFwProduct_Vtbl {
        unsafe extern "system" fn RuleCategories<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProduct_Impl::RuleCategories(this) {
                Ok(ok__) => {
                    core::ptr::write(rulecategories, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleCategories<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwProduct_Impl::SetRuleCategories(this, core::mem::transmute(&rulecategories)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProduct_Impl::DisplayName(this) {
                Ok(ok__) => {
                    core::ptr::write(displayname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwProduct_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
        }
        unsafe extern "system" fn PathToSignedProductExe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProduct_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProduct_Impl::PathToSignedProductExe(this) {
                Ok(ok__) => {
                    core::ptr::write(path, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RuleCategories: RuleCategories::<Identity, Impl, OFFSET>,
            SetRuleCategories: SetRuleCategories::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProduct as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProducts_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Register(&self, product: Option<&INetFwProduct>) -> windows_core::Result<windows_core::IUnknown>;
    fn Item(&self, index: i32) -> windows_core::Result<INetFwProduct>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwProducts {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwProducts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: isize>() -> INetFwProducts_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProducts_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, product: *mut core::ffi::c_void, registration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProducts_Impl::Register(this, windows_core::from_raw_borrowed(&product)) {
                Ok(ok__) => {
                    core::ptr::write(registration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, product: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProducts_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(product, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProducts_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProducts_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(newenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Register: Register::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProducts as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwProfile_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<NET_FW_PROFILE_TYPE>;
    fn FirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetFirewallEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExceptionsNotAllowed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetExceptionsNotAllowed(&self, notallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NotificationsDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetNotificationsDisabled(&self, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UnicastResponsesToMulticastBroadcastDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUnicastResponsesToMulticastBroadcastDisabled(&self, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RemoteAdminSettings(&self) -> windows_core::Result<INetFwRemoteAdminSettings>;
    fn IcmpSettings(&self) -> windows_core::Result<INetFwIcmpSettings>;
    fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
    fn Services(&self) -> windows_core::Result<INetFwServices>;
    fn AuthorizedApplications(&self) -> windows_core::Result<INetFwAuthorizedApplications>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwProfile {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwProfile_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>() -> INetFwProfile_Vtbl {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::FirewallEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwProfile_Impl::SetFirewallEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::ExceptionsNotAllowed(this) {
                Ok(ok__) => {
                    core::ptr::write(notallowed, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwProfile_Impl::SetExceptionsNotAllowed(this, core::mem::transmute_copy(&notallowed)).into()
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::NotificationsDisabled(this) {
                Ok(ok__) => {
                    core::ptr::write(disabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwProfile_Impl::SetNotificationsDisabled(this, core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::UnicastResponsesToMulticastBroadcastDisabled(this) {
                Ok(ok__) => {
                    core::ptr::write(disabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwProfile_Impl::SetUnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn RemoteAdminSettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteadminsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::RemoteAdminSettings(this) {
                Ok(ok__) => {
                    core::ptr::write(remoteadminsettings, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcmpSettings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmpsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::IcmpSettings(this) {
                Ok(ok__) => {
                    core::ptr::write(icmpsettings, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::GloballyOpenPorts(this) {
                Ok(ok__) => {
                    core::ptr::write(openports, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, services: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::Services(this) {
                Ok(ok__) => {
                    core::ptr::write(services, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwProfile_Impl::AuthorizedApplications(this) {
                Ok(ok__) => {
                    core::ptr::write(apps, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, Impl, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, Impl, OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Identity, Impl, OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Identity, Impl, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, Impl, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, Impl, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, Impl, OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Identity, Impl, OFFSET>,
            IcmpSettings: IcmpSettings::<Identity, Impl, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, Impl, OFFSET>,
            Services: Services::<Identity, Impl, OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwProfile as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRemoteAdminSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwRemoteAdminSettings {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRemoteAdminSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>() -> INetFwRemoteAdminSettings_Vtbl {
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRemoteAdminSettings_Impl::IpVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(ipversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRemoteAdminSettings_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRemoteAdminSettings_Impl::Scope(this) {
                Ok(ok__) => {
                    core::ptr::write(scope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRemoteAdminSettings_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRemoteAdminSettings_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(remoteaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRemoteAdminSettings_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRemoteAdminSettings_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRemoteAdminSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRemoteAdminSettings_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRemoteAdminSettings as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, desc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ApplicationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetApplicationName(&self, imagefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ServiceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceName(&self, servicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Protocol(&self) -> windows_core::Result<i32>;
    fn SetProtocol(&self, protocol: i32) -> windows_core::Result<()>;
    fn LocalPorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalPorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemotePorts(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemotePorts(&self, portnumbers: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalAddresses(&self, localaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IcmpTypesAndCodes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetIcmpTypesAndCodes(&self, icmptypesandcodes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Direction(&self) -> windows_core::Result<NET_FW_RULE_DIRECTION>;
    fn SetDirection(&self, dir: NET_FW_RULE_DIRECTION) -> windows_core::Result<()>;
    fn Interfaces(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetInterfaces(&self, interfaces: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn InterfaceTypes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInterfaceTypes(&self, interfacetypes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Grouping(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGrouping(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Profiles(&self) -> windows_core::Result<i32>;
    fn SetProfiles(&self, profiletypesbitmask: i32) -> windows_core::Result<()>;
    fn EdgeTraversal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEdgeTraversal(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Action(&self) -> windows_core::Result<NET_FW_ACTION>;
    fn SetAction(&self, action: NET_FW_ACTION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwRule {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>() -> INetFwRule_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(desc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetDescription(this, core::mem::transmute(&desc)).into()
        }
        unsafe extern "system" fn ApplicationName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::ApplicationName(this) {
                Ok(ok__) => {
                    core::ptr::write(imagefilename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetApplicationName(this, core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn ServiceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::ServiceName(this) {
                Ok(ok__) => {
                    core::ptr::write(servicename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetServiceName(this, core::mem::transmute(&servicename)).into()
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Protocol(this) {
                Ok(ok__) => {
                    core::ptr::write(protocol, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetProtocol(this, core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn LocalPorts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::LocalPorts(this) {
                Ok(ok__) => {
                    core::ptr::write(portnumbers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPorts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetLocalPorts(this, core::mem::transmute(&portnumbers)).into()
        }
        unsafe extern "system" fn RemotePorts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::RemotePorts(this) {
                Ok(ok__) => {
                    core::ptr::write(portnumbers, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemotePorts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetRemotePorts(this, core::mem::transmute(&portnumbers)).into()
        }
        unsafe extern "system" fn LocalAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::LocalAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(localaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetLocalAddresses(this, core::mem::transmute(&localaddrs)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(remoteaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::IcmpTypesAndCodes(this) {
                Ok(ok__) => {
                    core::ptr::write(icmptypesandcodes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetIcmpTypesAndCodes(this, core::mem::transmute(&icmptypesandcodes)).into()
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Direction(this) {
                Ok(ok__) => {
                    core::ptr::write(dir, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetDirection(this, core::mem::transmute_copy(&dir)).into()
        }
        unsafe extern "system" fn Interfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Interfaces(this) {
                Ok(ok__) => {
                    core::ptr::write(interfaces, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetInterfaces(this, core::mem::transmute(&interfaces)).into()
        }
        unsafe extern "system" fn InterfaceTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::InterfaceTypes(this) {
                Ok(ok__) => {
                    core::ptr::write(interfacetypes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetInterfaceTypes(this, core::mem::transmute(&interfacetypes)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Grouping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Grouping(this) {
                Ok(ok__) => {
                    core::ptr::write(context, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrouping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetGrouping(this, core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Profiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Profiles(this) {
                Ok(ok__) => {
                    core::ptr::write(profiletypesbitmask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetProfiles(this, core::mem::transmute_copy(&profiletypesbitmask)).into()
        }
        unsafe extern "system" fn EdgeTraversal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::EdgeTraversal(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetEdgeTraversal(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Action<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut NET_FW_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule_Impl::Action(this) {
                Ok(ok__) => {
                    core::ptr::write(action, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: NET_FW_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule_Impl::SetAction(this, core::mem::transmute_copy(&action)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationName: ApplicationName::<Identity, Impl, OFFSET>,
            SetApplicationName: SetApplicationName::<Identity, Impl, OFFSET>,
            ServiceName: ServiceName::<Identity, Impl, OFFSET>,
            SetServiceName: SetServiceName::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            SetProtocol: SetProtocol::<Identity, Impl, OFFSET>,
            LocalPorts: LocalPorts::<Identity, Impl, OFFSET>,
            SetLocalPorts: SetLocalPorts::<Identity, Impl, OFFSET>,
            RemotePorts: RemotePorts::<Identity, Impl, OFFSET>,
            SetRemotePorts: SetRemotePorts::<Identity, Impl, OFFSET>,
            LocalAddresses: LocalAddresses::<Identity, Impl, OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Identity, Impl, OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            SetDirection: SetDirection::<Identity, Impl, OFFSET>,
            Interfaces: Interfaces::<Identity, Impl, OFFSET>,
            SetInterfaces: SetInterfaces::<Identity, Impl, OFFSET>,
            InterfaceTypes: InterfaceTypes::<Identity, Impl, OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Grouping: Grouping::<Identity, Impl, OFFSET>,
            SetGrouping: SetGrouping::<Identity, Impl, OFFSET>,
            Profiles: Profiles::<Identity, Impl, OFFSET>,
            SetProfiles: SetProfiles::<Identity, Impl, OFFSET>,
            EdgeTraversal: EdgeTraversal::<Identity, Impl, OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Identity, Impl, OFFSET>,
            Action: Action::<Identity, Impl, OFFSET>,
            SetAction: SetAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRule2_Impl: Sized + INetFwRule_Impl {
    fn EdgeTraversalOptions(&self) -> windows_core::Result<i32>;
    fn SetEdgeTraversalOptions(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwRule2 {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule2_Impl, const OFFSET: isize>() -> INetFwRule2_Vtbl {
        unsafe extern "system" fn EdgeTraversalOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule2_Impl::EdgeTraversalOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(loptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule2_Impl::SetEdgeTraversalOptions(this, core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base__: INetFwRule_Vtbl::new::<Identity, Impl, OFFSET>(),
            EdgeTraversalOptions: EdgeTraversalOptions::<Identity, Impl, OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRule3_Impl: Sized + INetFwRule2_Impl {
    fn LocalAppPackageId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalAppPackageId(&self, wszpackageid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserOwner(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserOwner(&self, wszuserowner: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLocalUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteUserAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteUserAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoteMachineAuthorizedList(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteMachineAuthorizedList(&self, wszuserauthlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SecureFlags(&self) -> windows_core::Result<i32>;
    fn SetSecureFlags(&self, loptions: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwRule3 {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRule3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>() -> INetFwRule3_Vtbl {
        unsafe extern "system" fn LocalAppPackageId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule3_Impl::LocalAppPackageId(this) {
                Ok(ok__) => {
                    core::ptr::write(wszpackageid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule3_Impl::SetLocalAppPackageId(this, core::mem::transmute(&wszpackageid)).into()
        }
        unsafe extern "system" fn LocalUserOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule3_Impl::LocalUserOwner(this) {
                Ok(ok__) => {
                    core::ptr::write(wszuserowner, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule3_Impl::SetLocalUserOwner(this, core::mem::transmute(&wszuserowner)).into()
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule3_Impl::LocalUserAuthorizedList(this) {
                Ok(ok__) => {
                    core::ptr::write(wszuserauthlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule3_Impl::SetLocalUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule3_Impl::RemoteUserAuthorizedList(this) {
                Ok(ok__) => {
                    core::ptr::write(wszuserauthlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule3_Impl::SetRemoteUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule3_Impl::RemoteMachineAuthorizedList(this) {
                Ok(ok__) => {
                    core::ptr::write(wszuserauthlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule3_Impl::SetRemoteMachineAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn SecureFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRule3_Impl::SecureFlags(this) {
                Ok(ok__) => {
                    core::ptr::write(loptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecureFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRule3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRule3_Impl::SetSecureFlags(this, core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base__: INetFwRule2_Vtbl::new::<Identity, Impl, OFFSET>(),
            LocalAppPackageId: LocalAppPackageId::<Identity, Impl, OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Identity, Impl, OFFSET>,
            LocalUserOwner: LocalUserOwner::<Identity, Impl, OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Identity, Impl, OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Identity, Impl, OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Identity, Impl, OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Identity, Impl, OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Identity, Impl, OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Identity, Impl, OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Identity, Impl, OFFSET>,
            SecureFlags: SecureFlags::<Identity, Impl, OFFSET>,
            SetSecureFlags: SetSecureFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRule3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<INetFwRule as windows_core::Interface>::IID || iid == &<INetFwRule2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, rule: Option<&INetFwRule>) -> windows_core::Result<()>;
    fn Remove(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Item(&self, name: &windows_core::BSTR) -> windows_core::Result<INetFwRule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwRules {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwRules_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: isize>() -> INetFwRules_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRules_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRules_Impl::Add(this, windows_core::from_raw_borrowed(&rule)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwRules_Impl::Remove(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRules_Impl::Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    core::ptr::write(rule, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwRules_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(newenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwRules as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwService_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<NET_FW_SERVICE_TYPE>;
    fn Customized(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IpVersion(&self) -> windows_core::Result<NET_FW_IP_VERSION>;
    fn SetIpVersion(&self, ipversion: NET_FW_IP_VERSION) -> windows_core::Result<()>;
    fn Scope(&self) -> windows_core::Result<NET_FW_SCOPE>;
    fn SetScope(&self, scope: NET_FW_SCOPE) -> windows_core::Result<()>;
    fn RemoteAddresses(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRemoteAddresses(&self, remoteaddrs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GloballyOpenPorts(&self) -> windows_core::Result<INetFwOpenPorts>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwService {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>() -> INetFwService_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(name, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(r#type, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Customized<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, customized: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::Customized(this) {
                Ok(ok__) => {
                    core::ptr::write(customized, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::IpVersion(this) {
                Ok(ok__) => {
                    core::ptr::write(ipversion, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwService_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::Scope(this) {
                Ok(ok__) => {
                    core::ptr::write(scope, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwService_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    core::ptr::write(remoteaddrs, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwService_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(enabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwService_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwService_Impl::GloballyOpenPorts(this) {
                Ok(ok__) => {
                    core::ptr::write(openports, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Customized: Customized::<Identity, Impl, OFFSET>,
            IpVersion: IpVersion::<Identity, Impl, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
            SetScope: SetScope::<Identity, Impl, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, Impl, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwService as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServiceRestriction_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestrictService(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ServiceRestricted(&self, servicename: &windows_core::BSTR, appname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Rules(&self) -> windows_core::Result<INetFwRules>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwServiceRestriction {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwServiceRestriction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>() -> INetFwServiceRestriction_Vtbl {
        unsafe extern "system" fn RestrictService<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>, appname: core::mem::MaybeUninit<windows_core::BSTR>, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetFwServiceRestriction_Impl::RestrictService(this, core::mem::transmute(&servicename), core::mem::transmute(&appname), core::mem::transmute_copy(&restrictservice), core::mem::transmute_copy(&servicesidrestricted)).into()
        }
        unsafe extern "system" fn ServiceRestricted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>, appname: core::mem::MaybeUninit<windows_core::BSTR>, servicerestricted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwServiceRestriction_Impl::ServiceRestricted(this, core::mem::transmute(&servicename), core::mem::transmute(&appname)) {
                Ok(ok__) => {
                    core::ptr::write(servicerestricted, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServiceRestriction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwServiceRestriction_Impl::Rules(this) {
                Ok(ok__) => {
                    core::ptr::write(rules, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RestrictService: RestrictService::<Identity, Impl, OFFSET>,
            ServiceRestricted: ServiceRestricted::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServiceRestriction as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetFwServices_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, svctype: NET_FW_SERVICE_TYPE) -> windows_core::Result<INetFwService>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetFwServices {}
#[cfg(feature = "Win32_System_Com")]
impl INetFwServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: isize>() -> INetFwServices_Vtbl {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwServices_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(count, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwServices_Impl::Item(this, core::mem::transmute_copy(&svctype)) {
                Ok(ok__) => {
                    core::ptr::write(service, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetFwServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetFwServices_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(newenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetFwServices as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingConfiguration_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SharingEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SharingConnectionType(&self) -> windows_core::Result<SHARINGCONNECTIONTYPE>;
    fn DisableSharing(&self) -> windows_core::Result<()>;
    fn EnableSharing(&self, r#type: SHARINGCONNECTIONTYPE) -> windows_core::Result<()>;
    fn InternetFirewallEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisableInternetFirewall(&self) -> windows_core::Result<()>;
    fn EnableInternetFirewall(&self) -> windows_core::Result<()>;
    fn get_EnumPortMappings(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPortMappingCollection>;
    fn AddPortMapping(&self, bstrname: &windows_core::BSTR, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: &windows_core::BSTR, etargettype: ICS_TARGETTYPE) -> windows_core::Result<INetSharingPortMapping>;
    fn RemovePortMapping(&self, pmapping: Option<&INetSharingPortMapping>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingConfiguration {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingConfiguration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>() -> INetSharingConfiguration_Vtbl {
        unsafe extern "system" fn SharingEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingConfiguration_Impl::SharingEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingConnectionType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingConfiguration_Impl::SharingConnectionType(this) {
                Ok(ok__) => {
                    core::ptr::write(ptype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableSharing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingConfiguration_Impl::DisableSharing(this).into()
        }
        unsafe extern "system" fn EnableSharing<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingConfiguration_Impl::EnableSharing(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn InternetFirewallEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingConfiguration_Impl::InternetFirewallEnabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableInternetFirewall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingConfiguration_Impl::DisableInternetFirewall(this).into()
        }
        unsafe extern "system" fn EnableInternetFirewall<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingConfiguration_Impl::EnableInternetFirewall(this).into()
        }
        unsafe extern "system" fn get_EnumPortMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingConfiguration_Impl::get_EnumPortMappings(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(ppcoll, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: core::mem::MaybeUninit<windows_core::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingConfiguration_Impl::AddPortMapping(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&ucipprotocol), core::mem::transmute_copy(&usexternalport), core::mem::transmute_copy(&usinternalport), core::mem::transmute_copy(&dwoptions), core::mem::transmute(&bstrtargetnameoripaddress), core::mem::transmute_copy(&etargettype)) {
                Ok(ok__) => {
                    core::ptr::write(ppmapping, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortMapping<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmapping: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingConfiguration_Impl::RemovePortMapping(this, windows_core::from_raw_borrowed(&pmapping)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SharingEnabled: SharingEnabled::<Identity, Impl, OFFSET>,
            SharingConnectionType: SharingConnectionType::<Identity, Impl, OFFSET>,
            DisableSharing: DisableSharing::<Identity, Impl, OFFSET>,
            EnableSharing: EnableSharing::<Identity, Impl, OFFSET>,
            InternetFirewallEnabled: InternetFirewallEnabled::<Identity, Impl, OFFSET>,
            DisableInternetFirewall: DisableInternetFirewall::<Identity, Impl, OFFSET>,
            EnableInternetFirewall: EnableInternetFirewall::<Identity, Impl, OFFSET>,
            get_EnumPortMappings: get_EnumPortMappings::<Identity, Impl, OFFSET>,
            AddPortMapping: AddPortMapping::<Identity, Impl, OFFSET>,
            RemovePortMapping: RemovePortMapping::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingConfiguration as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingEveryConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingEveryConnectionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingEveryConnectionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>() -> INetSharingEveryConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingEveryConnectionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingEveryConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingEveryConnectionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingEveryConnectionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SharingInstalled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn get_EnumPublicConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPublicConnectionCollection>;
    fn get_EnumPrivateConnections(&self, flags: SHARINGCONNECTION_ENUM_FLAGS) -> windows_core::Result<INetSharingPrivateConnectionCollection>;
    fn get_INetSharingConfigurationForINetConnection(&self, pnetconnection: Option<&INetConnection>) -> windows_core::Result<INetSharingConfiguration>;
    fn EnumEveryConnection(&self) -> windows_core::Result<INetSharingEveryConnectionCollection>;
    fn get_NetConnectionProps(&self, pnetconnection: Option<&INetConnection>) -> windows_core::Result<INetConnectionProps>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingManager {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>() -> INetSharingManager_Vtbl {
        unsafe extern "system" fn SharingInstalled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinstalled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingManager_Impl::SharingInstalled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbinstalled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EnumPublicConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingManager_Impl::get_EnumPublicConnections(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(ppcoll, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EnumPrivateConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingManager_Impl::get_EnumPrivateConnections(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(ppcoll, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_INetSharingConfigurationForINetConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetconnection: *mut core::ffi::c_void, ppnetsharingconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingManager_Impl::get_INetSharingConfigurationForINetConnection(this, windows_core::from_raw_borrowed(&pnetconnection)) {
                Ok(ok__) => {
                    core::ptr::write(ppnetsharingconfiguration, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEveryConnection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingManager_Impl::EnumEveryConnection(this) {
                Ok(ok__) => {
                    core::ptr::write(ppcoll, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NetConnectionProps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetconnection: *mut core::ffi::c_void, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingManager_Impl::get_NetConnectionProps(this, windows_core::from_raw_borrowed(&pnetconnection)) {
                Ok(ok__) => {
                    core::ptr::write(ppprops, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SharingInstalled: SharingInstalled::<Identity, Impl, OFFSET>,
            get_EnumPublicConnections: get_EnumPublicConnections::<Identity, Impl, OFFSET>,
            get_EnumPrivateConnections: get_EnumPrivateConnections::<Identity, Impl, OFFSET>,
            get_INetSharingConfigurationForINetConnection: get_INetSharingConfigurationForINetConnection::<Identity, Impl, OFFSET>,
            EnumEveryConnection: EnumEveryConnection::<Identity, Impl, OFFSET>,
            get_NetConnectionProps: get_NetConnectionProps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Disable(&self) -> windows_core::Result<()>;
    fn Enable(&self) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<INetSharingPortMappingProps>;
    fn Delete(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingPortMapping {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMapping_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>() -> INetSharingPortMapping_Vtbl {
        unsafe extern "system" fn Disable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingPortMapping_Impl::Disable(this).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingPortMapping_Impl::Enable(this).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnspmp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMapping_Impl::Properties(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnspmp, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            INetSharingPortMapping_Impl::Delete(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Disable: Disable::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPortMapping as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingPortMappingCollection {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>() -> INetSharingPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPortMappingCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPortMappingProps_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IPProtocol(&self) -> windows_core::Result<u8>;
    fn ExternalPort(&self) -> windows_core::Result<i32>;
    fn InternalPort(&self) -> windows_core::Result<i32>;
    fn Options(&self) -> windows_core::Result<i32>;
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TargetIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingPortMappingProps {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPortMappingProps_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>() -> INetSharingPortMappingProps_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPProtocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucipprot: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::IPProtocol(this) {
                Ok(ok__) => {
                    core::ptr::write(pucipprot, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusport: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::ExternalPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pusport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusport: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::InternalPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pusport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoptions: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::Options(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwoptions, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::TargetName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtargetname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetIPAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetipaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::TargetIPAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtargetipaddress, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPortMappingProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPortMappingProps_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pbool, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            IPProtocol: IPProtocol::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            TargetName: TargetName::<Identity, Impl, OFFSET>,
            TargetIPAddress: TargetIPAddress::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPortMappingProps as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPrivateConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingPrivateConnectionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPrivateConnectionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>() -> INetSharingPrivateConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPrivateConnectionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPrivateConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPrivateConnectionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPrivateConnectionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait INetSharingPublicConnectionCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Count(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for INetSharingPublicConnectionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl INetSharingPublicConnectionCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>() -> INetSharingPublicConnectionCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPublicConnectionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: INetSharingPublicConnectionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match INetSharingPublicConnectionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetSharingPublicConnectionCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStaticPortMapping_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ExternalIPAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ExternalPort(&self) -> windows_core::Result<i32>;
    fn InternalPort(&self) -> windows_core::Result<i32>;
    fn Protocol(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InternalClient(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EditInternalClient(&self, bstrinternalclient: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enable(&self, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EditDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EditInternalPort(&self, linternalport: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStaticPortMapping {}
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMapping_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>() -> IStaticPortMapping_Vtbl {
        unsafe extern "system" fn ExternalIPAddress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::ExternalIPAddress(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::ExternalPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::InternalPort(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::Protocol(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::InternalClient(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMapping_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStaticPortMapping_Impl::EditInternalClient(this, core::mem::transmute(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStaticPortMapping_Impl::Enable(this, core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStaticPortMapping_Impl::EditDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linternalport: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStaticPortMapping_Impl::EditInternalPort(this, core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, Impl, OFFSET>,
            ExternalPort: ExternalPort::<Identity, Impl, OFFSET>,
            InternalPort: InternalPort::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            InternalClient: InternalClient::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            EditDescription: EditDescription::<Identity, Impl, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStaticPortMapping as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStaticPortMappingCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<IStaticPortMapping>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Remove(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Add(&self, lexternalport: i32, bstrprotocol: &windows_core::BSTR, linternalport: i32, bstrinternalclient: &windows_core::BSTR, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: &windows_core::BSTR) -> windows_core::Result<IStaticPortMapping>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStaticPortMappingCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IStaticPortMappingCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>() -> IStaticPortMappingCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMappingCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, ppspm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMappingCollection_Impl::get_Item(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)) {
                Ok(ok__) => {
                    core::ptr::write(ppspm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMappingCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pval, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IStaticPortMappingCollection_Impl::Remove(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IStaticPortMappingCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, linternalport: i32, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, ppspm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IStaticPortMappingCollection_Impl::Add(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&linternalport), core::mem::transmute(&bstrinternalclient), core::mem::transmute_copy(&benabled), core::mem::transmute(&bstrdescription)) {
                Ok(ok__) => {
                    core::ptr::write(ppspm, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStaticPortMappingCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUPnPNAT_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StaticPortMappingCollection(&self) -> windows_core::Result<IStaticPortMappingCollection>;
    fn DynamicPortMappingCollection(&self) -> windows_core::Result<IDynamicPortMappingCollection>;
    fn NATEventManager(&self) -> windows_core::Result<INATEventManager>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IUPnPNAT {}
#[cfg(feature = "Win32_System_Com")]
impl IUPnPNAT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: isize>() -> IUPnPNAT_Vtbl {
        unsafe extern "system" fn StaticPortMappingCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspms: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUPnPNAT_Impl::StaticPortMappingCollection(this) {
                Ok(ok__) => {
                    core::ptr::write(ppspms, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdpms: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUPnPNAT_Impl::DynamicPortMappingCollection(this) {
                Ok(ok__) => {
                    core::ptr::write(ppdpms, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NATEventManager<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IUPnPNAT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IUPnPNAT_Impl::NATEventManager(this) {
                Ok(ok__) => {
                    core::ptr::write(ppnem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StaticPortMappingCollection: StaticPortMappingCollection::<Identity, Impl, OFFSET>,
            DynamicPortMappingCollection: DynamicPortMappingCollection::<Identity, Impl, OFFSET>,
            NATEventManager: NATEventManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPNAT as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
