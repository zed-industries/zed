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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDynamicPortMapping_Vtbl
    where
        Identity: IDynamicPortMapping_Impl,
    {
        unsafe extern "system" fn ExternalIPAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::ExternalIPAddress(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::RemoteHost(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::ExternalPort(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::Protocol(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::InternalPort(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::InternalClient(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::Enabled(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::Description(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseDuration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::LeaseDuration(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewLease<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lleasedurationdesired: i32, pleasedurationreturned: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMapping_Impl::RenewLease(this, core::mem::transmute_copy(&lleasedurationdesired)) {
                Ok(ok__) => {
                    pleasedurationreturned.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicPortMapping_Impl::EditInternalClient(this, core::mem::transmute(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicPortMapping_Impl::Enable(this, core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicPortMapping_Impl::EditDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linternalport: i32) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicPortMapping_Impl::EditInternalPort(this, core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, OFFSET>,
            RemoteHost: RemoteHost::<Identity, OFFSET>,
            ExternalPort: ExternalPort::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            InternalPort: InternalPort::<Identity, OFFSET>,
            InternalClient: InternalClient::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            LeaseDuration: LeaseDuration::<Identity, OFFSET>,
            RenewLease: RenewLease::<Identity, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            EditDescription: EditDescription::<Identity, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDynamicPortMappingCollection_Vtbl
    where
        Identity: IDynamicPortMappingCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMappingCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: core::mem::MaybeUninit<windows_core::BSTR>, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, ppdpm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMappingCollection_Impl::get_Item(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)) {
                Ok(ok__) => {
                    ppdpm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMappingCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: core::mem::MaybeUninit<windows_core::BSTR>, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDynamicPortMappingCollection_Impl::Remove(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrremotehost: core::mem::MaybeUninit<windows_core::BSTR>, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, linternalport: i32, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, lleaseduration: i32, ppdpm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDynamicPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDynamicPortMappingCollection_Impl::Add(this, core::mem::transmute(&bstrremotehost), core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&linternalport), core::mem::transmute(&bstrinternalclient), core::mem::transmute_copy(&benabled), core::mem::transmute(&bstrdescription), core::mem::transmute_copy(&lleaseduration)) {
                Ok(ok__) => {
                    ppdpm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetConnection_Vtbl
    where
        Identity: IEnumNetConnection_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumNetConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetSharingEveryConnection_Vtbl
    where
        Identity: IEnumNetSharingEveryConnection_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingEveryConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingEveryConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingEveryConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingEveryConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingEveryConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingEveryConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingEveryConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumNetSharingEveryConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetSharingPortMapping_Vtbl
    where
        Identity: IEnumNetSharingPortMapping_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPortMapping_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPortMapping_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPortMapping_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumNetSharingPortMapping_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetSharingPrivateConnection_Vtbl
    where
        Identity: IEnumNetSharingPrivateConnection_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPrivateConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPrivateConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPrivateConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPrivateConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPrivateConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPrivateConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPrivateConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumNetSharingPrivateConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumNetSharingPublicConnection_Vtbl
    where
        Identity: IEnumNetSharingPublicConnection_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPublicConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPublicConnection_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgvar), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPublicConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPublicConnection_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPublicConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumNetSharingPublicConnection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumNetSharingPublicConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumNetSharingPublicConnection_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INATEventManager_Vtbl
    where
        Identity: INATEventManager_Impl,
    {
        unsafe extern "system" fn SetExternalIPAddressCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INATEventManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INATEventManager_Impl::SetExternalIPAddressCallback(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn SetNumberOfEntriesCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INATEventManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INATEventManager_Impl::SetNumberOfEntriesCallback(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetExternalIPAddressCallback: SetExternalIPAddressCallback::<Identity, OFFSET>,
            SetNumberOfEntriesCallback: SetNumberOfEntriesCallback::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INATExternalIPAddressCallback_Vtbl
    where
        Identity: INATExternalIPAddressCallback_Impl,
    {
        unsafe extern "system" fn NewExternalIPAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnewexternalipaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INATExternalIPAddressCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INATExternalIPAddressCallback_Impl::NewExternalIPAddress(this, core::mem::transmute(&bstrnewexternalipaddress)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewExternalIPAddress: NewExternalIPAddress::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INATNumberOfEntriesCallback_Vtbl
    where
        Identity: INATNumberOfEntriesCallback_Impl,
    {
        unsafe extern "system" fn NewNumberOfEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnewnumberofentries: i32) -> windows_core::HRESULT
        where
            Identity: INATNumberOfEntriesCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INATNumberOfEntriesCallback_Impl::NewNumberOfEntries(this, core::mem::transmute_copy(&lnewnumberofentries)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NewNumberOfEntries: NewNumberOfEntries::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetConnection_Vtbl
    where
        Identity: INetConnection_Impl,
    {
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnection_Impl::Connect(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnection_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnection_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Duplicate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwduplicatename: windows_core::PCWSTR, ppcon: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnection_Impl::Duplicate(this, core::mem::transmute(&pszwduplicatename)) {
                Ok(ok__) => {
                    ppcon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprops: *mut *mut NETCON_PROPERTIES) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnection_Impl::GetProperties(this) {
                Ok(ok__) => {
                    ppprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiObjectClassId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnection_Impl::GetUiObjectClassId(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszwnewname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: INetConnection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnection_Impl::Rename(this, core::mem::transmute(&pszwnewname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Duplicate: Duplicate::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetUiObjectClassId: GetUiObjectClassId::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetConnectionConnectUi_Vtbl
    where
        Identity: INetConnectionConnectUi_Impl,
    {
        unsafe extern "system" fn SetConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcon: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetConnectionConnectUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnectionConnectUi_Impl::SetConnection(this, windows_core::from_raw_borrowed(&pcon)).into()
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: INetConnectionConnectUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnectionConnectUi_Impl::Connect(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: INetConnectionConnectUi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetConnectionConnectUi_Impl::Disconnect(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetConnection: SetConnection::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetConnectionManager_Vtbl
    where
        Identity: INetConnectionManager_Impl,
    {
        unsafe extern "system" fn EnumConnections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: NETCONMGR_ENUM_FLAGS, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetConnectionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionManager_Impl::EnumConnections(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumConnections: EnumConnections::<Identity, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetConnectionProps_Vtbl
    where
        Identity: INetConnectionProps_Impl,
    {
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetConnectionProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionProps_Impl::Guid(this) {
                Ok(ok__) => {
                    pbstrguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetConnectionProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionProps_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetConnectionProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionProps_Impl::DeviceName(this) {
                Ok(ok__) => {
                    pbstrdevicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut NETCON_STATUS) -> windows_core::HRESULT
        where
            Identity: INetConnectionProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionProps_Impl::Status(this) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut NETCON_MEDIATYPE) -> windows_core::HRESULT
        where
            Identity: INetConnectionProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionProps_Impl::MediaType(this) {
                Ok(ok__) => {
                    pmediatype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: INetConnectionProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetConnectionProps_Impl::Characteristics(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Guid: Guid::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            MediaType: MediaType::<Identity, OFFSET>,
            Characteristics: Characteristics::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwAuthorizedApplication_Vtbl
    where
        Identity: INetFwAuthorizedApplication_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplication_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplication_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn ProcessImageFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplication_Impl::ProcessImageFileName(this) {
                Ok(ok__) => {
                    imagefilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessImageFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplication_Impl::SetProcessImageFileName(this, core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplication_Impl::IpVersion(this) {
                Ok(ok__) => {
                    ipversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplication_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplication_Impl::Scope(this) {
                Ok(ok__) => {
                    scope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplication_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplication_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    remoteaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplication_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplication_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplication_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            ProcessImageFileName: ProcessImageFileName::<Identity, OFFSET>,
            SetProcessImageFileName: SetProcessImageFileName::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwAuthorizedApplications_Vtbl
    where
        Identity: INetFwAuthorizedApplications_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplications_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, app: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplications_Impl::Add(this, windows_core::from_raw_borrowed(&app)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwAuthorizedApplications_Impl::Remove(this, core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>, app: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplications_Impl::Item(this, core::mem::transmute(&imagefilename)) {
                Ok(ok__) => {
                    app.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwAuthorizedApplications_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwAuthorizedApplications_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwIcmpSettings_Vtbl
    where
        Identity: INetFwIcmpSettings_Impl,
    {
        unsafe extern "system" fn AllowOutboundDestinationUnreachable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowOutboundDestinationUnreachable(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundDestinationUnreachable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowOutboundDestinationUnreachable(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowRedirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowRedirect(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowRedirect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowRedirect(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundEchoRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowInboundEchoRequest(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundEchoRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowInboundEchoRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundTimeExceeded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowOutboundTimeExceeded(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundTimeExceeded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowOutboundTimeExceeded(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundParameterProblem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowOutboundParameterProblem(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundParameterProblem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowOutboundParameterProblem(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundSourceQuench<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowOutboundSourceQuench(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundSourceQuench<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowOutboundSourceQuench(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundRouterRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowInboundRouterRequest(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundRouterRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowInboundRouterRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundTimestampRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowInboundTimestampRequest(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundTimestampRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowInboundTimestampRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowInboundMaskRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowInboundMaskRequest(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInboundMaskRequest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowInboundMaskRequest(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowOutboundPacketTooBig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwIcmpSettings_Impl::AllowOutboundPacketTooBig(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutboundPacketTooBig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwIcmpSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwIcmpSettings_Impl::SetAllowOutboundPacketTooBig(this, core::mem::transmute_copy(&allow)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AllowOutboundDestinationUnreachable: AllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            SetAllowOutboundDestinationUnreachable: SetAllowOutboundDestinationUnreachable::<Identity, OFFSET>,
            AllowRedirect: AllowRedirect::<Identity, OFFSET>,
            SetAllowRedirect: SetAllowRedirect::<Identity, OFFSET>,
            AllowInboundEchoRequest: AllowInboundEchoRequest::<Identity, OFFSET>,
            SetAllowInboundEchoRequest: SetAllowInboundEchoRequest::<Identity, OFFSET>,
            AllowOutboundTimeExceeded: AllowOutboundTimeExceeded::<Identity, OFFSET>,
            SetAllowOutboundTimeExceeded: SetAllowOutboundTimeExceeded::<Identity, OFFSET>,
            AllowOutboundParameterProblem: AllowOutboundParameterProblem::<Identity, OFFSET>,
            SetAllowOutboundParameterProblem: SetAllowOutboundParameterProblem::<Identity, OFFSET>,
            AllowOutboundSourceQuench: AllowOutboundSourceQuench::<Identity, OFFSET>,
            SetAllowOutboundSourceQuench: SetAllowOutboundSourceQuench::<Identity, OFFSET>,
            AllowInboundRouterRequest: AllowInboundRouterRequest::<Identity, OFFSET>,
            SetAllowInboundRouterRequest: SetAllowInboundRouterRequest::<Identity, OFFSET>,
            AllowInboundTimestampRequest: AllowInboundTimestampRequest::<Identity, OFFSET>,
            SetAllowInboundTimestampRequest: SetAllowInboundTimestampRequest::<Identity, OFFSET>,
            AllowInboundMaskRequest: AllowInboundMaskRequest::<Identity, OFFSET>,
            SetAllowInboundMaskRequest: SetAllowInboundMaskRequest::<Identity, OFFSET>,
            AllowOutboundPacketTooBig: AllowOutboundPacketTooBig::<Identity, OFFSET>,
            SetAllowOutboundPacketTooBig: SetAllowOutboundPacketTooBig::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwMgr_Vtbl
    where
        Identity: INetFwMgr_Impl,
    {
        unsafe extern "system" fn LocalPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwMgr_Impl::LocalPolicy(this) {
                Ok(ok__) => {
                    localpolicy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfileType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT
        where
            Identity: INetFwMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwMgr_Impl::CurrentProfileType(this) {
                Ok(ok__) => {
                    profiletype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDefaults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwMgr_Impl::RestoreDefaults(this).into()
        }
        unsafe extern "system" fn IsPortAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>, ipversion: NET_FW_IP_VERSION, portnumber: i32, localaddress: core::mem::MaybeUninit<windows_core::BSTR>, ipprotocol: NET_FW_IP_PROTOCOL, allowed: *mut core::mem::MaybeUninit<windows_core::VARIANT>, restricted: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwMgr_Impl::IsPortAllowed(this, core::mem::transmute(&imagefilename), core::mem::transmute_copy(&ipversion), core::mem::transmute_copy(&portnumber), core::mem::transmute(&localaddress), core::mem::transmute_copy(&ipprotocol), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
        }
        unsafe extern "system" fn IsIcmpTypeAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION, localaddress: core::mem::MaybeUninit<windows_core::BSTR>, r#type: u8, allowed: *mut core::mem::MaybeUninit<windows_core::VARIANT>, restricted: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwMgr_Impl::IsIcmpTypeAllowed(this, core::mem::transmute_copy(&ipversion), core::mem::transmute(&localaddress), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&allowed), core::mem::transmute_copy(&restricted)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LocalPolicy: LocalPolicy::<Identity, OFFSET>,
            CurrentProfileType: CurrentProfileType::<Identity, OFFSET>,
            RestoreDefaults: RestoreDefaults::<Identity, OFFSET>,
            IsPortAllowed: IsPortAllowed::<Identity, OFFSET>,
            IsIcmpTypeAllowed: IsIcmpTypeAllowed::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwOpenPort_Vtbl
    where
        Identity: INetFwOpenPort_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::IpVersion(this) {
                Ok(ok__) => {
                    ipversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: *mut NET_FW_IP_PROTOCOL) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::Protocol(this) {
                Ok(ok__) => {
                    ipprotocol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetProtocol(this, core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Port<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::Port(this) {
                Ok(ok__) => {
                    portnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetPort(this, core::mem::transmute_copy(&portnumber)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::Scope(this) {
                Ok(ok__) => {
                    scope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    remoteaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPort_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn BuiltIn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, builtin: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPort_Impl::BuiltIn(this) {
                Ok(ok__) => {
                    builtin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            SetProtocol: SetProtocol::<Identity, OFFSET>,
            Port: Port::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            BuiltIn: BuiltIn::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwOpenPorts_Vtbl
    where
        Identity: INetFwOpenPorts_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPorts_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, port: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPorts_Impl::Add(this, windows_core::from_raw_borrowed(&port)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwOpenPorts_Impl::Remove(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumber: i32, ipprotocol: NET_FW_IP_PROTOCOL, openport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPorts_Impl::Item(this, core::mem::transmute_copy(&portnumber), core::mem::transmute_copy(&ipprotocol)) {
                Ok(ok__) => {
                    openport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwOpenPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwOpenPorts_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwPolicy_Vtbl
    where
        Identity: INetFwPolicy_Impl,
    {
        unsafe extern "system" fn CurrentProfile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy_Impl::CurrentProfile(this) {
                Ok(ok__) => {
                    profile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfileByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy_Impl::GetProfileByType(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    profile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentProfile: CurrentProfile::<Identity, OFFSET>,
            GetProfileByType: GetProfileByType::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwPolicy2_Vtbl
    where
        Identity: INetFwPolicy2_Impl,
    {
        unsafe extern "system" fn CurrentProfileTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::CurrentProfileTypes(this) {
                Ok(ok__) => {
                    profiletypesbitmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_FirewallEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_FirewallEnabled(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_FirewallEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_FirewallEnabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn get_ExcludedInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    interfaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ExcludedInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, interfaces: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_ExcludedInterfaces(this, core::mem::transmute_copy(&profiletype), core::mem::transmute(&interfaces)).into()
        }
        unsafe extern "system" fn get_BlockAllInboundTraffic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    block.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_BlockAllInboundTraffic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, block: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_BlockAllInboundTraffic(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&block)).into()
        }
        unsafe extern "system" fn get_NotificationsDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    disabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_NotificationsDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_NotificationsDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn get_UnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    disabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_UnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_UnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn Rules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::Rules(this) {
                Ok(ok__) => {
                    rules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceRestriction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicerestriction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::ServiceRestriction(this) {
                Ok(ok__) => {
                    servicerestriction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableRuleGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: core::mem::MaybeUninit<windows_core::BSTR>, enable: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::EnableRuleGroup(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group), core::mem::transmute_copy(&enable)).into()
        }
        unsafe extern "system" fn IsRuleGroupEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32, group: core::mem::MaybeUninit<windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::IsRuleGroupEnabled(this, core::mem::transmute_copy(&profiletypesbitmask), core::mem::transmute(&group)) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreLocalFirewallDefaults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::RestoreLocalFirewallDefaults(this).into()
        }
        unsafe extern "system" fn get_DefaultInboundAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    action.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_DefaultInboundAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_DefaultInboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn get_DefaultOutboundAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: *mut NET_FW_ACTION) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype)) {
                Ok(ok__) => {
                    action.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_DefaultOutboundAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: NET_FW_PROFILE_TYPE2, action: NET_FW_ACTION) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwPolicy2_Impl::put_DefaultOutboundAction(this, core::mem::transmute_copy(&profiletype), core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn get_IsRuleGroupCurrentlyEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: core::mem::MaybeUninit<windows_core::BSTR>, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::get_IsRuleGroupCurrentlyEnabled(this, core::mem::transmute(&group)) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPolicyModifyState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modifystate: *mut NET_FW_MODIFY_STATE) -> windows_core::HRESULT
        where
            Identity: INetFwPolicy2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwPolicy2_Impl::LocalPolicyModifyState(this) {
                Ok(ok__) => {
                    modifystate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentProfileTypes: CurrentProfileTypes::<Identity, OFFSET>,
            get_FirewallEnabled: get_FirewallEnabled::<Identity, OFFSET>,
            put_FirewallEnabled: put_FirewallEnabled::<Identity, OFFSET>,
            get_ExcludedInterfaces: get_ExcludedInterfaces::<Identity, OFFSET>,
            put_ExcludedInterfaces: put_ExcludedInterfaces::<Identity, OFFSET>,
            get_BlockAllInboundTraffic: get_BlockAllInboundTraffic::<Identity, OFFSET>,
            put_BlockAllInboundTraffic: put_BlockAllInboundTraffic::<Identity, OFFSET>,
            get_NotificationsDisabled: get_NotificationsDisabled::<Identity, OFFSET>,
            put_NotificationsDisabled: put_NotificationsDisabled::<Identity, OFFSET>,
            get_UnicastResponsesToMulticastBroadcastDisabled: get_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            put_UnicastResponsesToMulticastBroadcastDisabled: put_UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            ServiceRestriction: ServiceRestriction::<Identity, OFFSET>,
            EnableRuleGroup: EnableRuleGroup::<Identity, OFFSET>,
            IsRuleGroupEnabled: IsRuleGroupEnabled::<Identity, OFFSET>,
            RestoreLocalFirewallDefaults: RestoreLocalFirewallDefaults::<Identity, OFFSET>,
            get_DefaultInboundAction: get_DefaultInboundAction::<Identity, OFFSET>,
            put_DefaultInboundAction: put_DefaultInboundAction::<Identity, OFFSET>,
            get_DefaultOutboundAction: get_DefaultOutboundAction::<Identity, OFFSET>,
            put_DefaultOutboundAction: put_DefaultOutboundAction::<Identity, OFFSET>,
            get_IsRuleGroupCurrentlyEnabled: get_IsRuleGroupCurrentlyEnabled::<Identity, OFFSET>,
            LocalPolicyModifyState: LocalPolicyModifyState::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwProduct_Vtbl
    where
        Identity: INetFwProduct_Impl,
    {
        unsafe extern "system" fn RuleCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwProduct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProduct_Impl::RuleCategories(this) {
                Ok(ok__) => {
                    rulecategories.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleCategories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulecategories: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwProduct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwProduct_Impl::SetRuleCategories(this, core::mem::transmute(&rulecategories)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwProduct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProduct_Impl::DisplayName(this) {
                Ok(ok__) => {
                    displayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwProduct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwProduct_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
        }
        unsafe extern "system" fn PathToSignedProductExe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwProduct_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProduct_Impl::PathToSignedProductExe(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RuleCategories: RuleCategories::<Identity, OFFSET>,
            SetRuleCategories: SetRuleCategories::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            PathToSignedProductExe: PathToSignedProductExe::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwProducts_Vtbl
    where
        Identity: INetFwProducts_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwProducts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProducts_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, product: *mut core::ffi::c_void, registration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProducts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProducts_Impl::Register(this, windows_core::from_raw_borrowed(&product)) {
                Ok(ok__) => {
                    registration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, product: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProducts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProducts_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    product.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProducts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProducts_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwProfile_Vtbl
    where
        Identity: INetFwProfile_Impl,
    {
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut NET_FW_PROFILE_TYPE) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::Type(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirewallEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::FirewallEnabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFirewallEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwProfile_Impl::SetFirewallEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn ExceptionsNotAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::ExceptionsNotAllowed(this) {
                Ok(ok__) => {
                    notallowed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExceptionsNotAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, notallowed: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwProfile_Impl::SetExceptionsNotAllowed(this, core::mem::transmute_copy(&notallowed)).into()
        }
        unsafe extern "system" fn NotificationsDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::NotificationsDisabled(this) {
                Ok(ok__) => {
                    disabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationsDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwProfile_Impl::SetNotificationsDisabled(this, core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn UnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::UnicastResponsesToMulticastBroadcastDisabled(this) {
                Ok(ok__) => {
                    disabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicastResponsesToMulticastBroadcastDisabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwProfile_Impl::SetUnicastResponsesToMulticastBroadcastDisabled(this, core::mem::transmute_copy(&disabled)).into()
        }
        unsafe extern "system" fn RemoteAdminSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteadminsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::RemoteAdminSettings(this) {
                Ok(ok__) => {
                    remoteadminsettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IcmpSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmpsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::IcmpSettings(this) {
                Ok(ok__) => {
                    icmpsettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::GloballyOpenPorts(this) {
                Ok(ok__) => {
                    openports.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, services: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::Services(this) {
                Ok(ok__) => {
                    services.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthorizedApplications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, apps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwProfile_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwProfile_Impl::AuthorizedApplications(this) {
                Ok(ok__) => {
                    apps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            FirewallEnabled: FirewallEnabled::<Identity, OFFSET>,
            SetFirewallEnabled: SetFirewallEnabled::<Identity, OFFSET>,
            ExceptionsNotAllowed: ExceptionsNotAllowed::<Identity, OFFSET>,
            SetExceptionsNotAllowed: SetExceptionsNotAllowed::<Identity, OFFSET>,
            NotificationsDisabled: NotificationsDisabled::<Identity, OFFSET>,
            SetNotificationsDisabled: SetNotificationsDisabled::<Identity, OFFSET>,
            UnicastResponsesToMulticastBroadcastDisabled: UnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            SetUnicastResponsesToMulticastBroadcastDisabled: SetUnicastResponsesToMulticastBroadcastDisabled::<Identity, OFFSET>,
            RemoteAdminSettings: RemoteAdminSettings::<Identity, OFFSET>,
            IcmpSettings: IcmpSettings::<Identity, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, OFFSET>,
            Services: Services::<Identity, OFFSET>,
            AuthorizedApplications: AuthorizedApplications::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwRemoteAdminSettings_Vtbl
    where
        Identity: INetFwRemoteAdminSettings_Impl,
    {
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRemoteAdminSettings_Impl::IpVersion(this) {
                Ok(ok__) => {
                    ipversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRemoteAdminSettings_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRemoteAdminSettings_Impl::Scope(this) {
                Ok(ok__) => {
                    scope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRemoteAdminSettings_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRemoteAdminSettings_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    remoteaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRemoteAdminSettings_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRemoteAdminSettings_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwRemoteAdminSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRemoteAdminSettings_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwRule_Vtbl
    where
        Identity: INetFwRule_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetName(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Description(this) {
                Ok(ok__) => {
                    desc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetDescription(this, core::mem::transmute(&desc)).into()
        }
        unsafe extern "system" fn ApplicationName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::ApplicationName(this) {
                Ok(ok__) => {
                    imagefilename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetApplicationName(this, core::mem::transmute(&imagefilename)).into()
        }
        unsafe extern "system" fn ServiceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::ServiceName(this) {
                Ok(ok__) => {
                    servicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetServiceName(this, core::mem::transmute(&servicename)).into()
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Protocol(this) {
                Ok(ok__) => {
                    protocol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, protocol: i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetProtocol(this, core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn LocalPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::LocalPorts(this) {
                Ok(ok__) => {
                    portnumbers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetLocalPorts(this, core::mem::transmute(&portnumbers)).into()
        }
        unsafe extern "system" fn RemotePorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::RemotePorts(this) {
                Ok(ok__) => {
                    portnumbers.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemotePorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portnumbers: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetRemotePorts(this, core::mem::transmute(&portnumbers)).into()
        }
        unsafe extern "system" fn LocalAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::LocalAddresses(this) {
                Ok(ok__) => {
                    localaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetLocalAddresses(this, core::mem::transmute(&localaddrs)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    remoteaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn IcmpTypesAndCodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::IcmpTypesAndCodes(this) {
                Ok(ok__) => {
                    icmptypesandcodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcmpTypesAndCodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, icmptypesandcodes: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetIcmpTypesAndCodes(this, core::mem::transmute(&icmptypesandcodes)).into()
        }
        unsafe extern "system" fn Direction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: *mut NET_FW_RULE_DIRECTION) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Direction(this) {
                Ok(ok__) => {
                    dir.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dir: NET_FW_RULE_DIRECTION) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetDirection(this, core::mem::transmute_copy(&dir)).into()
        }
        unsafe extern "system" fn Interfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Interfaces(this) {
                Ok(ok__) => {
                    interfaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfaces: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetInterfaces(this, core::mem::transmute(&interfaces)).into()
        }
        unsafe extern "system" fn InterfaceTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::InterfaceTypes(this) {
                Ok(ok__) => {
                    interfacetypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacetypes: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetInterfaceTypes(this, core::mem::transmute(&interfacetypes)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Grouping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Grouping(this) {
                Ok(ok__) => {
                    context.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrouping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetGrouping(this, core::mem::transmute(&context)).into()
        }
        unsafe extern "system" fn Profiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Profiles(this) {
                Ok(ok__) => {
                    profiletypesbitmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfiles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletypesbitmask: i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetProfiles(this, core::mem::transmute_copy(&profiletypesbitmask)).into()
        }
        unsafe extern "system" fn EdgeTraversal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::EdgeTraversal(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetEdgeTraversal(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Action<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut NET_FW_ACTION) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule_Impl::Action(this) {
                Ok(ok__) => {
                    action.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: NET_FW_ACTION) -> windows_core::HRESULT
        where
            Identity: INetFwRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule_Impl::SetAction(this, core::mem::transmute_copy(&action)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ApplicationName: ApplicationName::<Identity, OFFSET>,
            SetApplicationName: SetApplicationName::<Identity, OFFSET>,
            ServiceName: ServiceName::<Identity, OFFSET>,
            SetServiceName: SetServiceName::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            SetProtocol: SetProtocol::<Identity, OFFSET>,
            LocalPorts: LocalPorts::<Identity, OFFSET>,
            SetLocalPorts: SetLocalPorts::<Identity, OFFSET>,
            RemotePorts: RemotePorts::<Identity, OFFSET>,
            SetRemotePorts: SetRemotePorts::<Identity, OFFSET>,
            LocalAddresses: LocalAddresses::<Identity, OFFSET>,
            SetLocalAddresses: SetLocalAddresses::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            IcmpTypesAndCodes: IcmpTypesAndCodes::<Identity, OFFSET>,
            SetIcmpTypesAndCodes: SetIcmpTypesAndCodes::<Identity, OFFSET>,
            Direction: Direction::<Identity, OFFSET>,
            SetDirection: SetDirection::<Identity, OFFSET>,
            Interfaces: Interfaces::<Identity, OFFSET>,
            SetInterfaces: SetInterfaces::<Identity, OFFSET>,
            InterfaceTypes: InterfaceTypes::<Identity, OFFSET>,
            SetInterfaceTypes: SetInterfaceTypes::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Grouping: Grouping::<Identity, OFFSET>,
            SetGrouping: SetGrouping::<Identity, OFFSET>,
            Profiles: Profiles::<Identity, OFFSET>,
            SetProfiles: SetProfiles::<Identity, OFFSET>,
            EdgeTraversal: EdgeTraversal::<Identity, OFFSET>,
            SetEdgeTraversal: SetEdgeTraversal::<Identity, OFFSET>,
            Action: Action::<Identity, OFFSET>,
            SetAction: SetAction::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwRule2_Vtbl
    where
        Identity: INetFwRule2_Impl,
    {
        unsafe extern "system" fn EdgeTraversalOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule2_Impl::EdgeTraversalOptions(this) {
                Ok(ok__) => {
                    loptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEdgeTraversalOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule2_Impl::SetEdgeTraversalOptions(this, core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base__: INetFwRule_Vtbl::new::<Identity, OFFSET>(),
            EdgeTraversalOptions: EdgeTraversalOptions::<Identity, OFFSET>,
            SetEdgeTraversalOptions: SetEdgeTraversalOptions::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwRule3_Vtbl
    where
        Identity: INetFwRule3_Impl,
    {
        unsafe extern "system" fn LocalAppPackageId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule3_Impl::LocalAppPackageId(this) {
                Ok(ok__) => {
                    wszpackageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAppPackageId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpackageid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule3_Impl::SetLocalAppPackageId(this, core::mem::transmute(&wszpackageid)).into()
        }
        unsafe extern "system" fn LocalUserOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule3_Impl::LocalUserOwner(this) {
                Ok(ok__) => {
                    wszuserowner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserowner: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule3_Impl::SetLocalUserOwner(this, core::mem::transmute(&wszuserowner)).into()
        }
        unsafe extern "system" fn LocalUserAuthorizedList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule3_Impl::LocalUserAuthorizedList(this) {
                Ok(ok__) => {
                    wszuserauthlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalUserAuthorizedList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule3_Impl::SetLocalUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteUserAuthorizedList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule3_Impl::RemoteUserAuthorizedList(this) {
                Ok(ok__) => {
                    wszuserauthlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteUserAuthorizedList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule3_Impl::SetRemoteUserAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn RemoteMachineAuthorizedList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule3_Impl::RemoteMachineAuthorizedList(this) {
                Ok(ok__) => {
                    wszuserauthlist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteMachineAuthorizedList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuserauthlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule3_Impl::SetRemoteMachineAuthorizedList(this, core::mem::transmute(&wszuserauthlist)).into()
        }
        unsafe extern "system" fn SecureFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRule3_Impl::SecureFlags(this) {
                Ok(ok__) => {
                    loptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecureFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, loptions: i32) -> windows_core::HRESULT
        where
            Identity: INetFwRule3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRule3_Impl::SetSecureFlags(this, core::mem::transmute_copy(&loptions)).into()
        }
        Self {
            base__: INetFwRule2_Vtbl::new::<Identity, OFFSET>(),
            LocalAppPackageId: LocalAppPackageId::<Identity, OFFSET>,
            SetLocalAppPackageId: SetLocalAppPackageId::<Identity, OFFSET>,
            LocalUserOwner: LocalUserOwner::<Identity, OFFSET>,
            SetLocalUserOwner: SetLocalUserOwner::<Identity, OFFSET>,
            LocalUserAuthorizedList: LocalUserAuthorizedList::<Identity, OFFSET>,
            SetLocalUserAuthorizedList: SetLocalUserAuthorizedList::<Identity, OFFSET>,
            RemoteUserAuthorizedList: RemoteUserAuthorizedList::<Identity, OFFSET>,
            SetRemoteUserAuthorizedList: SetRemoteUserAuthorizedList::<Identity, OFFSET>,
            RemoteMachineAuthorizedList: RemoteMachineAuthorizedList::<Identity, OFFSET>,
            SetRemoteMachineAuthorizedList: SetRemoteMachineAuthorizedList::<Identity, OFFSET>,
            SecureFlags: SecureFlags::<Identity, OFFSET>,
            SetSecureFlags: SetSecureFlags::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwRules_Vtbl
    where
        Identity: INetFwRules_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRules_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRules_Impl::Add(this, windows_core::from_raw_borrowed(&rule)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwRules_Impl::Remove(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRules_Impl::Item(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwRules_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwService_Vtbl
    where
        Identity: INetFwService_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut NET_FW_SERVICE_TYPE) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::Type(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Customized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, customized: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::Customized(this) {
                Ok(ok__) => {
                    customized.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: *mut NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::IpVersion(this) {
                Ok(ok__) => {
                    ipversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipversion: NET_FW_IP_VERSION) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwService_Impl::SetIpVersion(this, core::mem::transmute_copy(&ipversion)).into()
        }
        unsafe extern "system" fn Scope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::Scope(this) {
                Ok(ok__) => {
                    scope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: NET_FW_SCOPE) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwService_Impl::SetScope(this, core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn RemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::RemoteAddresses(this) {
                Ok(ok__) => {
                    remoteaddrs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaddrs: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwService_Impl::SetRemoteAddresses(this, core::mem::transmute(&remoteaddrs)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::Enabled(this) {
                Ok(ok__) => {
                    enabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwService_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GloballyOpenPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, openports: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwService_Impl::GloballyOpenPorts(this) {
                Ok(ok__) => {
                    openports.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Customized: Customized::<Identity, OFFSET>,
            IpVersion: IpVersion::<Identity, OFFSET>,
            SetIpVersion: SetIpVersion::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            RemoteAddresses: RemoteAddresses::<Identity, OFFSET>,
            SetRemoteAddresses: SetRemoteAddresses::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            GloballyOpenPorts: GloballyOpenPorts::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwServiceRestriction_Vtbl
    where
        Identity: INetFwServiceRestriction_Impl,
    {
        unsafe extern "system" fn RestrictService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>, appname: core::mem::MaybeUninit<windows_core::BSTR>, restrictservice: super::super::Foundation::VARIANT_BOOL, servicesidrestricted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwServiceRestriction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetFwServiceRestriction_Impl::RestrictService(this, core::mem::transmute(&servicename), core::mem::transmute(&appname), core::mem::transmute_copy(&restrictservice), core::mem::transmute_copy(&servicesidrestricted)).into()
        }
        unsafe extern "system" fn ServiceRestricted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: core::mem::MaybeUninit<windows_core::BSTR>, appname: core::mem::MaybeUninit<windows_core::BSTR>, servicerestricted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetFwServiceRestriction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwServiceRestriction_Impl::ServiceRestricted(this, core::mem::transmute(&servicename), core::mem::transmute(&appname)) {
                Ok(ok__) => {
                    servicerestricted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwServiceRestriction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwServiceRestriction_Impl::Rules(this) {
                Ok(ok__) => {
                    rules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RestrictService: RestrictService::<Identity, OFFSET>,
            ServiceRestricted: ServiceRestricted::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetFwServices_Vtbl
    where
        Identity: INetFwServices_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetFwServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwServices_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, svctype: NET_FW_SERVICE_TYPE, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwServices_Impl::Item(this, core::mem::transmute_copy(&svctype)) {
                Ok(ok__) => {
                    service.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetFwServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetFwServices_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingConfiguration_Vtbl
    where
        Identity: INetSharingConfiguration_Impl,
    {
        unsafe extern "system" fn SharingEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingConfiguration_Impl::SharingEnabled(this) {
                Ok(ok__) => {
                    pbenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingConnectionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut SHARINGCONNECTIONTYPE) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingConfiguration_Impl::SharingConnectionType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableSharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingConfiguration_Impl::DisableSharing(this).into()
        }
        unsafe extern "system" fn EnableSharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: SHARINGCONNECTIONTYPE) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingConfiguration_Impl::EnableSharing(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn InternetFirewallEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingConfiguration_Impl::InternetFirewallEnabled(this) {
                Ok(ok__) => {
                    pbenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableInternetFirewall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingConfiguration_Impl::DisableInternetFirewall(this).into()
        }
        unsafe extern "system" fn EnableInternetFirewall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingConfiguration_Impl::EnableInternetFirewall(this).into()
        }
        unsafe extern "system" fn get_EnumPortMappings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingConfiguration_Impl::get_EnumPortMappings(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppcoll.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ucipprotocol: u8, usexternalport: u16, usinternalport: u16, dwoptions: u32, bstrtargetnameoripaddress: core::mem::MaybeUninit<windows_core::BSTR>, etargettype: ICS_TARGETTYPE, ppmapping: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingConfiguration_Impl::AddPortMapping(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&ucipprotocol), core::mem::transmute_copy(&usexternalport), core::mem::transmute_copy(&usinternalport), core::mem::transmute_copy(&dwoptions), core::mem::transmute(&bstrtargetnameoripaddress), core::mem::transmute_copy(&etargettype)) {
                Ok(ok__) => {
                    ppmapping.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmapping: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingConfiguration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingConfiguration_Impl::RemovePortMapping(this, windows_core::from_raw_borrowed(&pmapping)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SharingEnabled: SharingEnabled::<Identity, OFFSET>,
            SharingConnectionType: SharingConnectionType::<Identity, OFFSET>,
            DisableSharing: DisableSharing::<Identity, OFFSET>,
            EnableSharing: EnableSharing::<Identity, OFFSET>,
            InternetFirewallEnabled: InternetFirewallEnabled::<Identity, OFFSET>,
            DisableInternetFirewall: DisableInternetFirewall::<Identity, OFFSET>,
            EnableInternetFirewall: EnableInternetFirewall::<Identity, OFFSET>,
            get_EnumPortMappings: get_EnumPortMappings::<Identity, OFFSET>,
            AddPortMapping: AddPortMapping::<Identity, OFFSET>,
            RemovePortMapping: RemovePortMapping::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingEveryConnectionCollection_Vtbl
    where
        Identity: INetSharingEveryConnectionCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingEveryConnectionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingEveryConnectionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingEveryConnectionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingEveryConnectionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingManager_Vtbl
    where
        Identity: INetSharingManager_Impl,
    {
        unsafe extern "system" fn SharingInstalled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinstalled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetSharingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingManager_Impl::SharingInstalled(this) {
                Ok(ok__) => {
                    pbinstalled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EnumPublicConnections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingManager_Impl::get_EnumPublicConnections(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppcoll.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EnumPrivateConnections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SHARINGCONNECTION_ENUM_FLAGS, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingManager_Impl::get_EnumPrivateConnections(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppcoll.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_INetSharingConfigurationForINetConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetconnection: *mut core::ffi::c_void, ppnetsharingconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingManager_Impl::get_INetSharingConfigurationForINetConnection(this, windows_core::from_raw_borrowed(&pnetconnection)) {
                Ok(ok__) => {
                    ppnetsharingconfiguration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumEveryConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcoll: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingManager_Impl::EnumEveryConnection(this) {
                Ok(ok__) => {
                    ppcoll.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NetConnectionProps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetconnection: *mut core::ffi::c_void, ppprops: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingManager_Impl::get_NetConnectionProps(this, windows_core::from_raw_borrowed(&pnetconnection)) {
                Ok(ok__) => {
                    ppprops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SharingInstalled: SharingInstalled::<Identity, OFFSET>,
            get_EnumPublicConnections: get_EnumPublicConnections::<Identity, OFFSET>,
            get_EnumPrivateConnections: get_EnumPrivateConnections::<Identity, OFFSET>,
            get_INetSharingConfigurationForINetConnection: get_INetSharingConfigurationForINetConnection::<Identity, OFFSET>,
            EnumEveryConnection: EnumEveryConnection::<Identity, OFFSET>,
            get_NetConnectionProps: get_NetConnectionProps::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingPortMapping_Vtbl
    where
        Identity: INetSharingPortMapping_Impl,
    {
        unsafe extern "system" fn Disable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingPortMapping_Impl::Disable(this).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingPortMapping_Impl::Enable(this).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnspmp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMapping_Impl::Properties(this) {
                Ok(ok__) => {
                    ppnspmp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INetSharingPortMapping_Impl::Delete(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Disable: Disable::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingPortMappingCollection_Vtbl
    where
        Identity: INetSharingPortMappingCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingPortMappingProps_Vtbl
    where
        Identity: INetSharingPortMappingProps_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucipprot: *mut u8) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::IPProtocol(this) {
                Ok(ok__) => {
                    pucipprot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusport: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::ExternalPort(this) {
                Ok(ok__) => {
                    pusport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusport: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::InternalPort(this) {
                Ok(ok__) => {
                    pusport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoptions: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::Options(this) {
                Ok(ok__) => {
                    pdwoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::TargetName(this) {
                Ok(ok__) => {
                    pbstrtargetname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetIPAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtargetipaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::TargetIPAddress(this) {
                Ok(ok__) => {
                    pbstrtargetipaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: INetSharingPortMappingProps_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPortMappingProps_Impl::Enabled(this) {
                Ok(ok__) => {
                    pbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            IPProtocol: IPProtocol::<Identity, OFFSET>,
            ExternalPort: ExternalPort::<Identity, OFFSET>,
            InternalPort: InternalPort::<Identity, OFFSET>,
            Options: Options::<Identity, OFFSET>,
            TargetName: TargetName::<Identity, OFFSET>,
            TargetIPAddress: TargetIPAddress::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingPrivateConnectionCollection_Vtbl
    where
        Identity: INetSharingPrivateConnectionCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPrivateConnectionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPrivateConnectionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingPrivateConnectionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPrivateConnectionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INetSharingPublicConnectionCollection_Vtbl
    where
        Identity: INetSharingPublicConnectionCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: INetSharingPublicConnectionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPublicConnectionCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: INetSharingPublicConnectionCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INetSharingPublicConnectionCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStaticPortMapping_Vtbl
    where
        Identity: IStaticPortMapping_Impl,
    {
        unsafe extern "system" fn ExternalIPAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::ExternalIPAddress(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::ExternalPort(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::InternalPort(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::Protocol(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternalClient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::InternalClient(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::Enabled(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMapping_Impl::Description(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EditInternalClient<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStaticPortMapping_Impl::EditInternalClient(this, core::mem::transmute(&bstrinternalclient)).into()
        }
        unsafe extern "system" fn Enable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vb: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStaticPortMapping_Impl::Enable(this, core::mem::transmute_copy(&vb)).into()
        }
        unsafe extern "system" fn EditDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStaticPortMapping_Impl::EditDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn EditInternalPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linternalport: i32) -> windows_core::HRESULT
        where
            Identity: IStaticPortMapping_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStaticPortMapping_Impl::EditInternalPort(this, core::mem::transmute_copy(&linternalport)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ExternalIPAddress: ExternalIPAddress::<Identity, OFFSET>,
            ExternalPort: ExternalPort::<Identity, OFFSET>,
            InternalPort: InternalPort::<Identity, OFFSET>,
            Protocol: Protocol::<Identity, OFFSET>,
            InternalClient: InternalClient::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            EditInternalClient: EditInternalClient::<Identity, OFFSET>,
            Enable: Enable::<Identity, OFFSET>,
            EditDescription: EditDescription::<Identity, OFFSET>,
            EditInternalPort: EditInternalPort::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStaticPortMappingCollection_Vtbl
    where
        Identity: IStaticPortMappingCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStaticPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMappingCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, ppspm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStaticPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMappingCollection_Impl::get_Item(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)) {
                Ok(ok__) => {
                    ppspm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT
        where
            Identity: IStaticPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMappingCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IStaticPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStaticPortMappingCollection_Impl::Remove(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol)).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexternalport: i32, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, linternalport: i32, bstrinternalclient: core::mem::MaybeUninit<windows_core::BSTR>, benabled: super::super::Foundation::VARIANT_BOOL, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>, ppspm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStaticPortMappingCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStaticPortMappingCollection_Impl::Add(this, core::mem::transmute_copy(&lexternalport), core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&linternalport), core::mem::transmute(&bstrinternalclient), core::mem::transmute_copy(&benabled), core::mem::transmute(&bstrdescription)) {
                Ok(ok__) => {
                    ppspm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUPnPNAT_Vtbl
    where
        Identity: IUPnPNAT_Impl,
    {
        unsafe extern "system" fn StaticPortMappingCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppspms: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPNAT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPNAT_Impl::StaticPortMappingCollection(this) {
                Ok(ok__) => {
                    ppspms.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicPortMappingCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdpms: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPNAT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPNAT_Impl::DynamicPortMappingCollection(this) {
                Ok(ok__) => {
                    ppdpms.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NATEventManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUPnPNAT_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUPnPNAT_Impl::NATEventManager(this) {
                Ok(ok__) => {
                    ppnem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StaticPortMappingCollection: StaticPortMappingCollection::<Identity, OFFSET>,
            DynamicPortMappingCollection: DynamicPortMappingCollection::<Identity, OFFSET>,
            NATEventManager: NATEventManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUPnPNAT as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
