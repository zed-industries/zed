pub trait IEnumVdsObject_Impl: Sized {
    fn Next(&self, celt: u32, ppobjectarray: *mut Option<windows_core::IUnknown>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumVdsObject>;
}
impl windows_core::RuntimeName for IEnumVdsObject {}
impl IEnumVdsObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumVdsObject_Vtbl
    where
        Identity: IEnumVdsObject_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumVdsObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumVdsObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppobjectarray), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumVdsObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumVdsObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumVdsObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumVdsObject_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumVdsObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumVdsObject_Impl::Clone(this) {
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
        iid == &<IEnumVdsObject as windows_core::Interface>::IID
    }
}
pub trait IVdsAdmin_Impl: Sized {
    fn RegisterProvider(&self, providerid: &windows_core::GUID, providerclsid: &windows_core::GUID, pwszname: &windows_core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: &windows_core::PCWSTR, pwszversion: &windows_core::PCWSTR, guidversionid: &windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterProvider(&self, providerid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsAdmin {}
impl IVdsAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsAdmin_Vtbl
    where
        Identity: IVdsAdmin_Impl,
    {
        unsafe extern "system" fn RegisterProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID, providerclsid: windows_core::GUID, pwszname: windows_core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: windows_core::PCWSTR, pwszversion: windows_core::PCWSTR, guidversionid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdmin_Impl::RegisterProvider(this, core::mem::transmute(&providerid), core::mem::transmute(&providerclsid), core::mem::transmute(&pwszname), core::mem::transmute_copy(&r#type), core::mem::transmute(&pwszmachinename), core::mem::transmute(&pwszversion), core::mem::transmute(&guidversionid)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdmin_Impl::UnregisterProvider(this, core::mem::transmute(&providerid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProvider: RegisterProvider::<Identity, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdmin as windows_core::Interface>::IID
    }
}
pub trait IVdsAdvancedDisk_Impl: Sized {
    fn GetPartitionProperties(&self, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> windows_core::Result<()>;
    fn QueryPartitions(&self, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> windows_core::Result<()>;
    fn CreatePartition(&self, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS) -> windows_core::Result<IVdsAsync>;
    fn DeletePartition(&self, ulloffset: u64, bforce: super::super::Foundation::BOOL, bforceprotected: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ChangeAttributes(&self, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> windows_core::Result<()>;
    fn AssignDriveLetter(&self, ulloffset: u64, wcletter: u16) -> windows_core::Result<()>;
    fn DeleteDriveLetter(&self, ulloffset: u64, wcletter: u16) -> windows_core::Result<()>;
    fn GetDriveLetter(&self, ulloffset: u64, pwcletter: windows_core::PWSTR) -> windows_core::Result<()>;
    fn FormatPartition(&self, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: &windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> windows_core::Result<IVdsAsync>;
    fn Clean(&self, bforce: super::super::Foundation::BOOL, bforceoem: super::super::Foundation::BOOL, bfullclean: super::super::Foundation::BOOL) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsAdvancedDisk {}
impl IVdsAdvancedDisk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsAdvancedDisk_Vtbl
    where
        Identity: IVdsAdvancedDisk_Impl,
    {
        unsafe extern "system" fn GetPartitionProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::GetPartitionProperties(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&ppartitionprop)).into()
        }
        unsafe extern "system" fn QueryPartitions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::QueryPartitions(this, core::mem::transmute_copy(&pppartitionproparray), core::mem::transmute_copy(&plnumberofpartitions)).into()
        }
        unsafe extern "system" fn CreatePartition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsAdvancedDisk_Impl::CreatePartition(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&ullsize), core::mem::transmute_copy(&para)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePartition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, bforce: super::super::Foundation::BOOL, bforceprotected: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::DeletePartition(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bforceprotected)).into()
        }
        unsafe extern "system" fn ChangeAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::ChangeAttributes(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&para)).into()
        }
        unsafe extern "system" fn AssignDriveLetter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, wcletter: u16) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::AssignDriveLetter(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&wcletter)).into()
        }
        unsafe extern "system" fn DeleteDriveLetter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, wcletter: u16) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::DeleteDriveLetter(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&wcletter)).into()
        }
        unsafe extern "system" fn GetDriveLetter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, pwcletter: windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk_Impl::GetDriveLetter(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&pwcletter)).into()
        }
        unsafe extern "system" fn FormatPartition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsAdvancedDisk_Impl::FormatPartition(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&r#type), core::mem::transmute(&pwszlabel), core::mem::transmute_copy(&dwunitallocationsize), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bquickformat), core::mem::transmute_copy(&benablecompression)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clean<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforce: super::super::Foundation::BOOL, bforceoem: super::super::Foundation::BOOL, bfullclean: super::super::Foundation::BOOL, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsAdvancedDisk_Impl::Clean(this, core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bforceoem), core::mem::transmute_copy(&bfullclean)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionProperties: GetPartitionProperties::<Identity, OFFSET>,
            QueryPartitions: QueryPartitions::<Identity, OFFSET>,
            CreatePartition: CreatePartition::<Identity, OFFSET>,
            DeletePartition: DeletePartition::<Identity, OFFSET>,
            ChangeAttributes: ChangeAttributes::<Identity, OFFSET>,
            AssignDriveLetter: AssignDriveLetter::<Identity, OFFSET>,
            DeleteDriveLetter: DeleteDriveLetter::<Identity, OFFSET>,
            GetDriveLetter: GetDriveLetter::<Identity, OFFSET>,
            FormatPartition: FormatPartition::<Identity, OFFSET>,
            Clean: Clean::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdvancedDisk as windows_core::Interface>::IID
    }
}
pub trait IVdsAdvancedDisk2_Impl: Sized {
    fn ChangePartitionType(&self, ulloffset: u64, bforce: super::super::Foundation::BOOL, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsAdvancedDisk2 {}
impl IVdsAdvancedDisk2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsAdvancedDisk2_Vtbl
    where
        Identity: IVdsAdvancedDisk2_Impl,
    {
        unsafe extern "system" fn ChangePartitionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, bforce: super::super::Foundation::BOOL, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk2_Impl::ChangePartitionType(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&para)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ChangePartitionType: ChangePartitionType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdvancedDisk2 as windows_core::Interface>::IID
    }
}
pub trait IVdsAdvancedDisk3_Impl: Sized {
    fn GetProperties(&self, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> windows_core::Result<()>;
    fn GetUniqueId(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IVdsAdvancedDisk3 {}
impl IVdsAdvancedDisk3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsAdvancedDisk3_Vtbl
    where
        Identity: IVdsAdvancedDisk3_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdvancedDisk3_Impl::GetProperties(this, core::mem::transmute_copy(&padvdiskprop)).into()
        }
        unsafe extern "system" fn GetUniqueId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsAdvancedDisk3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsAdvancedDisk3_Impl::GetUniqueId(this) {
                Ok(ok__) => {
                    ppwszid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetUniqueId: GetUniqueId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdvancedDisk3 as windows_core::Interface>::IID
    }
}
pub trait IVdsAdviseSink_Impl: Sized {
    fn OnNotify(&self, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsAdviseSink {}
impl IVdsAdviseSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsAdviseSink_Vtbl
    where
        Identity: IVdsAdviseSink_Impl,
    {
        unsafe extern "system" fn OnNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> windows_core::HRESULT
        where
            Identity: IVdsAdviseSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAdviseSink_Impl::OnNotify(this, core::mem::transmute_copy(&lnumberofnotifications), core::mem::transmute_copy(&pnotificationarray)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAdviseSink as windows_core::Interface>::IID
    }
}
pub trait IVdsAsync_Impl: Sized {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, phrresult: *mut windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> windows_core::Result<()>;
    fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, pulpercentcompleted: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsAsync {}
impl IVdsAsync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsAsync_Vtbl
    where
        Identity: IVdsAsync_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAsync_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> windows_core::HRESULT
        where
            Identity: IVdsAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAsync_Impl::Wait(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&pasyncout)).into()
        }
        unsafe extern "system" fn QueryStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, pulpercentcompleted: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsAsync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsAsync_Impl::QueryStatus(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&pulpercentcompleted)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            QueryStatus: QueryStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsAsync as windows_core::Interface>::IID
    }
}
pub trait IVdsController_Impl: Sized {
    fn GetProperties(&self, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> windows_core::Result<()>;
    fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem>;
    fn GetPortProperties(&self, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> windows_core::Result<()>;
    fn FlushCache(&self) -> windows_core::Result<()>;
    fn InvalidateCache(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn QueryAssociatedLuns(&self) -> windows_core::Result<IEnumVdsObject>;
    fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsController {}
impl IVdsController_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsController_Vtbl
    where
        Identity: IVdsController_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsController_Impl::GetProperties(this, core::mem::transmute_copy(&pcontrollerprop)).into()
        }
        unsafe extern "system" fn GetSubSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubsystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsController_Impl::GetSubSystem(this) {
                Ok(ok__) => {
                    ppsubsystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsController_Impl::GetPortProperties(this, core::mem::transmute_copy(&sportnumber), core::mem::transmute_copy(&pportprop)).into()
        }
        unsafe extern "system" fn FlushCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsController_Impl::FlushCache(this).into()
        }
        unsafe extern "system" fn InvalidateCache<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsController_Impl::InvalidateCache(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsController_Impl::Reset(this).into()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsController_Impl::QueryAssociatedLuns(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsController_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsController_Impl::SetStatus(this, core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, OFFSET>,
            GetPortProperties: GetPortProperties::<Identity, OFFSET>,
            FlushCache: FlushCache::<Identity, OFFSET>,
            InvalidateCache: InvalidateCache::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsController as windows_core::Interface>::IID
    }
}
pub trait IVdsControllerControllerPort_Impl: Sized {
    fn QueryControllerPorts(&self) -> windows_core::Result<IEnumVdsObject>;
}
impl windows_core::RuntimeName for IVdsControllerControllerPort {}
impl IVdsControllerControllerPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsControllerControllerPort_Vtbl
    where
        Identity: IVdsControllerControllerPort_Impl,
    {
        unsafe extern "system" fn QueryControllerPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsControllerControllerPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsControllerControllerPort_Impl::QueryControllerPorts(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryControllerPorts: QueryControllerPorts::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsControllerControllerPort as windows_core::Interface>::IID
    }
}
pub trait IVdsControllerPort_Impl: Sized {
    fn GetProperties(&self, pportprop: *mut VDS_PORT_PROP) -> windows_core::Result<()>;
    fn GetController(&self) -> windows_core::Result<IVdsController>;
    fn QueryAssociatedLuns(&self) -> windows_core::Result<IEnumVdsObject>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_PORT_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsControllerPort {}
impl IVdsControllerPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsControllerPort_Vtbl
    where
        Identity: IVdsControllerPort_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsControllerPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsControllerPort_Impl::GetProperties(this, core::mem::transmute_copy(&pportprop)).into()
        }
        unsafe extern "system" fn GetController<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontroller: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsControllerPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsControllerPort_Impl::GetController(this) {
                Ok(ok__) => {
                    ppcontroller.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsControllerPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsControllerPort_Impl::QueryAssociatedLuns(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsControllerPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsControllerPort_Impl::Reset(this).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_PORT_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsControllerPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsControllerPort_Impl::SetStatus(this, core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetController: GetController::<Identity, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsControllerPort as windows_core::Interface>::IID
    }
}
pub trait IVdsCreatePartitionEx_Impl: Sized {
    fn CreatePartitionEx(&self, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsCreatePartitionEx {}
impl IVdsCreatePartitionEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsCreatePartitionEx_Vtbl
    where
        Identity: IVdsCreatePartitionEx_Impl,
    {
        unsafe extern "system" fn CreatePartitionEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsCreatePartitionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsCreatePartitionEx_Impl::CreatePartitionEx(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&ullsize), core::mem::transmute_copy(&ulalign), core::mem::transmute_copy(&para)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreatePartitionEx: CreatePartitionEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsCreatePartitionEx as windows_core::Interface>::IID
    }
}
pub trait IVdsDisk_Impl: Sized {
    fn GetProperties(&self, pdiskproperties: *mut VDS_DISK_PROP) -> windows_core::Result<()>;
    fn GetPack(&self) -> windows_core::Result<IVdsPack>;
    fn GetIdentificationData(&self, pluninfo: *mut VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()>;
    fn ConvertStyle(&self, newstyle: VDS_PARTITION_STYLE) -> windows_core::Result<()>;
    fn SetFlags(&self, ulflags: u32) -> windows_core::Result<()>;
    fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsDisk {}
impl IVdsDisk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDisk_Vtbl
    where
        Identity: IVdsDisk_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiskproperties: *mut VDS_DISK_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk_Impl::GetProperties(this, core::mem::transmute_copy(&pdiskproperties)).into()
        }
        unsafe extern "system" fn GetPack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppack: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsDisk_Impl::GetPack(this) {
                Ok(ok__) => {
                    pppack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentificationData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk_Impl::GetIdentificationData(this, core::mem::transmute_copy(&pluninfo)).into()
        }
        unsafe extern "system" fn QueryExtents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk_Impl::QueryExtents(this, core::mem::transmute_copy(&ppextentarray), core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn ConvertStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstyle: VDS_PARTITION_STYLE) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk_Impl::ConvertStyle(this, core::mem::transmute_copy(&newstyle)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk_Impl::SetFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk_Impl::ClearFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetPack: GetPack::<Identity, OFFSET>,
            GetIdentificationData: GetIdentificationData::<Identity, OFFSET>,
            QueryExtents: QueryExtents::<Identity, OFFSET>,
            ConvertStyle: ConvertStyle::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            ClearFlags: ClearFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDisk as windows_core::Interface>::IID
    }
}
pub trait IVdsDisk2_Impl: Sized {
    fn SetSANMode(&self, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsDisk2 {}
impl IVdsDisk2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDisk2_Vtbl
    where
        Identity: IVdsDisk2_Impl,
    {
        unsafe extern "system" fn SetSANMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsDisk2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk2_Impl::SetSANMode(this, core::mem::transmute_copy(&benable)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSANMode: SetSANMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDisk2 as windows_core::Interface>::IID
    }
}
pub trait IVdsDisk3_Impl: Sized {
    fn GetProperties2(&self, pdiskproperties: *mut VDS_DISK_PROP2) -> windows_core::Result<()>;
    fn QueryFreeExtents(&self, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsDisk3 {}
impl IVdsDisk3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDisk3_Vtbl
    where
        Identity: IVdsDisk3_Impl,
    {
        unsafe extern "system" fn GetProperties2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiskproperties: *mut VDS_DISK_PROP2) -> windows_core::HRESULT
        where
            Identity: IVdsDisk3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk3_Impl::GetProperties2(this, core::mem::transmute_copy(&pdiskproperties)).into()
        }
        unsafe extern "system" fn QueryFreeExtents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsDisk3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDisk3_Impl::QueryFreeExtents(this, core::mem::transmute_copy(&ulalign), core::mem::transmute_copy(&ppfreeextentarray), core::mem::transmute_copy(&plnumberoffreeextents)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties2: GetProperties2::<Identity, OFFSET>,
            QueryFreeExtents: QueryFreeExtents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDisk3 as windows_core::Interface>::IID
    }
}
pub trait IVdsDiskOnline_Impl: Sized {
    fn Online(&self) -> windows_core::Result<()>;
    fn Offline(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsDiskOnline {}
impl IVdsDiskOnline_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDiskOnline_Vtbl
    where
        Identity: IVdsDiskOnline_Impl,
    {
        unsafe extern "system" fn Online<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsDiskOnline_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDiskOnline_Impl::Online(this).into()
        }
        unsafe extern "system" fn Offline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsDiskOnline_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDiskOnline_Impl::Offline(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Online: Online::<Identity, OFFSET>, Offline: Offline::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDiskOnline as windows_core::Interface>::IID
    }
}
pub trait IVdsDiskPartitionMF_Impl: Sized {
    fn GetPartitionFileSystemProperties(&self, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> windows_core::Result<()>;
    fn GetPartitionFileSystemTypeName(&self, ulloffset: u64) -> windows_core::Result<windows_core::PWSTR>;
    fn QueryPartitionFileSystemFormatSupport(&self, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> windows_core::Result<()>;
    fn FormatPartitionEx(&self, ulloffset: u64, pwszfilesystemtypename: &windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsDiskPartitionMF {}
impl IVdsDiskPartitionMF_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDiskPartitionMF_Vtbl
    where
        Identity: IVdsDiskPartitionMF_Impl,
    {
        unsafe extern "system" fn GetPartitionFileSystemProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsDiskPartitionMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDiskPartitionMF_Impl::GetPartitionFileSystemProperties(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&pfilesystemprop)).into()
        }
        unsafe extern "system" fn GetPartitionFileSystemTypeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, ppwszfilesystemtypename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsDiskPartitionMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsDiskPartitionMF_Impl::GetPartitionFileSystemTypeName(this, core::mem::transmute_copy(&ulloffset)) {
                Ok(ok__) => {
                    ppwszfilesystemtypename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPartitionFileSystemFormatSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsDiskPartitionMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDiskPartitionMF_Impl::QueryPartitionFileSystemFormatSupport(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute_copy(&ppfilesystemsupportprops), core::mem::transmute_copy(&plnumberoffilesystems)).into()
        }
        unsafe extern "system" fn FormatPartitionEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, pwszfilesystemtypename: windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsDiskPartitionMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsDiskPartitionMF_Impl::FormatPartitionEx(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute(&pwszfilesystemtypename), core::mem::transmute_copy(&usfilesystemrevision), core::mem::transmute_copy(&uldesiredunitallocationsize), core::mem::transmute(&pwszlabel), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bquickformat), core::mem::transmute_copy(&benablecompression)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPartitionFileSystemProperties: GetPartitionFileSystemProperties::<Identity, OFFSET>,
            GetPartitionFileSystemTypeName: GetPartitionFileSystemTypeName::<Identity, OFFSET>,
            QueryPartitionFileSystemFormatSupport: QueryPartitionFileSystemFormatSupport::<Identity, OFFSET>,
            FormatPartitionEx: FormatPartitionEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDiskPartitionMF as windows_core::Interface>::IID
    }
}
pub trait IVdsDiskPartitionMF2_Impl: Sized {
    fn FormatPartitionEx2(&self, ulloffset: u64, pwszfilesystemtypename: &windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &windows_core::PCWSTR, options: u32) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsDiskPartitionMF2 {}
impl IVdsDiskPartitionMF2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDiskPartitionMF2_Vtbl
    where
        Identity: IVdsDiskPartitionMF2_Impl,
    {
        unsafe extern "system" fn FormatPartitionEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffset: u64, pwszfilesystemtypename: windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: windows_core::PCWSTR, options: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsDiskPartitionMF2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsDiskPartitionMF2_Impl::FormatPartitionEx2(this, core::mem::transmute_copy(&ulloffset), core::mem::transmute(&pwszfilesystemtypename), core::mem::transmute_copy(&usfilesystemrevision), core::mem::transmute_copy(&uldesiredunitallocationsize), core::mem::transmute(&pwszlabel), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FormatPartitionEx2: FormatPartitionEx2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDiskPartitionMF2 as windows_core::Interface>::IID
    }
}
pub trait IVdsDrive_Impl: Sized {
    fn GetProperties(&self, pdriveprop: *mut VDS_DRIVE_PROP) -> windows_core::Result<()>;
    fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()>;
    fn SetFlags(&self, ulflags: u32) -> windows_core::Result<()>;
    fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsDrive {}
impl IVdsDrive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDrive_Vtbl
    where
        Identity: IVdsDrive_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsDrive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDrive_Impl::GetProperties(this, core::mem::transmute_copy(&pdriveprop)).into()
        }
        unsafe extern "system" fn GetSubSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubsystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsDrive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsDrive_Impl::GetSubSystem(this) {
                Ok(ok__) => {
                    ppsubsystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsDrive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDrive_Impl::QueryExtents(this, core::mem::transmute_copy(&ppextentarray), core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsDrive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDrive_Impl::SetFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsDrive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDrive_Impl::ClearFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_DRIVE_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsDrive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDrive_Impl::SetStatus(this, core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, OFFSET>,
            QueryExtents: QueryExtents::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            ClearFlags: ClearFlags::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDrive as windows_core::Interface>::IID
    }
}
pub trait IVdsDrive2_Impl: Sized {
    fn GetProperties2(&self, pdriveprop2: *mut VDS_DRIVE_PROP2) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsDrive2 {}
impl IVdsDrive2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsDrive2_Vtbl
    where
        Identity: IVdsDrive2_Impl,
    {
        unsafe extern "system" fn GetProperties2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> windows_core::HRESULT
        where
            Identity: IVdsDrive2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsDrive2_Impl::GetProperties2(this, core::mem::transmute_copy(&pdriveprop2)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperties2: GetProperties2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsDrive2 as windows_core::Interface>::IID
    }
}
pub trait IVdsHbaPort_Impl: Sized {
    fn GetProperties(&self, phbaportprop: *mut VDS_HBAPORT_PROP) -> windows_core::Result<()>;
    fn SetAllPathStatuses(&self, status: VDS_PATH_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsHbaPort {}
impl IVdsHbaPort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHbaPort_Vtbl
    where
        Identity: IVdsHbaPort_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phbaportprop: *mut VDS_HBAPORT_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsHbaPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsHbaPort_Impl::GetProperties(this, core::mem::transmute_copy(&phbaportprop)).into()
        }
        unsafe extern "system" fn SetAllPathStatuses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_PATH_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsHbaPort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsHbaPort_Impl::SetAllPathStatuses(this, core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            SetAllPathStatuses: SetAllPathStatuses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHbaPort as windows_core::Interface>::IID
    }
}
pub trait IVdsHwProvider_Impl: Sized {
    fn QuerySubSystems(&self) -> windows_core::Result<IEnumVdsObject>;
    fn Reenumerate(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsHwProvider {}
impl IVdsHwProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHwProvider_Vtbl
    where
        Identity: IVdsHwProvider_Impl,
    {
        unsafe extern "system" fn QuerySubSystems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsHwProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProvider_Impl::QuerySubSystems(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsHwProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsHwProvider_Impl::Reenumerate(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsHwProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsHwProvider_Impl::Refresh(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QuerySubSystems: QuerySubSystems::<Identity, OFFSET>,
            Reenumerate: Reenumerate::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProvider as windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderPrivate_Impl: Sized {
    fn QueryIfCreatedLun(&self, pwszdevicepath: &windows_core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IVdsHwProviderPrivate {}
impl IVdsHwProviderPrivate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHwProviderPrivate_Vtbl
    where
        Identity: IVdsHwProviderPrivate_Impl,
    {
        unsafe extern "system" fn QueryIfCreatedLun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdevicepath: windows_core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderPrivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProviderPrivate_Impl::QueryIfCreatedLun(this, core::mem::transmute(&pwszdevicepath), core::mem::transmute_copy(&pvdsluninformation)) {
                Ok(ok__) => {
                    plunid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryIfCreatedLun: QueryIfCreatedLun::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivate as windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderPrivateMpio_Impl: Sized {
    fn SetAllPathStatusesFromHbaPort(&self, hbaportprop: &VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsHwProviderPrivateMpio {}
impl IVdsHwProviderPrivateMpio_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHwProviderPrivateMpio_Vtbl
    where
        Identity: IVdsHwProviderPrivateMpio_Impl,
    {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderPrivateMpio_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsHwProviderPrivateMpio_Impl::SetAllPathStatusesFromHbaPort(this, core::mem::transmute(&hbaportprop), core::mem::transmute_copy(&status)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetAllPathStatusesFromHbaPort: SetAllPathStatusesFromHbaPort::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivateMpio as windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderStoragePools_Impl: Sized {
    fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> windows_core::Result<IEnumVdsObject>;
    fn CreateLunInStoragePool(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: &windows_core::GUID, pwszunmaskinglist: &windows_core::PCWSTR, phints2: *const VDS_HINTS2) -> windows_core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSizeInStoragePool(&self, r#type: VDS_LUN_TYPE, storagepoolid: &windows_core::GUID, phints2: *const VDS_HINTS2) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IVdsHwProviderStoragePools {}
impl IVdsHwProviderStoragePools_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHwProviderStoragePools_Vtbl
    where
        Identity: IVdsHwProviderStoragePools_Impl,
    {
        unsafe extern "system" fn QueryStoragePools<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderStoragePools_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProviderStoragePools_Impl::QueryStoragePools(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&ullremainingfreespace), core::mem::transmute_copy(&ppoolattributes)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLunInStoragePool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: windows_core::GUID, pwszunmaskinglist: windows_core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderStoragePools_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProviderStoragePools_Impl::CreateLunInStoragePool(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ullsizeinbytes), core::mem::transmute(&storagepoolid), core::mem::transmute(&pwszunmaskinglist), core::mem::transmute_copy(&phints2)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: windows_core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderStoragePools_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProviderStoragePools_Impl::QueryMaxLunCreateSizeInStoragePool(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&storagepoolid), core::mem::transmute_copy(&phints2)) {
                Ok(ok__) => {
                    pullmaxlunsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryStoragePools: QueryStoragePools::<Identity, OFFSET>,
            CreateLunInStoragePool: CreateLunInStoragePool::<Identity, OFFSET>,
            QueryMaxLunCreateSizeInStoragePool: QueryMaxLunCreateSizeInStoragePool::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderStoragePools as windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderType_Impl: Sized {
    fn GetProviderType(&self) -> windows_core::Result<VDS_HWPROVIDER_TYPE>;
}
impl windows_core::RuntimeName for IVdsHwProviderType {}
impl IVdsHwProviderType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHwProviderType_Vtbl
    where
        Identity: IVdsHwProviderType_Impl,
    {
        unsafe extern "system" fn GetProviderType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProviderType_Impl::GetProviderType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProviderType: GetProviderType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderType as windows_core::Interface>::IID
    }
}
pub trait IVdsHwProviderType2_Impl: Sized {
    fn GetProviderType2(&self) -> windows_core::Result<VDS_HWPROVIDER_TYPE>;
}
impl windows_core::RuntimeName for IVdsHwProviderType2 {}
impl IVdsHwProviderType2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsHwProviderType2_Vtbl
    where
        Identity: IVdsHwProviderType2_Impl,
    {
        unsafe extern "system" fn GetProviderType2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> windows_core::HRESULT
        where
            Identity: IVdsHwProviderType2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsHwProviderType2_Impl::GetProviderType2(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProviderType2: GetProviderType2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsHwProviderType2 as windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiInitiatorAdapter_Impl: Sized {
    fn GetProperties(&self, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> windows_core::Result<()>;
    fn QueryInitiatorPortals(&self) -> windows_core::Result<IEnumVdsObject>;
    fn LoginToTarget(&self, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: &windows_core::GUID, targetportalid: &windows_core::GUID, initiatorportalid: &windows_core::GUID, ulloginflags: u32, bheaderdigest: super::super::Foundation::BOOL, bdatadigest: super::super::Foundation::BOOL, authtype: VDS_ISCSI_AUTH_TYPE) -> windows_core::Result<IVdsAsync>;
    fn LogoutFromTarget(&self, targetid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsIscsiInitiatorAdapter {}
impl IVdsIscsiInitiatorAdapter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsIscsiInitiatorAdapter_Vtbl
    where
        Identity: IVdsIscsiInitiatorAdapter_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiInitiatorAdapter_Impl::GetProperties(this, core::mem::transmute_copy(&pinitiatoradapterprop)).into()
        }
        unsafe extern "system" fn QueryInitiatorPortals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiInitiatorAdapter_Impl::QueryInitiatorPortals(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoginToTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: windows_core::GUID, targetportalid: windows_core::GUID, initiatorportalid: windows_core::GUID, ulloginflags: u32, bheaderdigest: super::super::Foundation::BOOL, bdatadigest: super::super::Foundation::BOOL, authtype: VDS_ISCSI_AUTH_TYPE, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiInitiatorAdapter_Impl::LoginToTarget(this, core::mem::transmute_copy(&logintype), core::mem::transmute(&targetid), core::mem::transmute(&targetportalid), core::mem::transmute(&initiatorportalid), core::mem::transmute_copy(&ulloginflags), core::mem::transmute_copy(&bheaderdigest), core::mem::transmute_copy(&bdatadigest), core::mem::transmute_copy(&authtype)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogoutFromTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorAdapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiInitiatorAdapter_Impl::LogoutFromTarget(this, core::mem::transmute(&targetid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            QueryInitiatorPortals: QueryInitiatorPortals::<Identity, OFFSET>,
            LoginToTarget: LoginToTarget::<Identity, OFFSET>,
            LogoutFromTarget: LogoutFromTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiInitiatorAdapter as windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiInitiatorPortal_Impl: Sized {
    fn GetProperties(&self, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> windows_core::Result<()>;
    fn GetInitiatorAdapter(&self) -> windows_core::Result<IVdsIscsiInitiatorAdapter>;
    fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::Result<()>;
    fn GetIpsecSecurity(&self, targetportalid: &windows_core::GUID) -> windows_core::Result<u64>;
    fn SetIpsecSecurity(&self, targetportalid: &windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsIscsiInitiatorPortal {}
impl IVdsIscsiInitiatorPortal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsIscsiInitiatorPortal_Vtbl
    where
        Identity: IVdsIscsiInitiatorPortal_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiInitiatorPortal_Impl::GetProperties(this, core::mem::transmute_copy(&pinitiatorportalprop)).into()
        }
        unsafe extern "system" fn GetInitiatorAdapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinitiatoradapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiInitiatorPortal_Impl::GetInitiatorAdapter(this) {
                Ok(ok__) => {
                    ppinitiatoradapter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiInitiatorPortal_Impl::SetIpsecTunnelAddress(this, core::mem::transmute_copy(&ptunneladdress), core::mem::transmute_copy(&pdestinationaddress)).into()
        }
        unsafe extern "system" fn GetIpsecSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetportalid: windows_core::GUID, pullsecurityflags: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiInitiatorPortal_Impl::GetIpsecSecurity(this, core::mem::transmute(&targetportalid)) {
                Ok(ok__) => {
                    pullsecurityflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetportalid: windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiInitiatorPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiInitiatorPortal_Impl::SetIpsecSecurity(this, core::mem::transmute(&targetportalid), core::mem::transmute_copy(&ullsecurityflags), core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetInitiatorAdapter: GetInitiatorAdapter::<Identity, OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Identity, OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Identity, OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiInitiatorPortal as windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiPortal_Impl: Sized {
    fn GetProperties(&self, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> windows_core::Result<()>;
    fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem>;
    fn QueryAssociatedPortalGroups(&self) -> windows_core::Result<IEnumVdsObject>;
    fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> windows_core::Result<()>;
    fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::Result<()>;
    fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> windows_core::Result<u64>;
    fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsIscsiPortal {}
impl IVdsIscsiPortal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsIscsiPortal_Vtbl
    where
        Identity: IVdsIscsiPortal_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiPortal_Impl::GetProperties(this, core::mem::transmute_copy(&pportalprop)).into()
        }
        unsafe extern "system" fn GetSubSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubsystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortal_Impl::GetSubSystem(this) {
                Ok(ok__) => {
                    ppsubsystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortal_Impl::QueryAssociatedPortalGroups(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiPortal_Impl::SetStatus(this, core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiPortal_Impl::SetIpsecTunnelAddress(this, core::mem::transmute_copy(&ptunneladdress), core::mem::transmute_copy(&pdestinationaddress)).into()
        }
        unsafe extern "system" fn GetIpsecSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortal_Impl::GetIpsecSecurity(this, core::mem::transmute_copy(&pinitiatorportaladdress)) {
                Ok(ok__) => {
                    pullsecurityflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiPortal_Impl::SetIpsecSecurity(this, core::mem::transmute_copy(&pinitiatorportaladdress), core::mem::transmute_copy(&ullsecurityflags), core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, OFFSET>,
            QueryAssociatedPortalGroups: QueryAssociatedPortalGroups::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            SetIpsecTunnelAddress: SetIpsecTunnelAddress::<Identity, OFFSET>,
            GetIpsecSecurity: GetIpsecSecurity::<Identity, OFFSET>,
            SetIpsecSecurity: SetIpsecSecurity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiPortal as windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalGroup_Impl: Sized {
    fn GetProperties(&self, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> windows_core::Result<()>;
    fn GetTarget(&self) -> windows_core::Result<IVdsIscsiTarget>;
    fn QueryAssociatedPortals(&self) -> windows_core::Result<IEnumVdsObject>;
    fn AddPortal(&self, portalid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn RemovePortal(&self, portalid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn Delete(&self) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsIscsiPortalGroup {}
impl IVdsIscsiPortalGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsIscsiPortalGroup_Vtbl
    where
        Identity: IVdsIscsiPortalGroup_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiPortalGroup_Impl::GetProperties(this, core::mem::transmute_copy(&pportalgroupprop)).into()
        }
        unsafe extern "system" fn GetTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortalGroup_Impl::GetTarget(this) {
                Ok(ok__) => {
                    pptarget.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedPortals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortalGroup_Impl::QueryAssociatedPortals(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPortal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portalid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortalGroup_Impl::AddPortal(this, core::mem::transmute(&portalid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePortal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, portalid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortalGroup_Impl::RemovePortal(this, core::mem::transmute(&portalid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiPortalGroup_Impl::Delete(this) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetTarget: GetTarget::<Identity, OFFSET>,
            QueryAssociatedPortals: QueryAssociatedPortals::<Identity, OFFSET>,
            AddPortal: AddPortal::<Identity, OFFSET>,
            RemovePortal: RemovePortal::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiPortalGroup as windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalLocal_Impl: Sized {
    fn SetIpsecSecurityLocal(&self, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsIscsiPortalLocal {}
impl IVdsIscsiPortalLocal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsIscsiPortalLocal_Vtbl
    where
        Identity: IVdsIscsiPortalLocal_Impl,
    {
        unsafe extern "system" fn SetIpsecSecurityLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiPortalLocal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiPortalLocal_Impl::SetIpsecSecurityLocal(this, core::mem::transmute_copy(&ullsecurityflags), core::mem::transmute_copy(&pipseckey)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetIpsecSecurityLocal: SetIpsecSecurityLocal::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiPortalLocal as windows_core::Interface>::IID
    }
}
pub trait IVdsIscsiTarget_Impl: Sized {
    fn GetProperties(&self, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> windows_core::Result<()>;
    fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem>;
    fn QueryPortalGroups(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryAssociatedLuns(&self) -> windows_core::Result<IEnumVdsObject>;
    fn CreatePortalGroup(&self) -> windows_core::Result<IVdsAsync>;
    fn Delete(&self) -> windows_core::Result<IVdsAsync>;
    fn SetFriendlyName(&self, pwszfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSharedSecret(&self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RememberInitiatorSharedSecret(&self, pwszinitiatorname: &windows_core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> windows_core::Result<()>;
    fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut windows_core::PWSTR, plnumberofinitiators: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsIscsiTarget {}
impl IVdsIscsiTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsIscsiTarget_Vtbl
    where
        Identity: IVdsIscsiTarget_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiTarget_Impl::GetProperties(this, core::mem::transmute_copy(&ptargetprop)).into()
        }
        unsafe extern "system" fn GetSubSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubsystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiTarget_Impl::GetSubSystem(this) {
                Ok(ok__) => {
                    ppsubsystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortalGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiTarget_Impl::QueryPortalGroups(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAssociatedLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiTarget_Impl::QueryAssociatedLuns(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePortalGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiTarget_Impl::CreatePortalGroup(this) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsIscsiTarget_Impl::Delete(this) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiTarget_Impl::SetFriendlyName(this, core::mem::transmute(&pwszfriendlyname)).into()
        }
        unsafe extern "system" fn SetSharedSecret<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiTarget_Impl::SetSharedSecret(this, core::mem::transmute_copy(&ptargetsharedsecret), core::mem::transmute(&pwszinitiatorname)).into()
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszinitiatorname: windows_core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiTarget_Impl::RememberInitiatorSharedSecret(this, core::mem::transmute(&pwszinitiatorname), core::mem::transmute_copy(&pinitiatorsharedsecret)).into()
        }
        unsafe extern "system" fn GetConnectedInitiators<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppwszinitiatorlist: *mut *mut windows_core::PWSTR, plnumberofinitiators: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsIscsiTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsIscsiTarget_Impl::GetConnectedInitiators(this, core::mem::transmute_copy(&pppwszinitiatorlist), core::mem::transmute_copy(&plnumberofinitiators)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, OFFSET>,
            QueryPortalGroups: QueryPortalGroups::<Identity, OFFSET>,
            QueryAssociatedLuns: QueryAssociatedLuns::<Identity, OFFSET>,
            CreatePortalGroup: CreatePortalGroup::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, OFFSET>,
            SetSharedSecret: SetSharedSecret::<Identity, OFFSET>,
            RememberInitiatorSharedSecret: RememberInitiatorSharedSecret::<Identity, OFFSET>,
            GetConnectedInitiators: GetConnectedInitiators::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsIscsiTarget as windows_core::Interface>::IID
    }
}
pub trait IVdsLun_Impl: Sized {
    fn GetProperties(&self, plunprop: *mut VDS_LUN_PROP) -> windows_core::Result<()>;
    fn GetSubSystem(&self) -> windows_core::Result<IVdsSubSystem>;
    fn GetIdentificationData(&self, pluninfo: *mut VDS_LUN_INFORMATION) -> windows_core::Result<()>;
    fn QueryActiveControllers(&self) -> windows_core::Result<IEnumVdsObject>;
    fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32) -> windows_core::Result<IVdsAsync>;
    fn Shrink(&self, ullnumberofbytestoremove: u64) -> windows_core::Result<IVdsAsync>;
    fn QueryPlexes(&self) -> windows_core::Result<IEnumVdsObject>;
    fn AddPlex(&self, lunid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn RemovePlex(&self, plexid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn Recover(&self) -> windows_core::Result<IVdsAsync>;
    fn SetMask(&self, pwszunmaskinglist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn AssociateControllers(&self, pactivecontrolleridarray: *const windows_core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const windows_core::GUID, lnumberofinactivecontrollers: i32) -> windows_core::Result<()>;
    fn QueryHints(&self, phints: *mut VDS_HINTS) -> windows_core::Result<()>;
    fn ApplyHints(&self, phints: *const VDS_HINTS) -> windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_LUN_STATUS) -> windows_core::Result<()>;
    fn QueryMaxLunExtendSize(&self, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IVdsLun {}
impl IVdsLun_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLun_Vtbl
    where
        Identity: IVdsLun_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::GetProperties(this, core::mem::transmute_copy(&plunprop)).into()
        }
        unsafe extern "system" fn GetSubSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubsystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::GetSubSystem(this) {
                Ok(ok__) => {
                    ppsubsystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentificationData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::GetIdentificationData(this, core::mem::transmute_copy(&pluninfo)).into()
        }
        unsafe extern "system" fn QueryActiveControllers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::QueryActiveControllers(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::Extend(this, core::mem::transmute_copy(&ullnumberofbytestoadd), core::mem::transmute_copy(&pdriveidarray), core::mem::transmute_copy(&lnumberofdrives)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::Shrink(this, core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPlexes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::QueryPlexes(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPlex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lunid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::AddPlex(this, core::mem::transmute(&lunid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plexid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::RemovePlex(this, core::mem::transmute(&plexid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recover<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::Recover(this) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszunmaskinglist: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::SetMask(this, core::mem::transmute(&pwszunmaskinglist)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::Delete(this).into()
        }
        unsafe extern "system" fn AssociateControllers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivecontrolleridarray: *const windows_core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const windows_core::GUID, lnumberofinactivecontrollers: i32) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::AssociateControllers(this, core::mem::transmute_copy(&pactivecontrolleridarray), core::mem::transmute_copy(&lnumberofactivecontrollers), core::mem::transmute_copy(&pinactivecontrolleridarray), core::mem::transmute_copy(&lnumberofinactivecontrollers)).into()
        }
        unsafe extern "system" fn QueryHints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phints: *mut VDS_HINTS) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::QueryHints(this, core::mem::transmute_copy(&phints)).into()
        }
        unsafe extern "system" fn ApplyHints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phints: *const VDS_HINTS) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::ApplyHints(this, core::mem::transmute_copy(&phints)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_LUN_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun_Impl::SetStatus(this, core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsLun_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLun_Impl::QueryMaxLunExtendSize(this, core::mem::transmute_copy(&pdriveidarray), core::mem::transmute_copy(&lnumberofdrives)) {
                Ok(ok__) => {
                    pullmaxbytestobeadded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSubSystem: GetSubSystem::<Identity, OFFSET>,
            GetIdentificationData: GetIdentificationData::<Identity, OFFSET>,
            QueryActiveControllers: QueryActiveControllers::<Identity, OFFSET>,
            Extend: Extend::<Identity, OFFSET>,
            Shrink: Shrink::<Identity, OFFSET>,
            QueryPlexes: QueryPlexes::<Identity, OFFSET>,
            AddPlex: AddPlex::<Identity, OFFSET>,
            RemovePlex: RemovePlex::<Identity, OFFSET>,
            Recover: Recover::<Identity, OFFSET>,
            SetMask: SetMask::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            AssociateControllers: AssociateControllers::<Identity, OFFSET>,
            QueryHints: QueryHints::<Identity, OFFSET>,
            ApplyHints: ApplyHints::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            QueryMaxLunExtendSize: QueryMaxLunExtendSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLun as windows_core::Interface>::IID
    }
}
pub trait IVdsLun2_Impl: Sized {
    fn QueryHints2(&self, phints2: *mut VDS_HINTS2) -> windows_core::Result<()>;
    fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsLun2 {}
impl IVdsLun2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLun2_Vtbl
    where
        Identity: IVdsLun2_Impl,
    {
        unsafe extern "system" fn QueryHints2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phints2: *mut VDS_HINTS2) -> windows_core::HRESULT
        where
            Identity: IVdsLun2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun2_Impl::QueryHints2(this, core::mem::transmute_copy(&phints2)).into()
        }
        unsafe extern "system" fn ApplyHints2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phints2: *const VDS_HINTS2) -> windows_core::HRESULT
        where
            Identity: IVdsLun2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLun2_Impl::ApplyHints2(this, core::mem::transmute_copy(&phints2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryHints2: QueryHints2::<Identity, OFFSET>,
            ApplyHints2: ApplyHints2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLun2 as windows_core::Interface>::IID
    }
}
pub trait IVdsLunControllerPorts_Impl: Sized {
    fn AssociateControllerPorts(&self, pactivecontrollerportidarray: *const windows_core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const windows_core::GUID, lnumberofinactivecontrollerports: i32) -> windows_core::Result<()>;
    fn QueryActiveControllerPorts(&self) -> windows_core::Result<IEnumVdsObject>;
}
impl windows_core::RuntimeName for IVdsLunControllerPorts {}
impl IVdsLunControllerPorts_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLunControllerPorts_Vtbl
    where
        Identity: IVdsLunControllerPorts_Impl,
    {
        unsafe extern "system" fn AssociateControllerPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivecontrollerportidarray: *const windows_core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const windows_core::GUID, lnumberofinactivecontrollerports: i32) -> windows_core::HRESULT
        where
            Identity: IVdsLunControllerPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunControllerPorts_Impl::AssociateControllerPorts(this, core::mem::transmute_copy(&pactivecontrollerportidarray), core::mem::transmute_copy(&lnumberofactivecontrollerports), core::mem::transmute_copy(&pinactivecontrollerportidarray), core::mem::transmute_copy(&lnumberofinactivecontrollerports)).into()
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLunControllerPorts_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLunControllerPorts_Impl::QueryActiveControllerPorts(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssociateControllerPorts: AssociateControllerPorts::<Identity, OFFSET>,
            QueryActiveControllerPorts: QueryActiveControllerPorts::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunControllerPorts as windows_core::Interface>::IID
    }
}
pub trait IVdsLunIscsi_Impl: Sized {
    fn AssociateTargets(&self, ptargetidarray: *const windows_core::GUID, lnumberoftargets: i32) -> windows_core::Result<()>;
    fn QueryAssociatedTargets(&self) -> windows_core::Result<IEnumVdsObject>;
}
impl windows_core::RuntimeName for IVdsLunIscsi {}
impl IVdsLunIscsi_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLunIscsi_Vtbl
    where
        Identity: IVdsLunIscsi_Impl,
    {
        unsafe extern "system" fn AssociateTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetidarray: *const windows_core::GUID, lnumberoftargets: i32) -> windows_core::HRESULT
        where
            Identity: IVdsLunIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunIscsi_Impl::AssociateTargets(this, core::mem::transmute_copy(&ptargetidarray), core::mem::transmute_copy(&lnumberoftargets)).into()
        }
        unsafe extern "system" fn QueryAssociatedTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLunIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLunIscsi_Impl::QueryAssociatedTargets(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssociateTargets: AssociateTargets::<Identity, OFFSET>,
            QueryAssociatedTargets: QueryAssociatedTargets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunIscsi as windows_core::Interface>::IID
    }
}
pub trait IVdsLunMpio_Impl: Sized {
    fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> windows_core::Result<()>;
    fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> windows_core::Result<()>;
    fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> windows_core::Result<()>;
    fn GetSupportedLbPolicies(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IVdsLunMpio {}
impl IVdsLunMpio_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLunMpio_Vtbl
    where
        Identity: IVdsLunMpio_Impl,
    {
        unsafe extern "system" fn GetPathInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsLunMpio_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunMpio_Impl::GetPathInfo(this, core::mem::transmute_copy(&pppaths), core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsLunMpio_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunMpio_Impl::GetLoadBalancePolicy(this, core::mem::transmute_copy(&ppolicy), core::mem::transmute_copy(&pppaths), core::mem::transmute_copy(&plnumberofpaths)).into()
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> windows_core::HRESULT
        where
            Identity: IVdsLunMpio_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunMpio_Impl::SetLoadBalancePolicy(this, core::mem::transmute_copy(&policy), core::mem::transmute_copy(&ppaths), core::mem::transmute_copy(&lnumberofpaths)).into()
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullbflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsLunMpio_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLunMpio_Impl::GetSupportedLbPolicies(this) {
                Ok(ok__) => {
                    pullbflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPathInfo: GetPathInfo::<Identity, OFFSET>,
            GetLoadBalancePolicy: GetLoadBalancePolicy::<Identity, OFFSET>,
            SetLoadBalancePolicy: SetLoadBalancePolicy::<Identity, OFFSET>,
            GetSupportedLbPolicies: GetSupportedLbPolicies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunMpio as windows_core::Interface>::IID
    }
}
pub trait IVdsLunNaming_Impl: Sized {
    fn SetFriendlyName(&self, pwszfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsLunNaming {}
impl IVdsLunNaming_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLunNaming_Vtbl
    where
        Identity: IVdsLunNaming_Impl,
    {
        unsafe extern "system" fn SetFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsLunNaming_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunNaming_Impl::SetFriendlyName(this, core::mem::transmute(&pwszfriendlyname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetFriendlyName: SetFriendlyName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunNaming as windows_core::Interface>::IID
    }
}
pub trait IVdsLunNumber_Impl: Sized {
    fn GetLunNumber(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IVdsLunNumber {}
impl IVdsLunNumber_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLunNumber_Vtbl
    where
        Identity: IVdsLunNumber_Impl,
    {
        unsafe extern "system" fn GetLunNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullunnumber: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsLunNumber_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLunNumber_Impl::GetLunNumber(this) {
                Ok(ok__) => {
                    pullunnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLunNumber: GetLunNumber::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunNumber as windows_core::Interface>::IID
    }
}
pub trait IVdsLunPlex_Impl: Sized {
    fn GetProperties(&self, pplexprop: *mut VDS_LUN_PLEX_PROP) -> windows_core::Result<()>;
    fn GetLun(&self) -> windows_core::Result<IVdsLun>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()>;
    fn QueryHints(&self, phints: *mut VDS_HINTS) -> windows_core::Result<()>;
    fn ApplyHints(&self, phints: *const VDS_HINTS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsLunPlex {}
impl IVdsLunPlex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsLunPlex_Vtbl
    where
        Identity: IVdsLunPlex_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsLunPlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunPlex_Impl::GetProperties(this, core::mem::transmute_copy(&pplexprop)).into()
        }
        unsafe extern "system" fn GetLun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplun: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsLunPlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsLunPlex_Impl::GetLun(this) {
                Ok(ok__) => {
                    pplun.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsLunPlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunPlex_Impl::QueryExtents(this, core::mem::transmute_copy(&ppextentarray), core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryHints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phints: *mut VDS_HINTS) -> windows_core::HRESULT
        where
            Identity: IVdsLunPlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunPlex_Impl::QueryHints(this, core::mem::transmute_copy(&phints)).into()
        }
        unsafe extern "system" fn ApplyHints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phints: *const VDS_HINTS) -> windows_core::HRESULT
        where
            Identity: IVdsLunPlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsLunPlex_Impl::ApplyHints(this, core::mem::transmute_copy(&phints)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetLun: GetLun::<Identity, OFFSET>,
            QueryExtents: QueryExtents::<Identity, OFFSET>,
            QueryHints: QueryHints::<Identity, OFFSET>,
            ApplyHints: ApplyHints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsLunPlex as windows_core::Interface>::IID
    }
}
pub trait IVdsMaintenance_Impl: Sized {
    fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> windows_core::Result<()>;
    fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> windows_core::Result<()>;
    fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsMaintenance {}
impl IVdsMaintenance_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsMaintenance_Vtbl
    where
        Identity: IVdsMaintenance_Impl,
    {
        unsafe extern "system" fn StartMaintenance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> windows_core::HRESULT
        where
            Identity: IVdsMaintenance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsMaintenance_Impl::StartMaintenance(this, core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn StopMaintenance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> windows_core::HRESULT
        where
            Identity: IVdsMaintenance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsMaintenance_Impl::StopMaintenance(this, core::mem::transmute_copy(&operation)).into()
        }
        unsafe extern "system" fn PulseMaintenance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> windows_core::HRESULT
        where
            Identity: IVdsMaintenance_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsMaintenance_Impl::PulseMaintenance(this, core::mem::transmute_copy(&operation), core::mem::transmute_copy(&ulcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartMaintenance: StartMaintenance::<Identity, OFFSET>,
            StopMaintenance: StopMaintenance::<Identity, OFFSET>,
            PulseMaintenance: PulseMaintenance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsMaintenance as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
pub trait IVdsOpenVDisk_Impl: Sized {
    fn Attach(&self, pstringsecuritydescriptor: &windows_core::PCWSTR, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32) -> windows_core::Result<IVdsAsync>;
    fn Detach(&self, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> windows_core::Result<()>;
    fn DetachAndDelete(&self, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> windows_core::Result<()>;
    fn Compact(&self, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32) -> windows_core::Result<IVdsAsync>;
    fn Merge(&self, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32) -> windows_core::Result<IVdsAsync>;
    fn Expand(&self, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64) -> windows_core::Result<IVdsAsync>;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl windows_core::RuntimeName for IVdsOpenVDisk {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl IVdsOpenVDisk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsOpenVDisk_Vtbl
    where
        Identity: IVdsOpenVDisk_Impl,
    {
        unsafe extern "system" fn Attach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstringsecuritydescriptor: windows_core::PCWSTR, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsOpenVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsOpenVDisk_Impl::Attach(this, core::mem::transmute(&pstringsecuritydescriptor), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&providerspecificflags), core::mem::transmute_copy(&timeoutinms)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Detach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsOpenVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsOpenVDisk_Impl::Detach(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&providerspecificflags)).into()
        }
        unsafe extern "system" fn DetachAndDelete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsOpenVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsOpenVDisk_Impl::DetachAndDelete(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&providerspecificflags)).into()
        }
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsOpenVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsOpenVDisk_Impl::Compact(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&reserved)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsOpenVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsOpenVDisk_Impl::Merge(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&mergedepth)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsOpenVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsOpenVDisk_Impl::Expand(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&newsize)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
            DetachAndDelete: DetachAndDelete::<Identity, OFFSET>,
            Compact: Compact::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsOpenVDisk as windows_core::Interface>::IID
    }
}
pub trait IVdsPack_Impl: Sized {
    fn GetProperties(&self, ppackprop: *mut VDS_PACK_PROP) -> windows_core::Result<()>;
    fn GetProvider(&self) -> windows_core::Result<IVdsProvider>;
    fn QueryVolumes(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryDisks(&self) -> windows_core::Result<IEnumVdsObject>;
    fn CreateVolume(&self, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32) -> windows_core::Result<IVdsAsync>;
    fn AddDisk(&self, diskid: &windows_core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn MigrateDisks(&self, pdiskarray: *const windows_core::GUID, lnumberofdisks: i32, targetpack: &windows_core::GUID, bforce: super::super::Foundation::BOOL, bqueryonly: super::super::Foundation::BOOL, presults: *mut windows_core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReplaceDisk(&self, olddiskid: &windows_core::GUID, newdiskid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn RemoveMissingDisk(&self, diskid: &windows_core::GUID) -> windows_core::Result<()>;
    fn Recover(&self) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsPack {}
impl IVdsPack_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsPack_Vtbl
    where
        Identity: IVdsPack_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppackprop: *mut VDS_PACK_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsPack_Impl::GetProperties(this, core::mem::transmute_copy(&ppackprop)).into()
        }
        unsafe extern "system" fn GetProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack_Impl::GetProvider(this) {
                Ok(ok__) => {
                    ppprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack_Impl::QueryVolumes(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack_Impl::QueryDisks(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack_Impl::CreateVolume(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pinputdiskarray), core::mem::transmute_copy(&lnumberofdisks), core::mem::transmute_copy(&ulstripesize)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, diskid: windows_core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsPack_Impl::AddDisk(this, core::mem::transmute(&diskid), core::mem::transmute_copy(&partitionstyle), core::mem::transmute_copy(&bashotspare)).into()
        }
        unsafe extern "system" fn MigrateDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiskarray: *const windows_core::GUID, lnumberofdisks: i32, targetpack: windows_core::GUID, bforce: super::super::Foundation::BOOL, bqueryonly: super::super::Foundation::BOOL, presults: *mut windows_core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsPack_Impl::MigrateDisks(this, core::mem::transmute_copy(&pdiskarray), core::mem::transmute_copy(&lnumberofdisks), core::mem::transmute(&targetpack), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bqueryonly), core::mem::transmute_copy(&presults), core::mem::transmute_copy(&pbrebootneeded)).into()
        }
        unsafe extern "system" fn ReplaceDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, olddiskid: windows_core::GUID, newdiskid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack_Impl::ReplaceDisk(this, core::mem::transmute(&olddiskid), core::mem::transmute(&newdiskid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMissingDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, diskid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsPack_Impl::RemoveMissingDisk(this, core::mem::transmute(&diskid)).into()
        }
        unsafe extern "system" fn Recover<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack_Impl::Recover(this) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetProvider: GetProvider::<Identity, OFFSET>,
            QueryVolumes: QueryVolumes::<Identity, OFFSET>,
            QueryDisks: QueryDisks::<Identity, OFFSET>,
            CreateVolume: CreateVolume::<Identity, OFFSET>,
            AddDisk: AddDisk::<Identity, OFFSET>,
            MigrateDisks: MigrateDisks::<Identity, OFFSET>,
            ReplaceDisk: ReplaceDisk::<Identity, OFFSET>,
            RemoveMissingDisk: RemoveMissingDisk::<Identity, OFFSET>,
            Recover: Recover::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsPack as windows_core::Interface>::IID
    }
}
pub trait IVdsPack2_Impl: Sized {
    fn CreateVolume2(&self, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ulalign: u32) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsPack2 {}
impl IVdsPack2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsPack2_Vtbl
    where
        Identity: IVdsPack2_Impl,
    {
        unsafe extern "system" fn CreateVolume2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ulalign: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsPack2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsPack2_Impl::CreateVolume2(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pinputdiskarray), core::mem::transmute_copy(&lnumberofdisks), core::mem::transmute_copy(&ulstripesize), core::mem::transmute_copy(&ulalign)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateVolume2: CreateVolume2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsPack2 as windows_core::Interface>::IID
    }
}
pub trait IVdsProvider_Impl: Sized {
    fn GetProperties(&self, pproviderprop: *mut VDS_PROVIDER_PROP) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsProvider {}
impl IVdsProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsProvider_Vtbl
    where
        Identity: IVdsProvider_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsProvider_Impl::GetProperties(this, core::mem::transmute_copy(&pproviderprop)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperties: GetProperties::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsProvider as windows_core::Interface>::IID
    }
}
pub trait IVdsProviderPrivate_Impl: Sized {
    fn GetObject(&self, objectid: &windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> windows_core::Result<windows_core::IUnknown>;
    fn OnLoad(&self, pwszmachinename: &windows_core::PCWSTR, pcallbackobject: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnUnload(&self, bforceunload: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsProviderPrivate {}
impl IVdsProviderPrivate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsProviderPrivate_Vtbl
    where
        Identity: IVdsProviderPrivate_Impl,
    {
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: windows_core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsProviderPrivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsProviderPrivate_Impl::GetObject(this, core::mem::transmute(&objectid), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppobjectunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLoad<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmachinename: windows_core::PCWSTR, pcallbackobject: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsProviderPrivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsProviderPrivate_Impl::OnLoad(this, core::mem::transmute(&pwszmachinename), windows_core::from_raw_borrowed(&pcallbackobject)).into()
        }
        unsafe extern "system" fn OnUnload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsProviderPrivate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsProviderPrivate_Impl::OnUnload(this, core::mem::transmute_copy(&bforceunload)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObject: GetObject::<Identity, OFFSET>,
            OnLoad: OnLoad::<Identity, OFFSET>,
            OnUnload: OnUnload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsProviderPrivate as windows_core::Interface>::IID
    }
}
pub trait IVdsProviderSupport_Impl: Sized {
    fn GetVersionSupport(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IVdsProviderSupport {}
impl IVdsProviderSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsProviderSupport_Vtbl
    where
        Identity: IVdsProviderSupport_Impl,
    {
        unsafe extern "system" fn GetVersionSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulversionsupport: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsProviderSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsProviderSupport_Impl::GetVersionSupport(this) {
                Ok(ok__) => {
                    ulversionsupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetVersionSupport: GetVersionSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsProviderSupport as windows_core::Interface>::IID
    }
}
pub trait IVdsRemovable_Impl: Sized {
    fn QueryMedia(&self) -> windows_core::Result<()>;
    fn Eject(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsRemovable {}
impl IVdsRemovable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsRemovable_Vtbl
    where
        Identity: IVdsRemovable_Impl,
    {
        unsafe extern "system" fn QueryMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsRemovable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsRemovable_Impl::QueryMedia(this).into()
        }
        unsafe extern "system" fn Eject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsRemovable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsRemovable_Impl::Eject(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryMedia: QueryMedia::<Identity, OFFSET>, Eject: Eject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsRemovable as windows_core::Interface>::IID
    }
}
pub trait IVdsService_Impl: Sized {
    fn IsServiceReady(&self) -> windows_core::Result<()>;
    fn WaitForServiceReady(&self) -> windows_core::Result<()>;
    fn GetProperties(&self) -> windows_core::Result<VDS_SERVICE_PROP>;
    fn QueryProviders(&self, masks: u32) -> windows_core::Result<IEnumVdsObject>;
    fn QueryMaskedDisks(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryUnallocatedDisks(&self) -> windows_core::Result<IEnumVdsObject>;
    fn GetObject(&self, objectid: &windows_core::GUID, r#type: VDS_OBJECT_TYPE) -> windows_core::Result<windows_core::IUnknown>;
    fn QueryDriveLetters(&self, wcfirstletter: u16, count: u32, pdriveletterproparray: *mut VDS_DRIVE_LETTER_PROP) -> windows_core::Result<()>;
    fn QueryFileSystemTypes(&self, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> windows_core::Result<()>;
    fn Reenumerate(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn CleanupObsoleteMountPoints(&self) -> windows_core::Result<()>;
    fn Advise(&self, psink: Option<&IVdsAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn Reboot(&self) -> windows_core::Result<()>;
    fn SetFlags(&self, ulflags: u32) -> windows_core::Result<()>;
    fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsService {}
impl IVdsService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsService_Vtbl
    where
        Identity: IVdsService_Impl,
    {
        unsafe extern "system" fn IsServiceReady<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::IsServiceReady(this).into()
        }
        unsafe extern "system" fn WaitForServiceReady<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::WaitForServiceReady(this).into()
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserviceprop: *mut VDS_SERVICE_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsService_Impl::GetProperties(this) {
                Ok(ok__) => {
                    pserviceprop.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, masks: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsService_Impl::QueryProviders(this, core::mem::transmute_copy(&masks)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaskedDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsService_Impl::QueryMaskedDisks(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryUnallocatedDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsService_Impl::QueryUnallocatedDisks(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: windows_core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsService_Impl::GetObject(this, core::mem::transmute(&objectid), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    ppobjectunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDriveLetters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wcfirstletter: u16, count: u32, pdriveletterproparray: *mut VDS_DRIVE_LETTER_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::QueryDriveLetters(this, core::mem::transmute_copy(&wcfirstletter), core::mem::transmute_copy(&count), core::mem::transmute_copy(&pdriveletterproparray)).into()
        }
        unsafe extern "system" fn QueryFileSystemTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::QueryFileSystemTypes(this, core::mem::transmute_copy(&ppfilesystemtypeprops), core::mem::transmute_copy(&plnumberoffilesystems)).into()
        }
        unsafe extern "system" fn Reenumerate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::Reenumerate(this).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn CleanupObsoleteMountPoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::CleanupObsoleteMountPoints(this).into()
        }
        unsafe extern "system" fn Advise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsService_Impl::Advise(this, windows_core::from_raw_borrowed(&psink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn Reboot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::Reboot(this).into()
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::SetFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsService_Impl::ClearFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsServiceReady: IsServiceReady::<Identity, OFFSET>,
            WaitForServiceReady: WaitForServiceReady::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            QueryProviders: QueryProviders::<Identity, OFFSET>,
            QueryMaskedDisks: QueryMaskedDisks::<Identity, OFFSET>,
            QueryUnallocatedDisks: QueryUnallocatedDisks::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            QueryDriveLetters: QueryDriveLetters::<Identity, OFFSET>,
            QueryFileSystemTypes: QueryFileSystemTypes::<Identity, OFFSET>,
            Reenumerate: Reenumerate::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            CleanupObsoleteMountPoints: CleanupObsoleteMountPoints::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            Reboot: Reboot::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            ClearFlags: ClearFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsService as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceHba_Impl: Sized {
    fn QueryHbaPorts(&self) -> windows_core::Result<IEnumVdsObject>;
}
impl windows_core::RuntimeName for IVdsServiceHba {}
impl IVdsServiceHba_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceHba_Vtbl
    where
        Identity: IVdsServiceHba_Impl,
    {
        unsafe extern "system" fn QueryHbaPorts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsServiceHba_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceHba_Impl::QueryHbaPorts(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryHbaPorts: QueryHbaPorts::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceHba as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceInitialization_Impl: Sized {
    fn Initialize(&self, pwszmachinename: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsServiceInitialization {}
impl IVdsServiceInitialization_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceInitialization_Vtbl
    where
        Identity: IVdsServiceInitialization_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmachinename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsServiceInitialization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceInitialization_Impl::Initialize(this, core::mem::transmute(&pwszmachinename)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceInitialization as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceIscsi_Impl: Sized {
    fn GetInitiatorName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn QueryInitiatorAdapters(&self) -> windows_core::Result<IEnumVdsObject>;
    fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()>;
    fn SetAllIpsecTunnelAddresses(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::Result<()>;
    fn SetAllIpsecSecurity(&self, targetportalid: &windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()>;
    fn SetInitiatorSharedSecret(&self, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RememberTargetSharedSecret(&self, targetid: &windows_core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsServiceIscsi {}
impl IVdsServiceIscsi_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceIscsi_Vtbl
    where
        Identity: IVdsServiceIscsi_Impl,
    {
        unsafe extern "system" fn GetInitiatorName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwsziscsiname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceIscsi_Impl::GetInitiatorName(this) {
                Ok(ok__) => {
                    ppwsziscsiname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInitiatorAdapters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceIscsi_Impl::QueryInitiatorAdapters(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceIscsi_Impl::SetIpsecGroupPresharedKey(this, core::mem::transmute_copy(&pipseckey)).into()
        }
        unsafe extern "system" fn SetAllIpsecTunnelAddresses<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceIscsi_Impl::SetAllIpsecTunnelAddresses(this, core::mem::transmute_copy(&ptunneladdress), core::mem::transmute_copy(&pdestinationaddress)).into()
        }
        unsafe extern "system" fn SetAllIpsecSecurity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetportalid: windows_core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceIscsi_Impl::SetAllIpsecSecurity(this, core::mem::transmute(&targetportalid), core::mem::transmute_copy(&ullsecurityflags), core::mem::transmute_copy(&pipseckey)).into()
        }
        unsafe extern "system" fn SetInitiatorSharedSecret<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceIscsi_Impl::SetInitiatorSharedSecret(this, core::mem::transmute_copy(&pinitiatorsharedsecret), core::mem::transmute(&targetid)).into()
        }
        unsafe extern "system" fn RememberTargetSharedSecret<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: windows_core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> windows_core::HRESULT
        where
            Identity: IVdsServiceIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceIscsi_Impl::RememberTargetSharedSecret(this, core::mem::transmute(&targetid), core::mem::transmute_copy(&ptargetsharedsecret)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInitiatorName: GetInitiatorName::<Identity, OFFSET>,
            QueryInitiatorAdapters: QueryInitiatorAdapters::<Identity, OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Identity, OFFSET>,
            SetAllIpsecTunnelAddresses: SetAllIpsecTunnelAddresses::<Identity, OFFSET>,
            SetAllIpsecSecurity: SetAllIpsecSecurity::<Identity, OFFSET>,
            SetInitiatorSharedSecret: SetInitiatorSharedSecret::<Identity, OFFSET>,
            RememberTargetSharedSecret: RememberTargetSharedSecret::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceIscsi as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceLoader_Impl: Sized {
    fn LoadService(&self, pwszmachinename: &windows_core::PCWSTR) -> windows_core::Result<IVdsService>;
}
impl windows_core::RuntimeName for IVdsServiceLoader {}
impl IVdsServiceLoader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceLoader_Vtbl
    where
        Identity: IVdsServiceLoader_Impl,
    {
        unsafe extern "system" fn LoadService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmachinename: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsServiceLoader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceLoader_Impl::LoadService(this, core::mem::transmute(&pwszmachinename)) {
                Ok(ok__) => {
                    ppservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LoadService: LoadService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceLoader as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceSAN_Impl: Sized {
    fn GetSANPolicy(&self) -> windows_core::Result<VDS_SAN_POLICY>;
    fn SetSANPolicy(&self, sanpolicy: VDS_SAN_POLICY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsServiceSAN {}
impl IVdsServiceSAN_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceSAN_Vtbl
    where
        Identity: IVdsServiceSAN_Impl,
    {
        unsafe extern "system" fn GetSANPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psanpolicy: *mut VDS_SAN_POLICY) -> windows_core::HRESULT
        where
            Identity: IVdsServiceSAN_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceSAN_Impl::GetSANPolicy(this) {
                Ok(ok__) => {
                    psanpolicy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSANPolicy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sanpolicy: VDS_SAN_POLICY) -> windows_core::HRESULT
        where
            Identity: IVdsServiceSAN_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceSAN_Impl::SetSANPolicy(this, core::mem::transmute_copy(&sanpolicy)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSANPolicy: GetSANPolicy::<Identity, OFFSET>,
            SetSANPolicy: SetSANPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceSAN as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceSw_Impl: Sized {
    fn GetDiskObject(&self, pwszdeviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IVdsServiceSw {}
impl IVdsServiceSw_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceSw_Vtbl
    where
        Identity: IVdsServiceSw_Impl,
    {
        unsafe extern "system" fn GetDiskObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszdeviceid: windows_core::PCWSTR, ppdiskunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsServiceSw_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceSw_Impl::GetDiskObject(this, core::mem::transmute(&pwszdeviceid)) {
                Ok(ok__) => {
                    ppdiskunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDiskObject: GetDiskObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceSw as windows_core::Interface>::IID
    }
}
pub trait IVdsServiceUninstallDisk_Impl: Sized {
    fn GetDiskIdFromLunInfo(&self, pluninfo: *const VDS_LUN_INFORMATION) -> windows_core::Result<windows_core::GUID>;
    fn UninstallDisks(&self, pdiskidarray: *const windows_core::GUID, ulcount: u32, bforce: super::super::Foundation::BOOLEAN, pbreboot: *mut u8, presults: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsServiceUninstallDisk {}
impl IVdsServiceUninstallDisk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsServiceUninstallDisk_Vtbl
    where
        Identity: IVdsServiceUninstallDisk_Impl,
    {
        unsafe extern "system" fn GetDiskIdFromLunInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pluninfo: *const VDS_LUN_INFORMATION, pdiskid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsServiceUninstallDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsServiceUninstallDisk_Impl::GetDiskIdFromLunInfo(this, core::mem::transmute_copy(&pluninfo)) {
                Ok(ok__) => {
                    pdiskid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UninstallDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiskidarray: *const windows_core::GUID, ulcount: u32, bforce: super::super::Foundation::BOOLEAN, pbreboot: *mut u8, presults: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IVdsServiceUninstallDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsServiceUninstallDisk_Impl::UninstallDisks(this, core::mem::transmute_copy(&pdiskidarray), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&pbreboot), core::mem::transmute_copy(&presults)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDiskIdFromLunInfo: GetDiskIdFromLunInfo::<Identity, OFFSET>,
            UninstallDisks: UninstallDisks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsServiceUninstallDisk as windows_core::Interface>::IID
    }
}
pub trait IVdsStoragePool_Impl: Sized {
    fn GetProvider(&self) -> windows_core::Result<IVdsProvider>;
    fn GetProperties(&self, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> windows_core::Result<()>;
    fn GetAttributes(&self, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> windows_core::Result<()>;
    fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()>;
    fn QueryAllocatedLuns(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryAllocatedStoragePools(&self) -> windows_core::Result<IEnumVdsObject>;
}
impl windows_core::RuntimeName for IVdsStoragePool {}
impl IVdsStoragePool_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsStoragePool_Vtbl
    where
        Identity: IVdsStoragePool_Impl,
    {
        unsafe extern "system" fn GetProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsStoragePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsStoragePool_Impl::GetProvider(this) {
                Ok(ok__) => {
                    ppprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsStoragePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsStoragePool_Impl::GetProperties(this, core::mem::transmute_copy(&pstoragepoolprop)).into()
        }
        unsafe extern "system" fn GetAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> windows_core::HRESULT
        where
            Identity: IVdsStoragePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsStoragePool_Impl::GetAttributes(this, core::mem::transmute_copy(&pstoragepoolattributes)).into()
        }
        unsafe extern "system" fn QueryDriveExtents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsStoragePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsStoragePool_Impl::QueryDriveExtents(this, core::mem::transmute_copy(&ppextentarray), core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn QueryAllocatedLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsStoragePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsStoragePool_Impl::QueryAllocatedLuns(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsStoragePool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsStoragePool_Impl::QueryAllocatedStoragePools(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProvider: GetProvider::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            QueryDriveExtents: QueryDriveExtents::<Identity, OFFSET>,
            QueryAllocatedLuns: QueryAllocatedLuns::<Identity, OFFSET>,
            QueryAllocatedStoragePools: QueryAllocatedStoragePools::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsStoragePool as windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystem_Impl: Sized {
    fn GetProperties(&self, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> windows_core::Result<()>;
    fn GetProvider(&self) -> windows_core::Result<IVdsProvider>;
    fn QueryControllers(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryLuns(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryDrives(&self) -> windows_core::Result<IEnumVdsObject>;
    fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> windows_core::Result<IVdsDrive>;
    fn Reenumerate(&self) -> windows_core::Result<()>;
    fn SetControllerStatus(&self, ponlinecontrolleridarray: *const windows_core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const windows_core::GUID, lnumberofofflinecontrollers: i32) -> windows_core::Result<()>;
    fn CreateLun(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &windows_core::PCWSTR, phints: *const VDS_HINTS) -> windows_core::Result<IVdsAsync>;
    fn ReplaceDrive(&self, drivetobereplaced: &windows_core::GUID, replacementdrive: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> windows_core::Result<()>;
    fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IVdsSubSystem {}
impl IVdsSubSystem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSubSystem_Vtbl
    where
        Identity: IVdsSubSystem_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystem_Impl::GetProperties(this, core::mem::transmute_copy(&psubsystemprop)).into()
        }
        unsafe extern "system" fn GetProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::GetProvider(this) {
                Ok(ok__) => {
                    ppprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryControllers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::QueryControllers(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryLuns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::QueryLuns(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryDrives<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::QueryDrives(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::GetDrive(this, core::mem::transmute_copy(&sbusnumber), core::mem::transmute_copy(&sslotnumber)) {
                Ok(ok__) => {
                    ppdrive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reenumerate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystem_Impl::Reenumerate(this).into()
        }
        unsafe extern "system" fn SetControllerStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ponlinecontrolleridarray: *const windows_core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const windows_core::GUID, lnumberofofflinecontrollers: i32) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystem_Impl::SetControllerStatus(this, core::mem::transmute_copy(&ponlinecontrolleridarray), core::mem::transmute_copy(&lnumberofonlinecontrollers), core::mem::transmute_copy(&pofflinecontrolleridarray), core::mem::transmute_copy(&lnumberofofflinecontrollers)).into()
        }
        unsafe extern "system" fn CreateLun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: windows_core::PCWSTR, phints: *const VDS_HINTS, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::CreateLun(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ullsizeinbytes), core::mem::transmute_copy(&pdriveidarray), core::mem::transmute_copy(&lnumberofdrives), core::mem::transmute(&pwszunmaskinglist), core::mem::transmute_copy(&phints)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceDrive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drivetobereplaced: windows_core::GUID, replacementdrive: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystem_Impl::ReplaceDrive(this, core::mem::transmute(&drivetobereplaced), core::mem::transmute(&replacementdrive)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystem_Impl::SetStatus(this, core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem_Impl::QueryMaxLunCreateSize(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdriveidarray), core::mem::transmute_copy(&lnumberofdrives), core::mem::transmute_copy(&phints)) {
                Ok(ok__) => {
                    pullmaxlunsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetProvider: GetProvider::<Identity, OFFSET>,
            QueryControllers: QueryControllers::<Identity, OFFSET>,
            QueryLuns: QueryLuns::<Identity, OFFSET>,
            QueryDrives: QueryDrives::<Identity, OFFSET>,
            GetDrive: GetDrive::<Identity, OFFSET>,
            Reenumerate: Reenumerate::<Identity, OFFSET>,
            SetControllerStatus: SetControllerStatus::<Identity, OFFSET>,
            CreateLun: CreateLun::<Identity, OFFSET>,
            ReplaceDrive: ReplaceDrive::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            QueryMaxLunCreateSize: QueryMaxLunCreateSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystem as windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystem2_Impl: Sized {
    fn GetProperties2(&self, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> windows_core::Result<()>;
    fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> windows_core::Result<IVdsDrive>;
    fn CreateLun2(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: &windows_core::PCWSTR, phints2: *const VDS_HINTS2) -> windows_core::Result<IVdsAsync>;
    fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IVdsSubSystem2 {}
impl IVdsSubSystem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSubSystem2_Vtbl
    where
        Identity: IVdsSubSystem2_Impl,
    {
        unsafe extern "system" fn GetProperties2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystem2_Impl::GetProperties2(this, core::mem::transmute_copy(&psubsystemprop2)).into()
        }
        unsafe extern "system" fn GetDrive2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem2_Impl::GetDrive2(this, core::mem::transmute_copy(&sbusnumber), core::mem::transmute_copy(&sslotnumber), core::mem::transmute_copy(&ulenclosurenumber)) {
                Ok(ok__) => {
                    ppdrive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLun2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, pwszunmaskinglist: windows_core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem2_Impl::CreateLun2(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ullsizeinbytes), core::mem::transmute_copy(&pdriveidarray), core::mem::transmute_copy(&lnumberofdrives), core::mem::transmute(&pwszunmaskinglist), core::mem::transmute_copy(&phints2)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const windows_core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystem2_Impl::QueryMaxLunCreateSize2(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pdriveidarray), core::mem::transmute_copy(&lnumberofdrives), core::mem::transmute_copy(&phints2)) {
                Ok(ok__) => {
                    pullmaxlunsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties2: GetProperties2::<Identity, OFFSET>,
            GetDrive2: GetDrive2::<Identity, OFFSET>,
            CreateLun2: CreateLun2::<Identity, OFFSET>,
            QueryMaxLunCreateSize2: QueryMaxLunCreateSize2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystem2 as windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemImportTarget_Impl: Sized {
    fn GetImportTarget(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetImportTarget(&self, pwsziscsiname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsSubSystemImportTarget {}
impl IVdsSubSystemImportTarget_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSubSystemImportTarget_Vtbl
    where
        Identity: IVdsSubSystemImportTarget_Impl,
    {
        unsafe extern "system" fn GetImportTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwsziscsiname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemImportTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystemImportTarget_Impl::GetImportTarget(this) {
                Ok(ok__) => {
                    ppwsziscsiname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImportTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsziscsiname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemImportTarget_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystemImportTarget_Impl::SetImportTarget(this, core::mem::transmute(&pwsziscsiname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetImportTarget: GetImportTarget::<Identity, OFFSET>,
            SetImportTarget: SetImportTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemImportTarget as windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemInterconnect_Impl: Sized {
    fn GetSupportedInterconnects(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IVdsSubSystemInterconnect {}
impl IVdsSubSystemInterconnect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSubSystemInterconnect_Vtbl
    where
        Identity: IVdsSubSystemInterconnect_Impl,
    {
        unsafe extern "system" fn GetSupportedInterconnects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemInterconnect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystemInterconnect_Impl::GetSupportedInterconnects(this) {
                Ok(ok__) => {
                    pulsupportedinterconnectsflag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSupportedInterconnects: GetSupportedInterconnects::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemInterconnect as windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemIscsi_Impl: Sized {
    fn QueryTargets(&self) -> windows_core::Result<IEnumVdsObject>;
    fn QueryPortals(&self) -> windows_core::Result<IEnumVdsObject>;
    fn CreateTarget(&self, pwsziscsiname: &windows_core::PCWSTR, pwszfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<IVdsAsync>;
    fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsSubSystemIscsi {}
impl IVdsSubSystemIscsi_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSubSystemIscsi_Vtbl
    where
        Identity: IVdsSubSystemIscsi_Impl,
    {
        unsafe extern "system" fn QueryTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystemIscsi_Impl::QueryTargets(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPortals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystemIscsi_Impl::QueryPortals(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTarget<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsziscsiname: windows_core::PCWSTR, pwszfriendlyname: windows_core::PCWSTR, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSubSystemIscsi_Impl::CreateTarget(this, core::mem::transmute(&pwsziscsiname), core::mem::transmute(&pwszfriendlyname)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemIscsi_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystemIscsi_Impl::SetIpsecGroupPresharedKey(this, core::mem::transmute_copy(&pipseckey)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryTargets: QueryTargets::<Identity, OFFSET>,
            QueryPortals: QueryPortals::<Identity, OFFSET>,
            CreateTarget: CreateTarget::<Identity, OFFSET>,
            SetIpsecGroupPresharedKey: SetIpsecGroupPresharedKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemIscsi as windows_core::Interface>::IID
    }
}
pub trait IVdsSubSystemNaming_Impl: Sized {
    fn SetFriendlyName(&self, pwszfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsSubSystemNaming {}
impl IVdsSubSystemNaming_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSubSystemNaming_Vtbl
    where
        Identity: IVdsSubSystemNaming_Impl,
    {
        unsafe extern "system" fn SetFriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsSubSystemNaming_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsSubSystemNaming_Impl::SetFriendlyName(this, core::mem::transmute(&pwszfriendlyname)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetFriendlyName: SetFriendlyName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSubSystemNaming as windows_core::Interface>::IID
    }
}
pub trait IVdsSwProvider_Impl: Sized {
    fn QueryPacks(&self) -> windows_core::Result<IEnumVdsObject>;
    fn CreatePack(&self) -> windows_core::Result<IVdsPack>;
}
impl windows_core::RuntimeName for IVdsSwProvider {}
impl IVdsSwProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsSwProvider_Vtbl
    where
        Identity: IVdsSwProvider_Impl,
    {
        unsafe extern "system" fn QueryPacks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSwProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSwProvider_Impl::QueryPacks(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppack: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsSwProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsSwProvider_Impl::CreatePack(this) {
                Ok(ok__) => {
                    pppack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryPacks: QueryPacks::<Identity, OFFSET>,
            CreatePack: CreatePack::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsSwProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
pub trait IVdsVDisk_Impl: Sized {
    fn Open(&self, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32) -> windows_core::Result<IVdsOpenVDisk>;
    fn GetProperties(&self, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> windows_core::Result<()>;
    fn GetHostVolume(&self) -> windows_core::Result<IVdsVolume>;
    fn GetDeviceName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl windows_core::RuntimeName for IVdsVDisk {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl IVdsVDisk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVDisk_Vtbl
    where
        Identity: IVdsVDisk_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32, ppopenvdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVDisk_Impl::Open(this, core::mem::transmute_copy(&accessmask), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&readwritedepth)) {
                Ok(ok__) => {
                    ppopenvdisk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> windows_core::HRESULT
        where
            Identity: IVdsVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVDisk_Impl::GetProperties(this, core::mem::transmute_copy(&pdiskproperties)).into()
        }
        unsafe extern "system" fn GetHostVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvolume: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVDisk_Impl::GetHostVolume(this) {
                Ok(ok__) => {
                    ppvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevicename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsVDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVDisk_Impl::GetDeviceName(this) {
                Ok(ok__) => {
                    ppdevicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetHostVolume: GetHostVolume::<Identity, OFFSET>,
            GetDeviceName: GetDeviceName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVDisk as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
pub trait IVdsVdProvider_Impl: Sized {
    fn QueryVDisks(&self) -> windows_core::Result<IEnumVdsObject>;
    fn CreateVDisk(&self, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: &windows_core::PCWSTR, pstringsecuritydescriptor: &windows_core::PCWSTR, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: *mut Option<IVdsAsync>) -> windows_core::Result<()>;
    fn AddVDisk(&self, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: &windows_core::PCWSTR, ppvdisk: *mut Option<IVdsVDisk>) -> windows_core::Result<()>;
    fn GetDiskFromVDisk(&self, pvdisk: Option<&IVdsVDisk>) -> windows_core::Result<IVdsDisk>;
    fn GetVDiskFromDisk(&self, pdisk: Option<&IVdsDisk>) -> windows_core::Result<IVdsVDisk>;
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl windows_core::RuntimeName for IVdsVdProvider {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl IVdsVdProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVdProvider_Vtbl
    where
        Identity: IVdsVdProvider_Impl,
    {
        unsafe extern "system" fn QueryVDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVdProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVdProvider_Impl::QueryVDisks(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: windows_core::PCWSTR, pstringsecuritydescriptor: windows_core::PCWSTR, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVdProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVdProvider_Impl::CreateVDisk(this, core::mem::transmute_copy(&virtualdevicetype), core::mem::transmute(&ppath), core::mem::transmute(&pstringsecuritydescriptor), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&providerspecificflags), core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&pcreatediskparameters), core::mem::transmute_copy(&ppasync)).into()
        }
        unsafe extern "system" fn AddVDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: windows_core::PCWSTR, ppvdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVdProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVdProvider_Impl::AddVDisk(this, core::mem::transmute_copy(&virtualdevicetype), core::mem::transmute(&ppath), core::mem::transmute_copy(&ppvdisk)).into()
        }
        unsafe extern "system" fn GetDiskFromVDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvdisk: *mut core::ffi::c_void, ppdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVdProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVdProvider_Impl::GetDiskFromVDisk(this, windows_core::from_raw_borrowed(&pvdisk)) {
                Ok(ok__) => {
                    ppdisk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVDiskFromDisk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisk: *mut core::ffi::c_void, ppvdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVdProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVdProvider_Impl::GetVDiskFromDisk(this, windows_core::from_raw_borrowed(&pdisk)) {
                Ok(ok__) => {
                    ppvdisk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryVDisks: QueryVDisks::<Identity, OFFSET>,
            CreateVDisk: CreateVDisk::<Identity, OFFSET>,
            AddVDisk: AddVDisk::<Identity, OFFSET>,
            GetDiskFromVDisk: GetDiskFromVDisk::<Identity, OFFSET>,
            GetVDiskFromDisk: GetVDiskFromDisk::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVdProvider as windows_core::Interface>::IID
    }
}
pub trait IVdsVolume_Impl: Sized {
    fn GetProperties(&self, pvolumeproperties: *mut VDS_VOLUME_PROP) -> windows_core::Result<()>;
    fn GetPack(&self) -> windows_core::Result<IVdsPack>;
    fn QueryPlexes(&self) -> windows_core::Result<IEnumVdsObject>;
    fn Extend(&self, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32) -> windows_core::Result<IVdsAsync>;
    fn Shrink(&self, ullnumberofbytestoremove: u64) -> windows_core::Result<IVdsAsync>;
    fn AddPlex(&self, volumeid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn BreakPlex(&self, plexid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn RemovePlex(&self, plexid: &windows_core::GUID) -> windows_core::Result<IVdsAsync>;
    fn Delete(&self, bforce: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetFlags(&self, ulflags: u32, brevertonclose: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ClearFlags(&self, ulflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsVolume {}
impl IVdsVolume_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolume_Vtbl
    where
        Identity: IVdsVolume_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvolumeproperties: *mut VDS_VOLUME_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolume_Impl::GetProperties(this, core::mem::transmute_copy(&pvolumeproperties)).into()
        }
        unsafe extern "system" fn GetPack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppack: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::GetPack(this) {
                Ok(ok__) => {
                    pppack.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryPlexes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::QueryPlexes(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extend<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::Extend(this, core::mem::transmute_copy(&pinputdiskarray), core::mem::transmute_copy(&lnumberofdisks)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::Shrink(this, core::mem::transmute_copy(&ullnumberofbytestoremove)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPlex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumeid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::AddPlex(this, core::mem::transmute(&volumeid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BreakPlex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plexid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::BreakPlex(this, core::mem::transmute(&plexid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plexid: windows_core::GUID, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolume_Impl::RemovePlex(this, core::mem::transmute(&plexid)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforce: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolume_Impl::Delete(this, core::mem::transmute_copy(&bforce)).into()
        }
        unsafe extern "system" fn SetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32, brevertonclose: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolume_Impl::SetFlags(this, core::mem::transmute_copy(&ulflags), core::mem::transmute_copy(&brevertonclose)).into()
        }
        unsafe extern "system" fn ClearFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolume_Impl::ClearFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetPack: GetPack::<Identity, OFFSET>,
            QueryPlexes: QueryPlexes::<Identity, OFFSET>,
            Extend: Extend::<Identity, OFFSET>,
            Shrink: Shrink::<Identity, OFFSET>,
            AddPlex: AddPlex::<Identity, OFFSET>,
            BreakPlex: BreakPlex::<Identity, OFFSET>,
            RemovePlex: RemovePlex::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            ClearFlags: ClearFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolume as windows_core::Interface>::IID
    }
}
pub trait IVdsVolume2_Impl: Sized {
    fn GetProperties2(&self, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsVolume2 {}
impl IVdsVolume2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolume2_Vtbl
    where
        Identity: IVdsVolume2_Impl,
    {
        unsafe extern "system" fn GetProperties2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> windows_core::HRESULT
        where
            Identity: IVdsVolume2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolume2_Impl::GetProperties2(this, core::mem::transmute_copy(&pvolumeproperties)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperties2: GetProperties2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolume2 as windows_core::Interface>::IID
    }
}
pub trait IVdsVolumeMF_Impl: Sized {
    fn GetFileSystemProperties(&self, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> windows_core::Result<()>;
    fn Format(&self, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: &windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> windows_core::Result<IVdsAsync>;
    fn AddAccessPath(&self, pwszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QueryAccessPaths(&self, pwszpatharray: *mut *mut windows_core::PWSTR, plnumberofaccesspaths: *mut i32) -> windows_core::Result<()>;
    fn QueryReparsePoints(&self, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> windows_core::Result<()>;
    fn DeleteAccessPath(&self, pwszpath: &windows_core::PCWSTR, bforce: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Mount(&self) -> windows_core::Result<()>;
    fn Dismount(&self, bforce: super::super::Foundation::BOOL, bpermanent: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetFileSystemFlags(&self, ulflags: u32) -> windows_core::Result<()>;
    fn ClearFileSystemFlags(&self, ulflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsVolumeMF {}
impl IVdsVolumeMF_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolumeMF_Vtbl
    where
        Identity: IVdsVolumeMF_Impl,
    {
        unsafe extern "system" fn GetFileSystemProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::GetFileSystemProperties(this, core::mem::transmute_copy(&pfilesystemprop)).into()
        }
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: windows_core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumeMF_Impl::Format(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&pwszlabel), core::mem::transmute_copy(&dwunitallocationsize), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bquickformat), core::mem::transmute_copy(&benablecompression)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::AddAccessPath(this, core::mem::transmute(&pwszpath)).into()
        }
        unsafe extern "system" fn QueryAccessPaths<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpatharray: *mut *mut windows_core::PWSTR, plnumberofaccesspaths: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::QueryAccessPaths(this, core::mem::transmute_copy(&pwszpatharray), core::mem::transmute_copy(&plnumberofaccesspaths)).into()
        }
        unsafe extern "system" fn QueryReparsePoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::QueryReparsePoints(this, core::mem::transmute_copy(&ppreparsepointprops), core::mem::transmute_copy(&plnumberofreparsepointprops)).into()
        }
        unsafe extern "system" fn DeleteAccessPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpath: windows_core::PCWSTR, bforce: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::DeleteAccessPath(this, core::mem::transmute(&pwszpath), core::mem::transmute_copy(&bforce)).into()
        }
        unsafe extern "system" fn Mount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::Mount(this).into()
        }
        unsafe extern "system" fn Dismount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bforce: super::super::Foundation::BOOL, bpermanent: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::Dismount(this, core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bpermanent)).into()
        }
        unsafe extern "system" fn SetFileSystemFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::SetFileSystemFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        unsafe extern "system" fn ClearFileSystemFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF_Impl::ClearFileSystemFlags(this, core::mem::transmute_copy(&ulflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileSystemProperties: GetFileSystemProperties::<Identity, OFFSET>,
            Format: Format::<Identity, OFFSET>,
            AddAccessPath: AddAccessPath::<Identity, OFFSET>,
            QueryAccessPaths: QueryAccessPaths::<Identity, OFFSET>,
            QueryReparsePoints: QueryReparsePoints::<Identity, OFFSET>,
            DeleteAccessPath: DeleteAccessPath::<Identity, OFFSET>,
            Mount: Mount::<Identity, OFFSET>,
            Dismount: Dismount::<Identity, OFFSET>,
            SetFileSystemFlags: SetFileSystemFlags::<Identity, OFFSET>,
            ClearFileSystemFlags: ClearFileSystemFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolumeMF as windows_core::Interface>::IID
    }
}
pub trait IVdsVolumeMF2_Impl: Sized {
    fn GetFileSystemTypeName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn QueryFileSystemFormatSupport(&self, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> windows_core::Result<()>;
    fn FormatEx(&self, pwszfilesystemtypename: &windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsVolumeMF2 {}
impl IVdsVolumeMF2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolumeMF2_Vtbl
    where
        Identity: IVdsVolumeMF2_Impl,
    {
        unsafe extern "system" fn GetFileSystemTypeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszfilesystemtypename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumeMF2_Impl::GetFileSystemTypeName(this) {
                Ok(ok__) => {
                    ppwszfilesystemtypename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryFileSystemFormatSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF2_Impl::QueryFileSystemFormatSupport(this, core::mem::transmute_copy(&ppfilesystemsupportprops), core::mem::transmute_copy(&plnumberoffilesystems)).into()
        }
        unsafe extern "system" fn FormatEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilesystemtypename: windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: windows_core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumeMF2_Impl::FormatEx(this, core::mem::transmute(&pwszfilesystemtypename), core::mem::transmute_copy(&usfilesystemrevision), core::mem::transmute_copy(&uldesiredunitallocationsize), core::mem::transmute(&pwszlabel), core::mem::transmute_copy(&bforce), core::mem::transmute_copy(&bquickformat), core::mem::transmute_copy(&benablecompression)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFileSystemTypeName: GetFileSystemTypeName::<Identity, OFFSET>,
            QueryFileSystemFormatSupport: QueryFileSystemFormatSupport::<Identity, OFFSET>,
            FormatEx: FormatEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolumeMF2 as windows_core::Interface>::IID
    }
}
pub trait IVdsVolumeMF3_Impl: Sized {
    fn QueryVolumeGuidPathnames(&self, pwszpatharray: *mut *mut windows_core::PWSTR, pulnumberofpaths: *mut u32) -> windows_core::Result<()>;
    fn FormatEx2(&self, pwszfilesystemtypename: &windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: &windows_core::PCWSTR, options: u32) -> windows_core::Result<IVdsAsync>;
    fn OfflineVolume(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsVolumeMF3 {}
impl IVdsVolumeMF3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolumeMF3_Vtbl
    where
        Identity: IVdsVolumeMF3_Impl,
    {
        unsafe extern "system" fn QueryVolumeGuidPathnames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpatharray: *mut *mut windows_core::PWSTR, pulnumberofpaths: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF3_Impl::QueryVolumeGuidPathnames(this, core::mem::transmute_copy(&pwszpatharray), core::mem::transmute_copy(&pulnumberofpaths)).into()
        }
        unsafe extern "system" fn FormatEx2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilesystemtypename: windows_core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: windows_core::PCWSTR, options: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumeMF3_Impl::FormatEx2(this, core::mem::transmute(&pwszfilesystemtypename), core::mem::transmute_copy(&usfilesystemrevision), core::mem::transmute_copy(&uldesiredunitallocationsize), core::mem::transmute(&pwszlabel), core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OfflineVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeMF3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeMF3_Impl::OfflineVolume(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryVolumeGuidPathnames: QueryVolumeGuidPathnames::<Identity, OFFSET>,
            FormatEx2: FormatEx2::<Identity, OFFSET>,
            OfflineVolume: OfflineVolume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolumeMF3 as windows_core::Interface>::IID
    }
}
pub trait IVdsVolumeOnline_Impl: Sized {
    fn Online(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IVdsVolumeOnline {}
impl IVdsVolumeOnline_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolumeOnline_Vtbl
    where
        Identity: IVdsVolumeOnline_Impl,
    {
        unsafe extern "system" fn Online<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeOnline_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumeOnline_Impl::Online(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Online: Online::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolumeOnline as windows_core::Interface>::IID
    }
}
pub trait IVdsVolumePlex_Impl: Sized {
    fn GetProperties(&self, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> windows_core::Result<()>;
    fn GetVolume(&self) -> windows_core::Result<IVdsVolume>;
    fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> windows_core::Result<()>;
    fn Repair(&self, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsVolumePlex {}
impl IVdsVolumePlex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolumePlex_Vtbl
    where
        Identity: IVdsVolumePlex_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> windows_core::HRESULT
        where
            Identity: IVdsVolumePlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumePlex_Impl::GetProperties(this, core::mem::transmute_copy(&pplexproperties)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvolume: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumePlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumePlex_Impl::GetVolume(this) {
                Ok(ok__) => {
                    ppvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryExtents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> windows_core::HRESULT
        where
            Identity: IVdsVolumePlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVdsVolumePlex_Impl::QueryExtents(this, core::mem::transmute_copy(&ppextentarray), core::mem::transmute_copy(&plnumberofextents)).into()
        }
        unsafe extern "system" fn Repair<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumePlex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumePlex_Impl::Repair(this, core::mem::transmute_copy(&pinputdiskarray), core::mem::transmute_copy(&lnumberofdisks)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            QueryExtents: QueryExtents::<Identity, OFFSET>,
            Repair: Repair::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolumePlex as windows_core::Interface>::IID
    }
}
pub trait IVdsVolumeShrink_Impl: Sized {
    fn QueryMaxReclaimableBytes(&self) -> windows_core::Result<u64>;
    fn Shrink(&self, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64) -> windows_core::Result<IVdsAsync>;
}
impl windows_core::RuntimeName for IVdsVolumeShrink {}
impl IVdsVolumeShrink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVdsVolumeShrink_Vtbl
    where
        Identity: IVdsVolumeShrink_Impl,
    {
        unsafe extern "system" fn QueryMaxReclaimableBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullmaxnumberofreclaimablebytes: *mut u64) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeShrink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumeShrink_Impl::QueryMaxReclaimableBytes(this) {
                Ok(ok__) => {
                    pullmaxnumberofreclaimablebytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shrink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVdsVolumeShrink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVdsVolumeShrink_Impl::Shrink(this, core::mem::transmute_copy(&ulldesirednumberofreclaimablebytes), core::mem::transmute_copy(&ullminnumberofreclaimablebytes)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryMaxReclaimableBytes: QueryMaxReclaimableBytes::<Identity, OFFSET>,
            Shrink: Shrink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVdsVolumeShrink as windows_core::Interface>::IID
    }
}
