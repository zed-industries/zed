#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplication_Impl: Sized + super::Com::IDispatch_Impl {
    fn MachineIdOfMachineName(&self, machinename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQApplication {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQApplication_Vtbl
    where
        Identity: IMSMQApplication_Impl,
    {
        unsafe extern "system" fn MachineIdOfMachineName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinename: core::mem::MaybeUninit<windows_core::BSTR>, pbstrguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication_Impl::MachineIdOfMachineName(this, core::mem::transmute(&machinename)) {
                Ok(ok__) => {
                    pbstrguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), MachineIdOfMachineName: MachineIdOfMachineName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQApplication as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplication2_Impl: Sized + IMSMQApplication_Impl {
    fn RegisterCertificate(&self, flags: *const windows_core::VARIANT, externalcertificate: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn MachineNameOfMachineId(&self, bstrguid: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn MSMQVersionMajor(&self) -> windows_core::Result<i16>;
    fn MSMQVersionMinor(&self) -> windows_core::Result<i16>;
    fn MSMQVersionBuild(&self) -> windows_core::Result<i16>;
    fn IsDsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQApplication2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQApplication2_Vtbl
    where
        Identity: IMSMQApplication2_Impl,
    {
        unsafe extern "system" fn RegisterCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *const core::mem::MaybeUninit<windows_core::VARIANT>, externalcertificate: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQApplication2_Impl::RegisterCertificate(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&externalcertificate)).into()
        }
        unsafe extern "system" fn MachineNameOfMachineId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguid: core::mem::MaybeUninit<windows_core::BSTR>, pbstrmachinename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication2_Impl::MachineNameOfMachineId(this, core::mem::transmute(&bstrguid)) {
                Ok(ok__) => {
                    pbstrmachinename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMajor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmsmqversionmajor: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication2_Impl::MSMQVersionMajor(this) {
                Ok(ok__) => {
                    psmsmqversionmajor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionMinor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmsmqversionminor: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication2_Impl::MSMQVersionMinor(this) {
                Ok(ok__) => {
                    psmsmqversionminor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MSMQVersionBuild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmsmqversionbuild: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication2_Impl::MSMQVersionBuild(this) {
                Ok(ok__) => {
                    psmsmqversionbuild.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDsEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisdsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication2_Impl::IsDsEnabled(this) {
                Ok(ok__) => {
                    pfisdsenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMSMQApplication_Vtbl::new::<Identity, OFFSET>(),
            RegisterCertificate: RegisterCertificate::<Identity, OFFSET>,
            MachineNameOfMachineId: MachineNameOfMachineId::<Identity, OFFSET>,
            MSMQVersionMajor: MSMQVersionMajor::<Identity, OFFSET>,
            MSMQVersionMinor: MSMQVersionMinor::<Identity, OFFSET>,
            MSMQVersionBuild: MSMQVersionBuild::<Identity, OFFSET>,
            IsDsEnabled: IsDsEnabled::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQApplication2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQApplication as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQApplication3_Impl: Sized + IMSMQApplication2_Impl {
    fn ActiveQueues(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn PrivateQueues(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DirectoryServiceServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BytesInAllQueues(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetMachine(&self, bstrmachine: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Machine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Tidy(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQApplication3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQApplication3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQApplication3_Vtbl
    where
        Identity: IMSMQApplication3_Impl,
    {
        unsafe extern "system" fn ActiveQueues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvactivequeues: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication3_Impl::ActiveQueues(this) {
                Ok(ok__) => {
                    pvactivequeues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateQueues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvprivatequeues: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication3_Impl::PrivateQueues(this) {
                Ok(ok__) => {
                    pvprivatequeues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryServiceServer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdirectoryserviceserver: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication3_Impl::DirectoryServiceServer(this) {
                Ok(ok__) => {
                    pbstrdirectoryserviceserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication3_Impl::IsConnected(this) {
                Ok(ok__) => {
                    pfisconnected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInAllQueues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytesinallqueues: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication3_Impl::BytesInAllQueues(this) {
                Ok(ok__) => {
                    pvbytesinallqueues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmachine: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQApplication3_Impl::SetMachine(this, core::mem::transmute(&bstrmachine)).into()
        }
        unsafe extern "system" fn Machine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQApplication3_Impl::Machine(this) {
                Ok(ok__) => {
                    pbstrmachine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQApplication3_Impl::Connect(this).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQApplication3_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn Tidy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQApplication3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQApplication3_Impl::Tidy(this).into()
        }
        Self {
            base__: IMSMQApplication2_Vtbl::new::<Identity, OFFSET>(),
            ActiveQueues: ActiveQueues::<Identity, OFFSET>,
            PrivateQueues: PrivateQueues::<Identity, OFFSET>,
            DirectoryServiceServer: DirectoryServiceServer::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            BytesInAllQueues: BytesInAllQueues::<Identity, OFFSET>,
            SetMachine: SetMachine::<Identity, OFFSET>,
            Machine: Machine::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Tidy: Tidy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQApplication3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQApplication as windows_core::Interface>::IID || iid == &<IMSMQApplication2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQCollection_Vtbl
    where
        Identity: IMSMQCollection_Impl,
    {
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *const core::mem::MaybeUninit<windows_core::VARIANT>, pvarret: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCollection_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pvarret.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQCoordinatedTransactionDispenser {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser_Vtbl
    where
        Identity: IMSMQCoordinatedTransactionDispenser_Impl,
    {
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQCoordinatedTransactionDispenser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCoordinatedTransactionDispenser_Impl::BeginTransaction(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQCoordinatedTransactionDispenser2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser2_Vtbl
    where
        Identity: IMSMQCoordinatedTransactionDispenser2_Impl,
    {
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQCoordinatedTransactionDispenser2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCoordinatedTransactionDispenser2_Impl::BeginTransaction(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQCoordinatedTransactionDispenser2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCoordinatedTransactionDispenser2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQCoordinatedTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQCoordinatedTransactionDispenser3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQCoordinatedTransactionDispenser3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQCoordinatedTransactionDispenser3_Vtbl
    where
        Identity: IMSMQCoordinatedTransactionDispenser3_Impl,
    {
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQCoordinatedTransactionDispenser3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCoordinatedTransactionDispenser3_Impl::BeginTransaction(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQCoordinatedTransactionDispenser3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQCoordinatedTransactionDispenser3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQCoordinatedTransactionDispenser3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn IsOpen(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IADs(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_IADs(&self, piads: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetADsPath(&self, bstradspath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Destinations(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_Destinations(&self, pdestinations: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQDestination {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQDestination_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQDestination_Vtbl
    where
        Identity: IMSMQDestination_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::Open(this).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::Close(this).into()
        }
        unsafe extern "system" fn IsOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisopen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::IsOpen(this) {
                Ok(ok__) => {
                    pfisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IADs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiads: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::IADs(this) {
                Ok(ok__) => {
                    ppiads.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_IADs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piads: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::putref_IADs(this, windows_core::from_raw_borrowed(&piads)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstradspath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::ADsPath(this) {
                Ok(ok__) => {
                    pbstradspath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetADsPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstradspath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::SetADsPath(this, core::mem::transmute(&bstradspath)).into()
        }
        unsafe extern "system" fn PathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::PathName(this) {
                Ok(ok__) => {
                    pbstrpathname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::FormatName(this) {
                Ok(ok__) => {
                    pbstrformatname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn Destinations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestinations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::Destinations(this) {
                Ok(ok__) => {
                    ppdestinations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Destinations<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinations: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQDestination_Impl::putref_Destinations(this, windows_core::from_raw_borrowed(&pdestinations)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQDestination_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            IADs: IADs::<Identity, OFFSET>,
            putref_IADs: putref_IADs::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
            SetADsPath: SetADsPath::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            Destinations: Destinations::<Identity, OFFSET>,
            putref_Destinations: putref_Destinations::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQDestination as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQEvent_Vtbl
    where
        Identity: IMSMQEvent_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent2_Impl: Sized + IMSMQEvent_Impl {
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQEvent2_Vtbl
    where
        Identity: IMSMQEvent2_Impl,
    {
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQEvent2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQEvent2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IMSMQEvent_Vtbl::new::<Identity, OFFSET>(), Properties: Properties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQEvent2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQEvent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQEvent3_Impl: Sized + IMSMQEvent2_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQEvent3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQEvent3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQEvent3_Vtbl
    where
        Identity: IMSMQEvent3_Impl,
    {
        Self { base__: IMSMQEvent2_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQEvent3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQEvent as windows_core::Interface>::IID || iid == &<IMSMQEvent2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQManagement_Impl: Sized + super::Com::IDispatch_Impl {
    fn Init(&self, machine: *const windows_core::VARIANT, pathname: *const windows_core::VARIANT, formatname: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Machine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MessageCount(&self) -> windows_core::Result<i32>;
    fn ForeignStatus(&self) -> windows_core::Result<i32>;
    fn QueueType(&self) -> windows_core::Result<i32>;
    fn IsLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TransactionalStatus(&self) -> windows_core::Result<i32>;
    fn BytesInQueue(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQManagement {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQManagement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQManagement_Vtbl
    where
        Identity: IMSMQManagement_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, machine: *const core::mem::MaybeUninit<windows_core::VARIANT>, pathname: *const core::mem::MaybeUninit<windows_core::VARIANT>, formatname: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQManagement_Impl::Init(this, core::mem::transmute_copy(&machine), core::mem::transmute_copy(&pathname), core::mem::transmute_copy(&formatname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::FormatName(this) {
                Ok(ok__) => {
                    pbstrformatname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Machine<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::Machine(this) {
                Ok(ok__) => {
                    pbstrmachine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmessagecount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::MessageCount(this) {
                Ok(ok__) => {
                    plmessagecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForeignStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plforeignstatus: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::ForeignStatus(this) {
                Ok(ok__) => {
                    plforeignstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plqueuetype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::QueueType(this) {
                Ok(ok__) => {
                    plqueuetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfislocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::IsLocal(this) {
                Ok(ok__) => {
                    pfislocal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionalStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltransactionalstatus: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::TransactionalStatus(this) {
                Ok(ok__) => {
                    pltransactionalstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytesinqueue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQManagement_Impl::BytesInQueue(this) {
                Ok(ok__) => {
                    pvbytesinqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            Machine: Machine::<Identity, OFFSET>,
            MessageCount: MessageCount::<Identity, OFFSET>,
            ForeignStatus: ForeignStatus::<Identity, OFFSET>,
            QueueType: QueueType::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            TransactionalStatus: TransactionalStatus::<Identity, OFFSET>,
            BytesInQueue: BytesInQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQManagement as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetBody(&self, varbody: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: Option<&IMSMQQueue>, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQMessage {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQMessage_Vtbl
    where
        Identity: IMSMQMessage_Impl,
    {
        unsafe extern "system" fn Class<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Class(this) {
                Ok(ok__) => {
                    plclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::AuthLevel(this) {
                Ok(ok__) => {
                    plauthlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::IsAuthenticated(this) {
                Ok(ok__) => {
                    pisauthenticated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Delivery(this) {
                Ok(ok__) => {
                    pldelivery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Trace(this) {
                Ok(ok__) => {
                    pltrace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Priority(this) {
                Ok(ok__) => {
                    plpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::ResponseQueueInfo(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::putref_ResponseQueueInfo(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    plappspecific.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::SourceMachineGuid(this) {
                Ok(ok__) => {
                    pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::BodyLength(this) {
                Ok(ok__) => {
                    pcbbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Body(this) {
                Ok(ok__) => {
                    pvarbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::AdminQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::putref_AdminQueueInfo(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Id(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::CorrelationId(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Ack(this) {
                Ok(ok__) => {
                    plack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::MaxTimeToReachQueue(this) {
                Ok(ok__) => {
                    plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::MaxTimeToReceive(this) {
                Ok(ok__) => {
                    plmaxtimetoreceive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    plhashalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::EncryptAlgorithm(this) {
                Ok(ok__) => {
                    plencryptalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::SentTime(this) {
                Ok(ok__) => {
                    pvarsenttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::ArrivedTime(this) {
                Ok(ok__) => {
                    plarrivedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::DestinationQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfodest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::SenderCertificate(this) {
                Ok(ok__) => {
                    pvarsendercert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::SenderId(this) {
                Ok(ok__) => {
                    pvarsenderid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage_Impl::SenderIdType(this) {
                Ok(ok__) => {
                    plsenderidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::Send(this, windows_core::from_raw_borrowed(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage_Impl::AttachCurrentSecurityContext(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetBody(&self, varbody: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: Option<&IMSMQQueue2>, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
    fn SenderVersion(&self) -> windows_core::Result<i32>;
    fn Extension(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetExtension(&self, varextension: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn DestinationSymmetricKey(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Signature(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSignature(&self, varsignature: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AuthenticationProviderType(&self) -> windows_core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()>;
    fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSenderId(&self, varsenderid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MsgClass(&self) -> windows_core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsFirstInTransaction(&self) -> windows_core::Result<i16>;
    fn IsLastInTransaction(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: Option<&IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: Option<&IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQMessage2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQMessage2_Vtbl
    where
        Identity: IMSMQMessage2_Impl,
    {
        unsafe extern "system" fn Class<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Class(this) {
                Ok(ok__) => {
                    plclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::AuthLevel(this) {
                Ok(ok__) => {
                    plauthlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::IsAuthenticated(this) {
                Ok(ok__) => {
                    pisauthenticated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Delivery(this) {
                Ok(ok__) => {
                    pldelivery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Trace(this) {
                Ok(ok__) => {
                    pltrace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Priority(this) {
                Ok(ok__) => {
                    plpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::ResponseQueueInfo_v1(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::putref_ResponseQueueInfo_v1(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    plappspecific.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::SourceMachineGuid(this) {
                Ok(ok__) => {
                    pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::BodyLength(this) {
                Ok(ok__) => {
                    pcbbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Body(this) {
                Ok(ok__) => {
                    pvarbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::AdminQueueInfo_v1(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::putref_AdminQueueInfo_v1(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Id(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::CorrelationId(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Ack(this) {
                Ok(ok__) => {
                    plack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::MaxTimeToReachQueue(this) {
                Ok(ok__) => {
                    plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::MaxTimeToReceive(this) {
                Ok(ok__) => {
                    plmaxtimetoreceive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    plhashalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::EncryptAlgorithm(this) {
                Ok(ok__) => {
                    plencryptalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::SentTime(this) {
                Ok(ok__) => {
                    pvarsenttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::ArrivedTime(this) {
                Ok(ok__) => {
                    plarrivedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::DestinationQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfodest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::SenderCertificate(this) {
                Ok(ok__) => {
                    pvarsendercert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::SenderId(this) {
                Ok(ok__) => {
                    pvarsenderid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::SenderIdType(this) {
                Ok(ok__) => {
                    plsenderidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::Send(this, windows_core::from_raw_borrowed(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::AttachCurrentSecurityContext(this).into()
        }
        unsafe extern "system" fn SenderVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::SenderVersion(this) {
                Ok(ok__) => {
                    plsenderversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarextension: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Extension(this) {
                Ok(ok__) => {
                    pvarextension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varextension: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetExtension(this, core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidconnectortype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::ConnectorTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidconnectortype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidconnectortype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetConnectorTypeGuid(this, core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoxactstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::TransactionStatusQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoxactstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardestsymmkey: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::DestinationSymmetricKey(this) {
                Ok(ok__) => {
                    pvardestsymmkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestsymmkey: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetDestinationSymmetricKey(this, core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsignature: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Signature(this) {
                Ok(ok__) => {
                    pvarsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsignature: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetSignature(this, core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthprovtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::AuthenticationProviderType(this) {
                Ok(ok__) => {
                    plauthprovtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthprovtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetAuthenticationProviderType(this, core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthprovname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::AuthenticationProviderName(this) {
                Ok(ok__) => {
                    pbstrauthprovname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthprovname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetAuthenticationProviderName(this, core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsenderid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetSenderId(this, core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmsgclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::MsgClass(this) {
                Ok(ok__) => {
                    plmsgclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmsgclass: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::SetMsgClass(this, core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarxactid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::TransactionId(this) {
                Ok(ok__) => {
                    pvarxactid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::IsFirstInTransaction(this) {
                Ok(ok__) => {
                    pisfirstinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::IsLastInTransaction(this) {
                Ok(ok__) => {
                    pislastinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::ResponseQueueInfo(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::putref_ResponseQueueInfo(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::AdminQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage2_Impl::putref_AdminQueueInfo(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage2_Impl::ReceivedAuthenticationLevel(this) {
                Ok(ok__) => {
                    psreceivedauthenticationlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
            SenderVersion: SenderVersion::<Identity, OFFSET>,
            Extension: Extension::<Identity, OFFSET>,
            SetExtension: SetExtension::<Identity, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, OFFSET>,
            Signature: Signature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, OFFSET>,
            SetSenderId: SetSenderId::<Identity, OFFSET>,
            MsgClass: MsgClass::<Identity, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetBody(&self, varbody: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: Option<&super::Com::IDispatch>, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
    fn SenderVersion(&self) -> windows_core::Result<i32>;
    fn Extension(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetExtension(&self, varextension: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn DestinationSymmetricKey(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Signature(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSignature(&self, varsignature: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AuthenticationProviderType(&self) -> windows_core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()>;
    fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSenderId(&self, varsenderid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MsgClass(&self) -> windows_core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsFirstInTransaction(&self) -> windows_core::Result<i16>;
    fn IsLastInTransaction(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: Option<&IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: Option<&IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: Option<&IMSMQQueueInfo3>) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: Option<&IMSMQQueueInfo3>) -> windows_core::Result<()>;
    fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Destination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()>;
    fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CompoundMessage(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQMessage3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQMessage3_Vtbl
    where
        Identity: IMSMQMessage3_Impl,
    {
        unsafe extern "system" fn Class<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Class(this) {
                Ok(ok__) => {
                    plclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AuthLevel(this) {
                Ok(ok__) => {
                    plauthlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::IsAuthenticated(this) {
                Ok(ok__) => {
                    pisauthenticated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Delivery(this) {
                Ok(ok__) => {
                    pldelivery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Trace(this) {
                Ok(ok__) => {
                    pltrace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Priority(this) {
                Ok(ok__) => {
                    plpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ResponseQueueInfo_v1(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_ResponseQueueInfo_v1(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    plappspecific.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SourceMachineGuid(this) {
                Ok(ok__) => {
                    pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::BodyLength(this) {
                Ok(ok__) => {
                    pcbbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Body(this) {
                Ok(ok__) => {
                    pvarbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AdminQueueInfo_v1(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_AdminQueueInfo_v1(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Id(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::CorrelationId(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Ack(this) {
                Ok(ok__) => {
                    plack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::MaxTimeToReachQueue(this) {
                Ok(ok__) => {
                    plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::MaxTimeToReceive(this) {
                Ok(ok__) => {
                    plmaxtimetoreceive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    plhashalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::EncryptAlgorithm(this) {
                Ok(ok__) => {
                    plencryptalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SentTime(this) {
                Ok(ok__) => {
                    pvarsenttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ArrivedTime(this) {
                Ok(ok__) => {
                    plarrivedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::DestinationQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfodest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SenderCertificate(this) {
                Ok(ok__) => {
                    pvarsendercert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SenderId(this) {
                Ok(ok__) => {
                    pvarsenderid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SenderIdType(this) {
                Ok(ok__) => {
                    plsenderidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::Send(this, windows_core::from_raw_borrowed(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::AttachCurrentSecurityContext(this).into()
        }
        unsafe extern "system" fn SenderVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SenderVersion(this) {
                Ok(ok__) => {
                    plsenderversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarextension: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Extension(this) {
                Ok(ok__) => {
                    pvarextension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varextension: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetExtension(this, core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidconnectortype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ConnectorTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidconnectortype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidconnectortype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetConnectorTypeGuid(this, core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoxactstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::TransactionStatusQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoxactstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardestsymmkey: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::DestinationSymmetricKey(this) {
                Ok(ok__) => {
                    pvardestsymmkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestsymmkey: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetDestinationSymmetricKey(this, core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsignature: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Signature(this) {
                Ok(ok__) => {
                    pvarsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsignature: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetSignature(this, core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthprovtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AuthenticationProviderType(this) {
                Ok(ok__) => {
                    plauthprovtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthprovtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetAuthenticationProviderType(this, core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthprovname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AuthenticationProviderName(this) {
                Ok(ok__) => {
                    pbstrauthprovname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthprovname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetAuthenticationProviderName(this, core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsenderid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetSenderId(this, core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmsgclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::MsgClass(this) {
                Ok(ok__) => {
                    plmsgclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmsgclass: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetMsgClass(this, core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarxactid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::TransactionId(this) {
                Ok(ok__) => {
                    pvarxactid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::IsFirstInTransaction(this) {
                Ok(ok__) => {
                    pisfirstinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::IsLastInTransaction(this) {
                Ok(ok__) => {
                    pislastinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ResponseQueueInfo_v2(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_ResponseQueueInfo_v2(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AdminQueueInfo_v2(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_AdminQueueInfo_v2(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ReceivedAuthenticationLevel(this) {
                Ok(ok__) => {
                    psreceivedauthenticationlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ResponseQueueInfo(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_ResponseQueueInfo(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::AdminQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_AdminQueueInfo(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::ResponseDestination(this) {
                Ok(ok__) => {
                    ppdestresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestresponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::putref_ResponseDestination(this, windows_core::from_raw_borrowed(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::Destination(this) {
                Ok(ok__) => {
                    ppdestdestination.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarlookupid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::LookupId(this) {
                Ok(ok__) => {
                    pvarlookupid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::IsAuthenticated2(this) {
                Ok(ok__) => {
                    pisauthenticated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::IsFirstInTransaction2(this) {
                Ok(ok__) => {
                    pisfirstinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::IsLastInTransaction2(this) {
                Ok(ok__) => {
                    pislastinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::AttachCurrentSecurityContext2(this).into()
        }
        unsafe extern "system" fn SoapEnvelope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsoapenvelope: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::SoapEnvelope(this) {
                Ok(ok__) => {
                    pbstrsoapenvelope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcompoundmessage: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage3_Impl::CompoundMessage(this) {
                Ok(ok__) => {
                    pvarcompoundmessage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapheader: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetSoapHeader(this, core::mem::transmute(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapbody: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage3_Impl::SetSoapBody(this, core::mem::transmute(&bstrsoapbody)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
            SenderVersion: SenderVersion::<Identity, OFFSET>,
            Extension: Extension::<Identity, OFFSET>,
            SetExtension: SetExtension::<Identity, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, OFFSET>,
            Signature: Signature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, OFFSET>,
            SetSenderId: SetSenderId::<Identity, OFFSET>,
            MsgClass: MsgClass::<Identity, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            LookupId: LookupId::<Identity, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQMessage4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Class(&self) -> windows_core::Result<i32>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn AuthLevel(&self) -> windows_core::Result<i32>;
    fn SetAuthLevel(&self, lauthlevel: i32) -> windows_core::Result<()>;
    fn IsAuthenticated(&self) -> windows_core::Result<i16>;
    fn Delivery(&self) -> windows_core::Result<i32>;
    fn SetDelivery(&self, ldelivery: i32) -> windows_core::Result<()>;
    fn Trace(&self) -> windows_core::Result<i32>;
    fn SetTrace(&self, ltrace: i32) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<i32>;
    fn SetPriority(&self, lpriority: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn ResponseQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_ResponseQueueInfo_v1(&self, pqinforesponse: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn AppSpecific(&self) -> windows_core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> windows_core::Result<()>;
    fn SourceMachineGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BodyLength(&self) -> windows_core::Result<i32>;
    fn Body(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetBody(&self, varbody: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AdminQueueInfo_v1(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn putref_AdminQueueInfo_v1(&self, pqinfoadmin: Option<&IMSMQQueueInfo>) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn CorrelationId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetCorrelationId(&self, varmsgid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Ack(&self) -> windows_core::Result<i32>;
    fn SetAck(&self, lack: i32) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MaxTimeToReachQueue(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReachQueue(&self, lmaxtimetoreachqueue: i32) -> windows_core::Result<()>;
    fn MaxTimeToReceive(&self) -> windows_core::Result<i32>;
    fn SetMaxTimeToReceive(&self, lmaxtimetoreceive: i32) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetHashAlgorithm(&self, lhashalg: i32) -> windows_core::Result<()>;
    fn EncryptAlgorithm(&self) -> windows_core::Result<i32>;
    fn SetEncryptAlgorithm(&self, lencryptalg: i32) -> windows_core::Result<()>;
    fn SentTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ArrivedTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn DestinationQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn SenderCertificate(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSenderCertificate(&self, varsendercert: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SenderId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SenderIdType(&self) -> windows_core::Result<i32>;
    fn SetSenderIdType(&self, lsenderidtype: i32) -> windows_core::Result<()>;
    fn Send(&self, destinationqueue: Option<&super::Com::IDispatch>, transaction: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn AttachCurrentSecurityContext(&self) -> windows_core::Result<()>;
    fn SenderVersion(&self) -> windows_core::Result<i32>;
    fn Extension(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetExtension(&self, varextension: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn ConnectorTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConnectorTypeGuid(&self, bstrguidconnectortype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TransactionStatusQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn DestinationSymmetricKey(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetDestinationSymmetricKey(&self, vardestsymmkey: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Signature(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSignature(&self, varsignature: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AuthenticationProviderType(&self) -> windows_core::Result<i32>;
    fn SetAuthenticationProviderType(&self, lauthprovtype: i32) -> windows_core::Result<()>;
    fn AuthenticationProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAuthenticationProviderName(&self, bstrauthprovname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSenderId(&self, varsenderid: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn MsgClass(&self) -> windows_core::Result<i32>;
    fn SetMsgClass(&self, lmsgclass: i32) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn TransactionId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsFirstInTransaction(&self) -> windows_core::Result<i16>;
    fn IsLastInTransaction(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_ResponseQueueInfo_v2(&self, pqinforesponse: Option<&IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn AdminQueueInfo_v2(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn putref_AdminQueueInfo_v2(&self, pqinfoadmin: Option<&IMSMQQueueInfo2>) -> windows_core::Result<()>;
    fn ReceivedAuthenticationLevel(&self) -> windows_core::Result<i16>;
    fn ResponseQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn putref_ResponseQueueInfo(&self, pqinforesponse: Option<&IMSMQQueueInfo4>) -> windows_core::Result<()>;
    fn AdminQueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn putref_AdminQueueInfo(&self, pqinfoadmin: Option<&IMSMQQueueInfo4>) -> windows_core::Result<()>;
    fn ResponseDestination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn putref_ResponseDestination(&self, pdestresponse: Option<&super::Com::IDispatch>) -> windows_core::Result<()>;
    fn Destination(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn IsAuthenticated2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsFirstInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsLastInTransaction2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AttachCurrentSecurityContext2(&self) -> windows_core::Result<()>;
    fn SoapEnvelope(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CompoundMessage(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSoapHeader(&self, bstrsoapheader: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSoapBody(&self, bstrsoapbody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQMessage4 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQMessage4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQMessage4_Vtbl
    where
        Identity: IMSMQMessage4_Impl,
    {
        unsafe extern "system" fn Class<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Class(this) {
                Ok(ok__) => {
                    plclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn AuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AuthLevel(this) {
                Ok(ok__) => {
                    plauthlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetAuthLevel(this, core::mem::transmute_copy(&lauthlevel)).into()
        }
        unsafe extern "system" fn IsAuthenticated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::IsAuthenticated(this) {
                Ok(ok__) => {
                    pisauthenticated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldelivery: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Delivery(this) {
                Ok(ok__) => {
                    pldelivery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelivery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ldelivery: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetDelivery(this, core::mem::transmute_copy(&ldelivery)).into()
        }
        unsafe extern "system" fn Trace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltrace: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Trace(this) {
                Ok(ok__) => {
                    pltrace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltrace: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetTrace(this, core::mem::transmute_copy(&ltrace)).into()
        }
        unsafe extern "system" fn Priority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Priority(this) {
                Ok(ok__) => {
                    plpriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetPriority(this, core::mem::transmute_copy(&lpriority)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn ResponseQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ResponseQueueInfo_v1(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_ResponseQueueInfo_v1(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plappspecific: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AppSpecific(this) {
                Ok(ok__) => {
                    plappspecific.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lappspecific: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetAppSpecific(this, core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn SourceMachineGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidsrcmachine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SourceMachineGuid(this) {
                Ok(ok__) => {
                    pbstrguidsrcmachine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodyLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbody: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::BodyLength(this) {
                Ok(ok__) => {
                    pcbbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Body<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbody: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Body(this) {
                Ok(ok__) => {
                    pvarbody.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varbody: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetBody(this, core::mem::transmute(&varbody)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AdminQueueInfo_v1(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_AdminQueueInfo_v1(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Id(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmsgid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::CorrelationId(this) {
                Ok(ok__) => {
                    pvarmsgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varmsgid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetCorrelationId(this, core::mem::transmute(&varmsgid)).into()
        }
        unsafe extern "system" fn Ack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plack: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Ack(this) {
                Ok(ok__) => {
                    plack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAck<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lack: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetAck(this, core::mem::transmute_copy(&lack)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn MaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreachqueue: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::MaxTimeToReachQueue(this) {
                Ok(ok__) => {
                    plmaxtimetoreachqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReachQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreachqueue: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetMaxTimeToReachQueue(this, core::mem::transmute_copy(&lmaxtimetoreachqueue)).into()
        }
        unsafe extern "system" fn MaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaxtimetoreceive: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::MaxTimeToReceive(this) {
                Ok(ok__) => {
                    plmaxtimetoreceive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxTimeToReceive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxtimetoreceive: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetMaxTimeToReceive(this, core::mem::transmute_copy(&lmaxtimetoreceive)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhashalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    plhashalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lhashalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetHashAlgorithm(this, core::mem::transmute_copy(&lhashalg)).into()
        }
        unsafe extern "system" fn EncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plencryptalg: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::EncryptAlgorithm(this) {
                Ok(ok__) => {
                    plencryptalg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptAlgorithm<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lencryptalg: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetEncryptAlgorithm(this, core::mem::transmute_copy(&lencryptalg)).into()
        }
        unsafe extern "system" fn SentTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenttime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SentTime(this) {
                Ok(ok__) => {
                    pvarsenttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrivedTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plarrivedtime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ArrivedTime(this) {
                Ok(ok__) => {
                    plarrivedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfodest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::DestinationQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfodest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsendercert: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SenderCertificate(this) {
                Ok(ok__) => {
                    pvarsendercert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderCertificate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsendercert: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetSenderCertificate(this, core::mem::transmute(&varsendercert)).into()
        }
        unsafe extern "system" fn SenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsenderid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SenderId(this) {
                Ok(ok__) => {
                    pvarsenderid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderidtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SenderIdType(this) {
                Ok(ok__) => {
                    plsenderidtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSenderIdType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsenderidtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetSenderIdType(this, core::mem::transmute_copy(&lsenderidtype)).into()
        }
        unsafe extern "system" fn Send<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationqueue: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::Send(this, windows_core::from_raw_borrowed(&destinationqueue), core::mem::transmute_copy(&transaction)).into()
        }
        unsafe extern "system" fn AttachCurrentSecurityContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::AttachCurrentSecurityContext(this).into()
        }
        unsafe extern "system" fn SenderVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsenderversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SenderVersion(this) {
                Ok(ok__) => {
                    plsenderversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarextension: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Extension(this) {
                Ok(ok__) => {
                    pvarextension.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtension<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varextension: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetExtension(this, core::mem::transmute(&varextension)).into()
        }
        unsafe extern "system" fn ConnectorTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidconnectortype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ConnectorTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidconnectortype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectorTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidconnectortype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetConnectorTypeGuid(this, core::mem::transmute(&bstrguidconnectortype)).into()
        }
        unsafe extern "system" fn TransactionStatusQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoxactstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::TransactionStatusQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoxactstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationSymmetricKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardestsymmkey: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::DestinationSymmetricKey(this) {
                Ok(ok__) => {
                    pvardestsymmkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestinationSymmetricKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vardestsymmkey: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetDestinationSymmetricKey(this, core::mem::transmute(&vardestsymmkey)).into()
        }
        unsafe extern "system" fn Signature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsignature: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Signature(this) {
                Ok(ok__) => {
                    pvarsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsignature: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetSignature(this, core::mem::transmute(&varsignature)).into()
        }
        unsafe extern "system" fn AuthenticationProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthprovtype: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AuthenticationProviderType(this) {
                Ok(ok__) => {
                    plauthprovtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthprovtype: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetAuthenticationProviderType(this, core::mem::transmute_copy(&lauthprovtype)).into()
        }
        unsafe extern "system" fn AuthenticationProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrauthprovname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AuthenticationProviderName(this) {
                Ok(ok__) => {
                    pbstrauthprovname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationProviderName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthprovname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetAuthenticationProviderName(this, core::mem::transmute(&bstrauthprovname)).into()
        }
        unsafe extern "system" fn SetSenderId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsenderid: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetSenderId(this, core::mem::transmute(&varsenderid)).into()
        }
        unsafe extern "system" fn MsgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmsgclass: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::MsgClass(this) {
                Ok(ok__) => {
                    plmsgclass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMsgClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmsgclass: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetMsgClass(this, core::mem::transmute_copy(&lmsgclass)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarxactid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::TransactionId(this) {
                Ok(ok__) => {
                    pvarxactid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::IsFirstInTransaction(this) {
                Ok(ok__) => {
                    pisfirstinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::IsLastInTransaction(this) {
                Ok(ok__) => {
                    pislastinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ResponseQueueInfo_v2(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_ResponseQueueInfo_v2(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AdminQueueInfo_v2(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_AdminQueueInfo_v2(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ReceivedAuthenticationLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreceivedauthenticationlevel: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ReceivedAuthenticationLevel(this) {
                Ok(ok__) => {
                    psreceivedauthenticationlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinforesponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ResponseQueueInfo(this) {
                Ok(ok__) => {
                    ppqinforesponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinforesponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_ResponseQueueInfo(this, windows_core::from_raw_borrowed(&pqinforesponse)).into()
        }
        unsafe extern "system" fn AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfoadmin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::AdminQueueInfo(this) {
                Ok(ok__) => {
                    ppqinfoadmin.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AdminQueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqinfoadmin: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_AdminQueueInfo(this, windows_core::from_raw_borrowed(&pqinfoadmin)).into()
        }
        unsafe extern "system" fn ResponseDestination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestresponse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::ResponseDestination(this) {
                Ok(ok__) => {
                    ppdestresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_ResponseDestination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestresponse: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::putref_ResponseDestination(this, windows_core::from_raw_borrowed(&pdestresponse)).into()
        }
        unsafe extern "system" fn Destination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdestdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::Destination(this) {
                Ok(ok__) => {
                    ppdestdestination.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarlookupid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::LookupId(this) {
                Ok(ok__) => {
                    pvarlookupid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticated2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisauthenticated: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::IsAuthenticated2(this) {
                Ok(ok__) => {
                    pisauthenticated.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFirstInTransaction2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisfirstinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::IsFirstInTransaction2(this) {
                Ok(ok__) => {
                    pisfirstinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLastInTransaction2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislastinxact: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::IsLastInTransaction2(this) {
                Ok(ok__) => {
                    pislastinxact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachCurrentSecurityContext2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::AttachCurrentSecurityContext2(this).into()
        }
        unsafe extern "system" fn SoapEnvelope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsoapenvelope: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::SoapEnvelope(this) {
                Ok(ok__) => {
                    pbstrsoapenvelope.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompoundMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcompoundmessage: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQMessage4_Impl::CompoundMessage(this) {
                Ok(ok__) => {
                    pvarcompoundmessage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSoapHeader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapheader: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetSoapHeader(this, core::mem::transmute(&bstrsoapheader)).into()
        }
        unsafe extern "system" fn SetSoapBody<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsoapbody: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQMessage4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQMessage4_Impl::SetSoapBody(this, core::mem::transmute(&bstrsoapbody)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Class: Class::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            AuthLevel: AuthLevel::<Identity, OFFSET>,
            SetAuthLevel: SetAuthLevel::<Identity, OFFSET>,
            IsAuthenticated: IsAuthenticated::<Identity, OFFSET>,
            Delivery: Delivery::<Identity, OFFSET>,
            SetDelivery: SetDelivery::<Identity, OFFSET>,
            Trace: Trace::<Identity, OFFSET>,
            SetTrace: SetTrace::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            ResponseQueueInfo_v1: ResponseQueueInfo_v1::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v1: putref_ResponseQueueInfo_v1::<Identity, OFFSET>,
            AppSpecific: AppSpecific::<Identity, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, OFFSET>,
            SourceMachineGuid: SourceMachineGuid::<Identity, OFFSET>,
            BodyLength: BodyLength::<Identity, OFFSET>,
            Body: Body::<Identity, OFFSET>,
            SetBody: SetBody::<Identity, OFFSET>,
            AdminQueueInfo_v1: AdminQueueInfo_v1::<Identity, OFFSET>,
            putref_AdminQueueInfo_v1: putref_AdminQueueInfo_v1::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            SetCorrelationId: SetCorrelationId::<Identity, OFFSET>,
            Ack: Ack::<Identity, OFFSET>,
            SetAck: SetAck::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            MaxTimeToReachQueue: MaxTimeToReachQueue::<Identity, OFFSET>,
            SetMaxTimeToReachQueue: SetMaxTimeToReachQueue::<Identity, OFFSET>,
            MaxTimeToReceive: MaxTimeToReceive::<Identity, OFFSET>,
            SetMaxTimeToReceive: SetMaxTimeToReceive::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            EncryptAlgorithm: EncryptAlgorithm::<Identity, OFFSET>,
            SetEncryptAlgorithm: SetEncryptAlgorithm::<Identity, OFFSET>,
            SentTime: SentTime::<Identity, OFFSET>,
            ArrivedTime: ArrivedTime::<Identity, OFFSET>,
            DestinationQueueInfo: DestinationQueueInfo::<Identity, OFFSET>,
            SenderCertificate: SenderCertificate::<Identity, OFFSET>,
            SetSenderCertificate: SetSenderCertificate::<Identity, OFFSET>,
            SenderId: SenderId::<Identity, OFFSET>,
            SenderIdType: SenderIdType::<Identity, OFFSET>,
            SetSenderIdType: SetSenderIdType::<Identity, OFFSET>,
            Send: Send::<Identity, OFFSET>,
            AttachCurrentSecurityContext: AttachCurrentSecurityContext::<Identity, OFFSET>,
            SenderVersion: SenderVersion::<Identity, OFFSET>,
            Extension: Extension::<Identity, OFFSET>,
            SetExtension: SetExtension::<Identity, OFFSET>,
            ConnectorTypeGuid: ConnectorTypeGuid::<Identity, OFFSET>,
            SetConnectorTypeGuid: SetConnectorTypeGuid::<Identity, OFFSET>,
            TransactionStatusQueueInfo: TransactionStatusQueueInfo::<Identity, OFFSET>,
            DestinationSymmetricKey: DestinationSymmetricKey::<Identity, OFFSET>,
            SetDestinationSymmetricKey: SetDestinationSymmetricKey::<Identity, OFFSET>,
            Signature: Signature::<Identity, OFFSET>,
            SetSignature: SetSignature::<Identity, OFFSET>,
            AuthenticationProviderType: AuthenticationProviderType::<Identity, OFFSET>,
            SetAuthenticationProviderType: SetAuthenticationProviderType::<Identity, OFFSET>,
            AuthenticationProviderName: AuthenticationProviderName::<Identity, OFFSET>,
            SetAuthenticationProviderName: SetAuthenticationProviderName::<Identity, OFFSET>,
            SetSenderId: SetSenderId::<Identity, OFFSET>,
            MsgClass: MsgClass::<Identity, OFFSET>,
            SetMsgClass: SetMsgClass::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            IsFirstInTransaction: IsFirstInTransaction::<Identity, OFFSET>,
            IsLastInTransaction: IsLastInTransaction::<Identity, OFFSET>,
            ResponseQueueInfo_v2: ResponseQueueInfo_v2::<Identity, OFFSET>,
            putref_ResponseQueueInfo_v2: putref_ResponseQueueInfo_v2::<Identity, OFFSET>,
            AdminQueueInfo_v2: AdminQueueInfo_v2::<Identity, OFFSET>,
            putref_AdminQueueInfo_v2: putref_AdminQueueInfo_v2::<Identity, OFFSET>,
            ReceivedAuthenticationLevel: ReceivedAuthenticationLevel::<Identity, OFFSET>,
            ResponseQueueInfo: ResponseQueueInfo::<Identity, OFFSET>,
            putref_ResponseQueueInfo: putref_ResponseQueueInfo::<Identity, OFFSET>,
            AdminQueueInfo: AdminQueueInfo::<Identity, OFFSET>,
            putref_AdminQueueInfo: putref_AdminQueueInfo::<Identity, OFFSET>,
            ResponseDestination: ResponseDestination::<Identity, OFFSET>,
            putref_ResponseDestination: putref_ResponseDestination::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            LookupId: LookupId::<Identity, OFFSET>,
            IsAuthenticated2: IsAuthenticated2::<Identity, OFFSET>,
            IsFirstInTransaction2: IsFirstInTransaction2::<Identity, OFFSET>,
            IsLastInTransaction2: IsLastInTransaction2::<Identity, OFFSET>,
            AttachCurrentSecurityContext2: AttachCurrentSecurityContext2::<Identity, OFFSET>,
            SoapEnvelope: SoapEnvelope::<Identity, OFFSET>,
            CompoundMessage: CompoundMessage::<Identity, OFFSET>,
            SetSoapHeader: SetSoapHeader::<Identity, OFFSET>,
            SetSoapBody: SetSoapBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQMessage4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQOutgoingQueueManagement_Impl: Sized + IMSMQManagement_Impl {
    fn State(&self) -> windows_core::Result<i32>;
    fn NextHops(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EodGetSendInfo(&self) -> windows_core::Result<IMSMQCollection>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn EodResend(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQOutgoingQueueManagement {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQOutgoingQueueManagement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQOutgoingQueueManagement_Vtbl
    where
        Identity: IMSMQOutgoingQueueManagement_Impl,
    {
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plstate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQOutgoingQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQOutgoingQueueManagement_Impl::State(this) {
                Ok(ok__) => {
                    plstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextHops<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvnexthops: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQOutgoingQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQOutgoingQueueManagement_Impl::NextHops(this) {
                Ok(ok__) => {
                    pvnexthops.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetSendInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQOutgoingQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQOutgoingQueueManagement_Impl::EodGetSendInfo(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQOutgoingQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQOutgoingQueueManagement_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQOutgoingQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQOutgoingQueueManagement_Impl::Pause(this).into()
        }
        unsafe extern "system" fn EodResend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQOutgoingQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQOutgoingQueueManagement_Impl::EodResend(this).into()
        }
        Self {
            base__: IMSMQManagement_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, OFFSET>,
            NextHops: NextHops::<Identity, OFFSET>,
            EodGetSendInfo: EodGetSendInfo::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            EodResend: EodResend::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQOutgoingQueueManagement as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQManagement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQPrivateDestination_Impl: Sized + super::Com::IDispatch_Impl {
    fn Handle(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetHandle(&self, varhandle: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQPrivateDestination {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateDestination_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQPrivateDestination_Vtbl
    where
        Identity: IMSMQPrivateDestination_Impl,
    {
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarhandle: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQPrivateDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQPrivateDestination_Impl::Handle(this) {
                Ok(ok__) => {
                    pvarhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varhandle: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQPrivateDestination_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQPrivateDestination_Impl::SetHandle(this, core::mem::transmute(&varhandle)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Handle: Handle::<Identity, OFFSET>, SetHandle: SetHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQPrivateDestination as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQPrivateEvent_Impl: Sized + super::Com::IDispatch_Impl {
    fn Hwnd(&self) -> windows_core::Result<i32>;
    fn FireArrivedEvent(&self, pq: Option<&IMSMQQueue>, msgcursor: i32) -> windows_core::Result<()>;
    fn FireArrivedErrorEvent(&self, pq: Option<&IMSMQQueue>, hrstatus: windows_core::HRESULT, msgcursor: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQPrivateEvent {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQPrivateEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQPrivateEvent_Vtbl
    where
        Identity: IMSMQPrivateEvent_Impl,
    {
        unsafe extern "system" fn Hwnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQPrivateEvent_Impl::Hwnd(this) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireArrivedEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pq: *mut core::ffi::c_void, msgcursor: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQPrivateEvent_Impl::FireArrivedEvent(this, windows_core::from_raw_borrowed(&pq), core::mem::transmute_copy(&msgcursor)).into()
        }
        unsafe extern "system" fn FireArrivedErrorEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pq: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT, msgcursor: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQPrivateEvent_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQPrivateEvent_Impl::FireArrivedErrorEvent(this, windows_core::from_raw_borrowed(&pq), core::mem::transmute_copy(&hrstatus), core::mem::transmute_copy(&msgcursor)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Hwnd: Hwnd::<Identity, OFFSET>,
            FireArrivedEvent: FireArrivedEvent::<Identity, OFFSET>,
            FireArrivedErrorEvent: FireArrivedErrorEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQPrivateEvent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQuery {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQuery_Vtbl
    where
        Identity: IMSMQQuery_Impl,
    {
        unsafe extern "system" fn LookupQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const core::mem::MaybeUninit<windows_core::VARIANT>, servicetypeguid: *const core::mem::MaybeUninit<windows_core::VARIANT>, label: *const core::mem::MaybeUninit<windows_core::VARIANT>, createtime: *const core::mem::MaybeUninit<windows_core::VARIANT>, modifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>, relservicetype: *const core::mem::MaybeUninit<windows_core::VARIANT>, rellabel: *const core::mem::MaybeUninit<windows_core::VARIANT>, relcreatetime: *const core::mem::MaybeUninit<windows_core::VARIANT>, relmodifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                Ok(ok__) => {
                    ppqinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), LookupQueue: LookupQueue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery2_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQuery2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQuery2_Vtbl
    where
        Identity: IMSMQQuery2_Impl,
    {
        unsafe extern "system" fn LookupQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, queueguid: *const core::mem::MaybeUninit<windows_core::VARIANT>, servicetypeguid: *const core::mem::MaybeUninit<windows_core::VARIANT>, label: *const core::mem::MaybeUninit<windows_core::VARIANT>, createtime: *const core::mem::MaybeUninit<windows_core::VARIANT>, modifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>, relservicetype: *const core::mem::MaybeUninit<windows_core::VARIANT>, rellabel: *const core::mem::MaybeUninit<windows_core::VARIANT>, relcreatetime: *const core::mem::MaybeUninit<windows_core::VARIANT>, relmodifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppqinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery2_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                Ok(ok__) => {
                    ppqinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LookupQueue: LookupQueue::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery3_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT, multicastaddress: *const windows_core::VARIANT, relmulticastaddress: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos3>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQuery3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQuery3_Vtbl
    where
        Identity: IMSMQQuery3_Impl,
    {
        unsafe extern "system" fn LookupQueue_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            queueguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            servicetypeguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            label: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            createtime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            modifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relservicetype: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            rellabel: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relcreatetime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relmodifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            ppqinfos: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery3_Impl::LookupQueue_v2(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                Ok(ok__) => {
                    ppqinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            queueguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            servicetypeguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            label: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            createtime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            modifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relservicetype: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            rellabel: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relcreatetime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relmodifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            multicastaddress: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relmulticastaddress: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            ppqinfos: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery3_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime), core::mem::transmute_copy(&multicastaddress), core::mem::transmute_copy(&relmulticastaddress)) {
                Ok(ok__) => {
                    ppqinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            LookupQueue: LookupQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQuery4_Impl: Sized + super::Com::IDispatch_Impl {
    fn LookupQueue_v2(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos4>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn LookupQueue(&self, queueguid: *const windows_core::VARIANT, servicetypeguid: *const windows_core::VARIANT, label: *const windows_core::VARIANT, createtime: *const windows_core::VARIANT, modifytime: *const windows_core::VARIANT, relservicetype: *const windows_core::VARIANT, rellabel: *const windows_core::VARIANT, relcreatetime: *const windows_core::VARIANT, relmodifytime: *const windows_core::VARIANT, multicastaddress: *const windows_core::VARIANT, relmulticastaddress: *const windows_core::VARIANT) -> windows_core::Result<IMSMQQueueInfos4>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQuery4 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQuery4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQuery4_Vtbl
    where
        Identity: IMSMQQuery4_Impl,
    {
        unsafe extern "system" fn LookupQueue_v2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            queueguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            servicetypeguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            label: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            createtime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            modifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relservicetype: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            rellabel: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relcreatetime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relmodifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            ppqinfos: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery4_Impl::LookupQueue_v2(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime)) {
                Ok(ok__) => {
                    ppqinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery4_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LookupQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            queueguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            servicetypeguid: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            label: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            createtime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            modifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relservicetype: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            rellabel: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relcreatetime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relmodifytime: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            multicastaddress: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            relmulticastaddress: *const core::mem::MaybeUninit<windows_core::VARIANT>,
            ppqinfos: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT
        where
            Identity: IMSMQQuery4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQuery4_Impl::LookupQueue(this, core::mem::transmute_copy(&queueguid), core::mem::transmute_copy(&servicetypeguid), core::mem::transmute_copy(&label), core::mem::transmute_copy(&createtime), core::mem::transmute_copy(&modifytime), core::mem::transmute_copy(&relservicetype), core::mem::transmute_copy(&rellabel), core::mem::transmute_copy(&relcreatetime), core::mem::transmute_copy(&relmodifytime), core::mem::transmute_copy(&multicastaddress), core::mem::transmute_copy(&relmulticastaddress)) {
                Ok(ok__) => {
                    ppqinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LookupQueue_v2: LookupQueue_v2::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            LookupQueue: LookupQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQuery4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: Option<&IMSMQEvent>, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueue {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueue_Vtbl
    where
        Identity: IMSMQQueue_Impl,
    {
        unsafe extern "system" fn Access<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::Access(this) {
                Ok(ok__) => {
                    placcess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::ShareMode(this) {
                Ok(ok__) => {
                    plsharemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::QueueInfo(this) {
                Ok(ok__) => {
                    ppqinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::Handle(this) {
                Ok(ok__) => {
                    plhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::IsOpen(this) {
                Ok(ok__) => {
                    pisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue_Impl::Close(this).into()
        }
        unsafe extern "system" fn Receive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue_Impl::EnableNotification(this, windows_core::from_raw_borrowed(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue_Impl::Reset(this).into()
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: Option<&IMSMQEvent2>, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueue2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueue2_Vtbl
    where
        Identity: IMSMQQueue2_Impl,
    {
        unsafe extern "system" fn Access<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Access(this) {
                Ok(ok__) => {
                    placcess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::ShareMode(this) {
                Ok(ok__) => {
                    plsharemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::QueueInfo(this) {
                Ok(ok__) => {
                    ppqinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Handle(this) {
                Ok(ok__) => {
                    plhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::IsOpen(this) {
                Ok(ok__) => {
                    pisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue2_Impl::Close(this).into()
        }
        unsafe extern "system" fn Receive_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Receive_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Peek_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue2_Impl::EnableNotification(this, windows_core::from_raw_borrowed(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue2_Impl::Reset(this).into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::ReceiveCurrent_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::PeekNext_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::PeekCurrent_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive_v1: Receive_v1::<Identity, OFFSET>,
            Peek_v1: Peek_v1::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: Option<&IMSMQEvent3>, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Handle2(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ReceiveByLookupId(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveNextByLookupId(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceivePreviousByLookupId(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveFirstByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn ReceiveLastByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekByLookupId(&self, lookupid: &windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekNextByLookupId(&self, lookupid: &windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekPreviousByLookupId(&self, lookupid: &windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekFirstByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn PeekLastByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage3>;
    fn Purge(&self) -> windows_core::Result<()>;
    fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueue3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueue3_Vtbl
    where
        Identity: IMSMQQueue3_Impl,
    {
        unsafe extern "system" fn Access<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Access(this) {
                Ok(ok__) => {
                    placcess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ShareMode(this) {
                Ok(ok__) => {
                    plsharemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::QueueInfo(this) {
                Ok(ok__) => {
                    ppqinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Handle(this) {
                Ok(ok__) => {
                    plhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::IsOpen(this) {
                Ok(ok__) => {
                    pisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue3_Impl::Close(this).into()
        }
        unsafe extern "system" fn Receive_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Receive_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Peek_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue3_Impl::EnableNotification(this, windows_core::from_raw_borrowed(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue3_Impl::Reset(this).into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceiveCurrent_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekNext_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekCurrent_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarhandle: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::Handle2(this) {
                Ok(ok__) => {
                    pvarhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceiveByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceiveNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceivePreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceiveFirstByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::ReceiveLastByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekPreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekFirstByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::PeekLastByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue3_Impl::Purge(this).into()
        }
        unsafe extern "system" fn IsOpen2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue3_Impl::IsOpen2(this) {
                Ok(ok__) => {
                    pisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive_v1: Receive_v1::<Identity, OFFSET>,
            Peek_v1: Peek_v1::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Handle2: Handle2::<Identity, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
            IsOpen2: IsOpen2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueue4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Access(&self) -> windows_core::Result<i32>;
    fn ShareMode(&self) -> windows_core::Result<i32>;
    fn QueueInfo(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn Handle(&self) -> windows_core::Result<i32>;
    fn IsOpen(&self) -> windows_core::Result<i16>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Receive_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Peek_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn EnableNotification(&self, event: Option<&IMSMQEvent3>, cursor: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn ReceiveCurrent_v1(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekNext_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn PeekCurrent_v1(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage>;
    fn Receive(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn Peek(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveCurrent(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekNext(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekCurrent(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, receivetimeout: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Handle2(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ReceiveByLookupId(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveNextByLookupId(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceivePreviousByLookupId(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveFirstByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn ReceiveLastByLookupId(&self, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekByLookupId(&self, lookupid: &windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekNextByLookupId(&self, lookupid: &windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekPreviousByLookupId(&self, lookupid: &windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekFirstByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn PeekLastByLookupId(&self, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
    fn Purge(&self) -> windows_core::Result<()>;
    fn IsOpen2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ReceiveByLookupIdAllowPeek(&self, lookupid: &windows_core::VARIANT, transaction: *const windows_core::VARIANT, wantdestinationqueue: *const windows_core::VARIANT, wantbody: *const windows_core::VARIANT, wantconnectortype: *const windows_core::VARIANT) -> windows_core::Result<IMSMQMessage4>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueue4 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueue4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueue4_Vtbl
    where
        Identity: IMSMQQueue4_Impl,
    {
        unsafe extern "system" fn Access<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, placcess: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Access(this) {
                Ok(ok__) => {
                    placcess.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsharemode: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ShareMode(this) {
                Ok(ok__) => {
                    plsharemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::QueueInfo(this) {
                Ok(ok__) => {
                    ppqinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plhandle: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Handle(this) {
                Ok(ok__) => {
                    plhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::IsOpen(this) {
                Ok(ok__) => {
                    pisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue4_Impl::Close(this).into()
        }
        unsafe extern "system" fn Receive_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Receive_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Peek_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *mut core::ffi::c_void, cursor: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue4_Impl::EnableNotification(this, windows_core::from_raw_borrowed(&event), core::mem::transmute_copy(&cursor), core::mem::transmute_copy(&receivetimeout)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue4_Impl::Reset(this).into()
        }
        unsafe extern "system" fn ReceiveCurrent_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveCurrent_v1(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekNext_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent_v1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekCurrent_v1(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Receive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Receive(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Peek(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveCurrent(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekNext(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, receivetimeout: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekCurrent(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&receivetimeout), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarhandle: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::Handle2(this) {
                Ok(ok__) => {
                    pvarhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveNextByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivePreviousByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceivePreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveFirstByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveFirstByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveLastByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveLastByLookupId(this, core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekNextByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekNextByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekPreviousByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekPreviousByLookupId(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekFirstByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekFirstByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeekLastByLookupId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::PeekLastByLookupId(this, core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueue4_Impl::Purge(this).into()
        }
        unsafe extern "system" fn IsOpen2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisopen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::IsOpen2(this) {
                Ok(ok__) => {
                    pisopen.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveByLookupIdAllowPeek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lookupid: core::mem::MaybeUninit<windows_core::VARIANT>, transaction: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantdestinationqueue: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantbody: *const core::mem::MaybeUninit<windows_core::VARIANT>, wantconnectortype: *const core::mem::MaybeUninit<windows_core::VARIANT>, ppmsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueue4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueue4_Impl::ReceiveByLookupIdAllowPeek(this, core::mem::transmute(&lookupid), core::mem::transmute_copy(&transaction), core::mem::transmute_copy(&wantdestinationqueue), core::mem::transmute_copy(&wantbody), core::mem::transmute_copy(&wantconnectortype)) {
                Ok(ok__) => {
                    ppmsg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Access: Access::<Identity, OFFSET>,
            ShareMode: ShareMode::<Identity, OFFSET>,
            QueueInfo: QueueInfo::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            IsOpen: IsOpen::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Receive_v1: Receive_v1::<Identity, OFFSET>,
            Peek_v1: Peek_v1::<Identity, OFFSET>,
            EnableNotification: EnableNotification::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            ReceiveCurrent_v1: ReceiveCurrent_v1::<Identity, OFFSET>,
            PeekNext_v1: PeekNext_v1::<Identity, OFFSET>,
            PeekCurrent_v1: PeekCurrent_v1::<Identity, OFFSET>,
            Receive: Receive::<Identity, OFFSET>,
            Peek: Peek::<Identity, OFFSET>,
            ReceiveCurrent: ReceiveCurrent::<Identity, OFFSET>,
            PeekNext: PeekNext::<Identity, OFFSET>,
            PeekCurrent: PeekCurrent::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Handle2: Handle2::<Identity, OFFSET>,
            ReceiveByLookupId: ReceiveByLookupId::<Identity, OFFSET>,
            ReceiveNextByLookupId: ReceiveNextByLookupId::<Identity, OFFSET>,
            ReceivePreviousByLookupId: ReceivePreviousByLookupId::<Identity, OFFSET>,
            ReceiveFirstByLookupId: ReceiveFirstByLookupId::<Identity, OFFSET>,
            ReceiveLastByLookupId: ReceiveLastByLookupId::<Identity, OFFSET>,
            PeekByLookupId: PeekByLookupId::<Identity, OFFSET>,
            PeekNextByLookupId: PeekNextByLookupId::<Identity, OFFSET>,
            PeekPreviousByLookupId: PeekPreviousByLookupId::<Identity, OFFSET>,
            PeekFirstByLookupId: PeekFirstByLookupId::<Identity, OFFSET>,
            PeekLastByLookupId: PeekLastByLookupId::<Identity, OFFSET>,
            Purge: Purge::<Identity, OFFSET>,
            IsOpen2: IsOpen2::<Identity, OFFSET>,
            ReceiveByLookupIdAllowPeek: ReceiveByLookupIdAllowPeek::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueue4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfo {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfo_Vtbl
    where
        Identity: IMSMQQueueInfo_Impl,
    {
        unsafe extern "system" fn QueueGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::QueueGuid(this) {
                Ok(ok__) => {
                    pbstrguidqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::ServiceTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidservicetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::PathName(this) {
                Ok(ok__) => {
                    pbstrpathname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::FormatName(this) {
                Ok(ok__) => {
                    pbstrformatname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::IsTransactional(this) {
                Ok(ok__) => {
                    pistransactional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::Quota(this) {
                Ok(ok__) => {
                    plquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::BasePriority(this) {
                Ok(ok__) => {
                    plbasepriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::CreateTime(this) {
                Ok(ok__) => {
                    pvarcreatetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::ModifyTime(this) {
                Ok(ok__) => {
                    pvarmodifytime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::Authenticate(this) {
                Ok(ok__) => {
                    plauthenticate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::JournalQuota(this) {
                Ok(ok__) => {
                    pljournalquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::IsWorldReadable(this) {
                Ok(ok__) => {
                    pisworldreadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const core::mem::MaybeUninit<windows_core::VARIANT>, isworldreadable: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                Ok(ok__) => {
                    ppq.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo_Impl::Update(this).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo2_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue2>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
    fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Security(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSecurity(&self, varsecurity: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfo2_Vtbl
    where
        Identity: IMSMQQueueInfo2_Impl,
    {
        unsafe extern "system" fn QueueGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::QueueGuid(this) {
                Ok(ok__) => {
                    pbstrguidqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::ServiceTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidservicetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::PathName(this) {
                Ok(ok__) => {
                    pbstrpathname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::FormatName(this) {
                Ok(ok__) => {
                    pbstrformatname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::IsTransactional(this) {
                Ok(ok__) => {
                    pistransactional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Quota(this) {
                Ok(ok__) => {
                    plquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::BasePriority(this) {
                Ok(ok__) => {
                    plbasepriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::CreateTime(this) {
                Ok(ok__) => {
                    pvarcreatetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::ModifyTime(this) {
                Ok(ok__) => {
                    pvarmodifytime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Authenticate(this) {
                Ok(ok__) => {
                    plauthenticate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::JournalQuota(this) {
                Ok(ok__) => {
                    pljournalquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::IsWorldReadable(this) {
                Ok(ok__) => {
                    pisworldreadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const core::mem::MaybeUninit<windows_core::VARIANT>, isworldreadable: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                Ok(ok__) => {
                    ppq.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::Update(this).into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathnamedns: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::PathNameDNS(this) {
                Ok(ok__) => {
                    pbstrpathnamedns.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsecurity: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo2_Impl::Security(this) {
                Ok(ok__) => {
                    pvarsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsecurity: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo2_Impl::SetSecurity(this, core::mem::transmute(&varsecurity)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo3_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue3>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
    fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Security(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSecurity(&self, varsecurity: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfo3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfo3_Vtbl
    where
        Identity: IMSMQQueueInfo3_Impl,
    {
        unsafe extern "system" fn QueueGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::QueueGuid(this) {
                Ok(ok__) => {
                    pbstrguidqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::ServiceTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidservicetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::PathName(this) {
                Ok(ok__) => {
                    pbstrpathname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::FormatName(this) {
                Ok(ok__) => {
                    pbstrformatname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::IsTransactional(this) {
                Ok(ok__) => {
                    pistransactional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Quota(this) {
                Ok(ok__) => {
                    plquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::BasePriority(this) {
                Ok(ok__) => {
                    plbasepriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::CreateTime(this) {
                Ok(ok__) => {
                    pvarcreatetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::ModifyTime(this) {
                Ok(ok__) => {
                    pvarmodifytime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Authenticate(this) {
                Ok(ok__) => {
                    plauthenticate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::JournalQuota(this) {
                Ok(ok__) => {
                    pljournalquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::IsWorldReadable(this) {
                Ok(ok__) => {
                    pisworldreadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const core::mem::MaybeUninit<windows_core::VARIANT>, isworldreadable: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                Ok(ok__) => {
                    ppq.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::Update(this).into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathnamedns: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::PathNameDNS(this) {
                Ok(ok__) => {
                    pbstrpathnamedns.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsecurity: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::Security(this) {
                Ok(ok__) => {
                    pvarsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsecurity: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetSecurity(this, core::mem::transmute(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::IsTransactional2(this) {
                Ok(ok__) => {
                    pistransactional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::IsWorldReadable2(this) {
                Ok(ok__) => {
                    pisworldreadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmulticastaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::MulticastAddress(this) {
                Ok(ok__) => {
                    pbstrmulticastaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmulticastaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo3_Impl::SetMulticastAddress(this, core::mem::transmute(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstradspath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo3_Impl::ADsPath(this) {
                Ok(ok__) => {
                    pbstradspath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfo4_Impl: Sized + super::Com::IDispatch_Impl {
    fn QueueGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServiceTypeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetServiceTypeGuid(&self, bstrguidservicetype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PathName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPathName(&self, bstrpathname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FormatName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFormatName(&self, bstrformatname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsTransactional(&self) -> windows_core::Result<i16>;
    fn PrivLevel(&self) -> windows_core::Result<i32>;
    fn SetPrivLevel(&self, lprivlevel: i32) -> windows_core::Result<()>;
    fn Journal(&self) -> windows_core::Result<i32>;
    fn SetJournal(&self, ljournal: i32) -> windows_core::Result<()>;
    fn Quota(&self) -> windows_core::Result<i32>;
    fn SetQuota(&self, lquota: i32) -> windows_core::Result<()>;
    fn BasePriority(&self) -> windows_core::Result<i32>;
    fn SetBasePriority(&self, lbasepriority: i32) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ModifyTime(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Authenticate(&self) -> windows_core::Result<i32>;
    fn SetAuthenticate(&self, lauthenticate: i32) -> windows_core::Result<()>;
    fn JournalQuota(&self) -> windows_core::Result<i32>;
    fn SetJournalQuota(&self, ljournalquota: i32) -> windows_core::Result<()>;
    fn IsWorldReadable(&self) -> windows_core::Result<i16>;
    fn Create(&self, istransactional: *const windows_core::VARIANT, isworldreadable: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Open(&self, access: i32, sharemode: i32) -> windows_core::Result<IMSMQQueue4>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Update(&self) -> windows_core::Result<()>;
    fn PathNameDNS(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn Security(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetSecurity(&self, varsecurity: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn IsTransactional2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorldReadable2(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MulticastAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMulticastAddress(&self, bstrmulticastaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfo4 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfo4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfo4_Vtbl
    where
        Identity: IMSMQQueueInfo4_Impl,
    {
        unsafe extern "system" fn QueueGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidqueue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::QueueGuid(this) {
                Ok(ok__) => {
                    pbstrguidqueue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrguidservicetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::ServiceTypeGuid(this) {
                Ok(ok__) => {
                    pbstrguidservicetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceTypeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrguidservicetype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetServiceTypeGuid(this, core::mem::transmute(&bstrguidservicetype)).into()
        }
        unsafe extern "system" fn Label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Label(this) {
                Ok(ok__) => {
                    pbstrlabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetLabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn PathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::PathName(this) {
                Ok(ok__) => {
                    pbstrpathname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPathName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpathname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetPathName(this, core::mem::transmute(&bstrpathname)).into()
        }
        unsafe extern "system" fn FormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrformatname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::FormatName(this) {
                Ok(ok__) => {
                    pbstrformatname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrformatname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetFormatName(this, core::mem::transmute(&bstrformatname)).into()
        }
        unsafe extern "system" fn IsTransactional<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::IsTransactional(this) {
                Ok(ok__) => {
                    pistransactional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprivlevel: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::PrivLevel(this) {
                Ok(ok__) => {
                    plprivlevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lprivlevel: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetPrivLevel(this, core::mem::transmute_copy(&lprivlevel)).into()
        }
        unsafe extern "system" fn Journal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournal: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Journal(this) {
                Ok(ok__) => {
                    pljournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournal: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetJournal(this, core::mem::transmute_copy(&ljournal)).into()
        }
        unsafe extern "system" fn Quota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Quota(this) {
                Ok(ok__) => {
                    plquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetQuota(this, core::mem::transmute_copy(&lquota)).into()
        }
        unsafe extern "system" fn BasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbasepriority: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::BasePriority(this) {
                Ok(ok__) => {
                    plbasepriority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbasepriority: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetBasePriority(this, core::mem::transmute_copy(&lbasepriority)).into()
        }
        unsafe extern "system" fn CreateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcreatetime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::CreateTime(this) {
                Ok(ok__) => {
                    pvarcreatetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodifytime: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::ModifyTime(this) {
                Ok(ok__) => {
                    pvarmodifytime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Authenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plauthenticate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Authenticate(this) {
                Ok(ok__) => {
                    plauthenticate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lauthenticate: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetAuthenticate(this, core::mem::transmute_copy(&lauthenticate)).into()
        }
        unsafe extern "system" fn JournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalquota: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::JournalQuota(this) {
                Ok(ok__) => {
                    pljournalquota.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJournalQuota<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ljournalquota: i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetJournalQuota(this, core::mem::transmute_copy(&ljournalquota)).into()
        }
        unsafe extern "system" fn IsWorldReadable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut i16) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::IsWorldReadable(this) {
                Ok(ok__) => {
                    pisworldreadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, istransactional: *const core::mem::MaybeUninit<windows_core::VARIANT>, isworldreadable: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::Create(this, core::mem::transmute_copy(&istransactional), core::mem::transmute_copy(&isworldreadable)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, access: i32, sharemode: i32, ppq: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Open(this, core::mem::transmute_copy(&access), core::mem::transmute_copy(&sharemode)) {
                Ok(ok__) => {
                    ppq.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::Update(this).into()
        }
        unsafe extern "system" fn PathNameDNS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpathnamedns: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::PathNameDNS(this) {
                Ok(ok__) => {
                    pbstrpathnamedns.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarsecurity: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::Security(this) {
                Ok(ok__) => {
                    pvarsecurity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varsecurity: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetSecurity(this, core::mem::transmute(&varsecurity)).into()
        }
        unsafe extern "system" fn IsTransactional2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistransactional: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::IsTransactional2(this) {
                Ok(ok__) => {
                    pistransactional.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWorldReadable2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisworldreadable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::IsWorldReadable2(this) {
                Ok(ok__) => {
                    pisworldreadable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MulticastAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmulticastaddress: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::MulticastAddress(this) {
                Ok(ok__) => {
                    pbstrmulticastaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmulticastaddress: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfo4_Impl::SetMulticastAddress(this, core::mem::transmute(&bstrmulticastaddress)).into()
        }
        unsafe extern "system" fn ADsPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstradspath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfo4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfo4_Impl::ADsPath(this) {
                Ok(ok__) => {
                    pbstradspath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            QueueGuid: QueueGuid::<Identity, OFFSET>,
            ServiceTypeGuid: ServiceTypeGuid::<Identity, OFFSET>,
            SetServiceTypeGuid: SetServiceTypeGuid::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            PathName: PathName::<Identity, OFFSET>,
            SetPathName: SetPathName::<Identity, OFFSET>,
            FormatName: FormatName::<Identity, OFFSET>,
            SetFormatName: SetFormatName::<Identity, OFFSET>,
            IsTransactional: IsTransactional::<Identity, OFFSET>,
            PrivLevel: PrivLevel::<Identity, OFFSET>,
            SetPrivLevel: SetPrivLevel::<Identity, OFFSET>,
            Journal: Journal::<Identity, OFFSET>,
            SetJournal: SetJournal::<Identity, OFFSET>,
            Quota: Quota::<Identity, OFFSET>,
            SetQuota: SetQuota::<Identity, OFFSET>,
            BasePriority: BasePriority::<Identity, OFFSET>,
            SetBasePriority: SetBasePriority::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            ModifyTime: ModifyTime::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            SetAuthenticate: SetAuthenticate::<Identity, OFFSET>,
            JournalQuota: JournalQuota::<Identity, OFFSET>,
            SetJournalQuota: SetJournalQuota::<Identity, OFFSET>,
            IsWorldReadable: IsWorldReadable::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            PathNameDNS: PathNameDNS::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            IsTransactional2: IsTransactional2::<Identity, OFFSET>,
            IsWorldReadable2: IsWorldReadable2::<Identity, OFFSET>,
            MulticastAddress: MulticastAddress::<Identity, OFFSET>,
            SetMulticastAddress: SetMulticastAddress::<Identity, OFFSET>,
            ADsPath: ADsPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfo4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfos {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfos_Vtbl
    where
        Identity: IMSMQQueueInfos_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos_Impl::Next(this) {
                Ok(ok__) => {
                    ppqinfonext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Reset: Reset::<Identity, OFFSET>, Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos2_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfos2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfos2_Vtbl
    where
        Identity: IMSMQQueueInfos2_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfos2_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos2_Impl::Next(this) {
                Ok(ok__) => {
                    ppqinfonext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos3_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfos3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfos3_Vtbl
    where
        Identity: IMSMQQueueInfos3_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfos3_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos3_Impl::Next(this) {
                Ok(ok__) => {
                    ppqinfonext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueInfos4_Impl: Sized + super::Com::IDispatch_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<IMSMQQueueInfo4>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueInfos4 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueInfos4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueInfos4_Vtbl
    where
        Identity: IMSMQQueueInfos4_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQQueueInfos4_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqinfonext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos4_Impl::Next(this) {
                Ok(ok__) => {
                    ppqinfonext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueInfos4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueInfos4_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueInfos4 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQQueueManagement_Impl: Sized + IMSMQManagement_Impl {
    fn JournalMessageCount(&self) -> windows_core::Result<i32>;
    fn BytesInJournal(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn EodGetReceiveInfo(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQQueueManagement {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQQueueManagement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQQueueManagement_Vtbl
    where
        Identity: IMSMQQueueManagement_Impl,
    {
        unsafe extern "system" fn JournalMessageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pljournalmessagecount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueManagement_Impl::JournalMessageCount(this) {
                Ok(ok__) => {
                    pljournalmessagecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesInJournal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytesinjournal: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueManagement_Impl::BytesInJournal(this) {
                Ok(ok__) => {
                    pvbytesinjournal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EodGetReceiveInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvcollection: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQQueueManagement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQQueueManagement_Impl::EodGetReceiveInfo(this) {
                Ok(ok__) => {
                    pvcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMSMQManagement_Vtbl::new::<Identity, OFFSET>(),
            JournalMessageCount: JournalMessageCount::<Identity, OFFSET>,
            BytesInJournal: BytesInJournal::<Identity, OFFSET>,
            EodGetReceiveInfo: EodGetReceiveInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQQueueManagement as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQManagement as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Transaction(&self) -> windows_core::Result<i32>;
    fn Commit(&self, fretaining: *const windows_core::VARIANT, grftc: *const windows_core::VARIANT, grfrm: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn Abort(&self, fretaining: *const windows_core::VARIANT, fasync: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQTransaction {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQTransaction_Vtbl
    where
        Identity: IMSMQTransaction_Impl,
    {
        unsafe extern "system" fn Transaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltransaction: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMSMQTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransaction_Impl::Transaction(this) {
                Ok(ok__) => {
                    pltransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: *const core::mem::MaybeUninit<windows_core::VARIANT>, grftc: *const core::mem::MaybeUninit<windows_core::VARIANT>, grfrm: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQTransaction_Impl::Commit(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&grftc), core::mem::transmute_copy(&grfrm)).into()
        }
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fretaining: *const core::mem::MaybeUninit<windows_core::VARIANT>, fasync: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQTransaction_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQTransaction_Impl::Abort(this, core::mem::transmute_copy(&fretaining), core::mem::transmute_copy(&fasync)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Transaction: Transaction::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransaction as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction2_Impl: Sized + IMSMQTransaction_Impl {
    fn InitNew(&self, vartransaction: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQTransaction2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQTransaction2_Vtbl
    where
        Identity: IMSMQTransaction2_Impl,
    {
        unsafe extern "system" fn InitNew<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartransaction: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQTransaction2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMSMQTransaction2_Impl::InitNew(this, core::mem::transmute(&vartransaction)).into()
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQTransaction2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransaction2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IMSMQTransaction_Vtbl::new::<Identity, OFFSET>(), InitNew: InitNew::<Identity, OFFSET>, Properties: Properties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransaction2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQTransaction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransaction3_Impl: Sized + IMSMQTransaction2_Impl {
    fn ITransaction(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQTransaction3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransaction3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQTransaction3_Vtbl
    where
        Identity: IMSMQTransaction3_Impl,
    {
        unsafe extern "system" fn ITransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaritransaction: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IMSMQTransaction3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransaction3_Impl::ITransaction(this) {
                Ok(ok__) => {
                    pvaritransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IMSMQTransaction2_Vtbl::new::<Identity, OFFSET>(), ITransaction: ITransaction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransaction3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMSMQTransaction as windows_core::Interface>::IID || iid == &<IMSMQTransaction2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQTransactionDispenser {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQTransactionDispenser_Vtbl
    where
        Identity: IMSMQTransactionDispenser_Impl,
    {
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQTransactionDispenser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransactionDispenser_Impl::BeginTransaction(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), BeginTransaction: BeginTransaction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser2_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction2>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQTransactionDispenser2 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQTransactionDispenser2_Vtbl
    where
        Identity: IMSMQTransactionDispenser2_Impl,
    {
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQTransactionDispenser2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransactionDispenser2_Impl::BeginTransaction(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQTransactionDispenser2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransactionDispenser2_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSMQTransactionDispenser3_Impl: Sized + super::Com::IDispatch_Impl {
    fn BeginTransaction(&self) -> windows_core::Result<IMSMQTransaction3>;
    fn Properties(&self) -> windows_core::Result<super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMSMQTransactionDispenser3 {}
#[cfg(feature = "Win32_System_Com")]
impl IMSMQTransactionDispenser3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMSMQTransactionDispenser3_Vtbl
    where
        Identity: IMSMQTransactionDispenser3_Impl,
    {
        unsafe extern "system" fn BeginTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptransaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQTransactionDispenser3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransactionDispenser3_Impl::BeginTransaction(this) {
                Ok(ok__) => {
                    ptransaction.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMSMQTransactionDispenser3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMSMQTransactionDispenser3_Impl::Properties(this) {
                Ok(ok__) => {
                    ppcolproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BeginTransaction: BeginTransaction::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMSMQTransactionDispenser3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _DMSMQEventEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _DMSMQEventEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _DMSMQEventEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _DMSMQEventEvents_Vtbl
    where
        Identity: _DMSMQEventEvents_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_DMSMQEventEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
