pub trait IAsynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn RegisterCallback(&self, pdataretrievercallback: Option<&IDataRetrieverCallback>) -> windows_core::Result<()>;
    fn RevokeCallback(&self, pdataretrievercallback: Option<&IDataRetrieverCallback>) -> windows_core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: Option<&ILoadChangeContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsynchronousDataRetriever {}
impl IAsynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAsynchronousDataRetriever_Vtbl
    where
        Identity: IAsynchronousDataRetriever_Impl,
    {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: IAsynchronousDataRetriever_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn RegisterCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataretrievercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAsynchronousDataRetriever_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::RegisterCallback(this, windows_core::from_raw_borrowed(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn RevokeCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataretrievercallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAsynchronousDataRetriever_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::RevokeCallback(this, windows_core::from_raw_borrowed(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploadchangecontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAsynchronousDataRetriever_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsynchronousDataRetriever_Impl::LoadChangeData(this, windows_core::from_raw_borrowed(&ploadchangecontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, OFFSET>,
            RevokeCallback: RevokeCallback::<Identity, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsynchronousDataRetriever as windows_core::Interface>::IID
    }
}
pub trait IChangeConflict_Impl: Sized {
    fn GetDestinationProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSourceProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn GetResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChangeConflict {}
impl IChangeConflict_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IChangeConflict_Vtbl
    where
        Identity: IChangeConflict_Impl,
    {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetDestinationProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetSourceProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetDestinationProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChangeConflict_Impl::GetSourceProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::GetResolveActionForChange(this, core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::SetResolveActionForChange(this, core::mem::transmute_copy(&resolveaction)).into()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::GetResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IChangeConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeConflict_Impl::SetResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&resolveaction)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, OFFSET>,
            GetResolveActionForChange: GetResolveActionForChange::<Identity, OFFSET>,
            SetResolveActionForChange: SetResolveActionForChange::<Identity, OFFSET>,
            GetResolveActionForChangeUnit: GetResolveActionForChangeUnit::<Identity, OFFSET>,
            SetResolveActionForChangeUnit: SetResolveActionForChangeUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChangeConflict as windows_core::Interface>::IID
    }
}
pub trait IChangeUnitException_Impl: Sized {
    fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChangeUnitException {}
impl IChangeUnitException_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IChangeUnitException_Vtbl
    where
        Identity: IChangeUnitException_Impl,
    {
        unsafe extern "system" fn GetItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChangeUnitException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitException_Impl::GetItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChangeUnitException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitException_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IChangeUnitException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, OFFSET>,
            GetClockVector: GetClockVector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChangeUnitException as windows_core::Interface>::IID
    }
}
pub trait IChangeUnitListFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> windows_core::Result<()>;
    fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChangeUnitListFilterInfo {}
impl IChangeUnitListFilterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IChangeUnitListFilterInfo_Vtbl
    where
        Identity: IChangeUnitListFilterInfo_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> windows_core::HRESULT
        where
            Identity: IChangeUnitListFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitListFilterInfo_Impl::Initialize(this, core::mem::transmute_copy(&ppbchangeunitids), core::mem::transmute_copy(&dwchangeunitcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChangeUnitListFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitListFilterInfo_Impl::GetChangeUnitIdCount(this, core::mem::transmute_copy(&pdwchangeunitidcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChangeUnitListFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChangeUnitListFilterInfo_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&dwchangeunitidindex), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Identity, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChangeUnitListFilterInfo as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
pub trait IClockVector_Impl: Sized {
    fn GetClockVectorElements(&self, riid: *const windows_core::GUID, ppienumclockvector: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClockVector {}
impl IClockVector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClockVector_Vtbl
    where
        Identity: IClockVector_Impl,
    {
        unsafe extern "system" fn GetClockVectorElements<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppienumclockvector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVector_Impl::GetClockVectorElements(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppienumclockvector)).into()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVector_Impl::GetClockVectorElementCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClockVectorElements: GetClockVectorElements::<Identity, OFFSET>,
            GetClockVectorElementCount: GetClockVectorElementCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClockVector as windows_core::Interface>::IID
    }
}
pub trait IClockVectorElement_Impl: Sized {
    fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> windows_core::Result<()>;
    fn GetTickCount(&self, pulltickcount: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClockVectorElement {}
impl IClockVectorElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IClockVectorElement_Vtbl
    where
        Identity: IClockVectorElement_Impl,
    {
        unsafe extern "system" fn GetReplicaKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwreplicakey: *mut u32) -> windows_core::HRESULT
        where
            Identity: IClockVectorElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVectorElement_Impl::GetReplicaKey(this, core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn GetTickCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulltickcount: *mut u64) -> windows_core::HRESULT
        where
            Identity: IClockVectorElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClockVectorElement_Impl::GetTickCount(this, core::mem::transmute_copy(&pulltickcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReplicaKey: GetReplicaKey::<Identity, OFFSET>,
            GetTickCount: GetTickCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClockVectorElement as windows_core::Interface>::IID
    }
}
pub trait ICombinedFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> windows_core::Result<()>;
    fn GetFilterInfo(&self, dwfilterindex: u32) -> windows_core::Result<ISyncFilterInfo>;
    fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICombinedFilterInfo {}
impl ICombinedFilterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICombinedFilterInfo_Vtbl
    where
        Identity: ICombinedFilterInfo_Impl,
    {
        unsafe extern "system" fn GetFilterCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfiltercount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICombinedFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICombinedFilterInfo_Impl::GetFilterCount(this, core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICombinedFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICombinedFilterInfo_Impl::GetFilterInfo(this, core::mem::transmute_copy(&dwfilterindex)) {
                Ok(ok__) => {
                    ppifilterinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> windows_core::HRESULT
        where
            Identity: ICombinedFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICombinedFilterInfo_Impl::GetFilterCombinationType(this, core::mem::transmute_copy(&pfiltercombinationtype)).into()
        }
        Self {
            base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, OFFSET>,
            GetFilterInfo: GetFilterInfo::<Identity, OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICombinedFilterInfo as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
pub trait IConstraintConflict_Impl: Sized {
    fn GetDestinationProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetSourceProviderConflictingChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetDestinationProviderOriginalChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetDestinationProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSourceProviderConflictingData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetDestinationProviderOriginalData(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn GetConstraintResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn SetConstraintResolveActionForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::Result<()>;
    fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::Result<()>;
    fn IsTemporary(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IConstraintConflict {}
impl IConstraintConflict_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConstraintConflict_Vtbl
    where
        Identity: IConstraintConflict_Impl,
    {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetSourceProviderConflictingChange(this) {
                Ok(ok__) => {
                    ppconflictingchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginalchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderOriginalChange(this) {
                Ok(ok__) => {
                    pporiginalchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetSourceProviderConflictingData(this) {
                Ok(ok__) => {
                    ppconflictingdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginaldata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConstraintConflict_Impl::GetDestinationProviderOriginalData(this) {
                Ok(ok__) => {
                    pporiginaldata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::GetConstraintResolveActionForChange(this, core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::SetConstraintResolveActionForChange(this, core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::GetConstraintResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::SetConstraintResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::GetConstraintConflictReason(this, core::mem::transmute_copy(&pconstraintconflictreason)).into()
        }
        unsafe extern "system" fn IsTemporary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConstraintConflict_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstraintConflict_Impl::IsTemporary(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, OFFSET>,
            GetDestinationProviderOriginalChange: GetDestinationProviderOriginalChange::<Identity, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, OFFSET>,
            GetDestinationProviderOriginalData: GetDestinationProviderOriginalData::<Identity, OFFSET>,
            GetConstraintResolveActionForChange: GetConstraintResolveActionForChange::<Identity, OFFSET>,
            SetConstraintResolveActionForChange: SetConstraintResolveActionForChange::<Identity, OFFSET>,
            GetConstraintResolveActionForChangeUnit: GetConstraintResolveActionForChangeUnit::<Identity, OFFSET>,
            SetConstraintResolveActionForChangeUnit: SetConstraintResolveActionForChangeUnit::<Identity, OFFSET>,
            GetConstraintConflictReason: GetConstraintConflictReason::<Identity, OFFSET>,
            IsTemporary: IsTemporary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConstraintConflict as windows_core::Interface>::IID
    }
}
pub trait IConstructReplicaKeyMap_Impl: Sized {
    fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IConstructReplicaKeyMap {}
impl IConstructReplicaKeyMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConstructReplicaKeyMap_Vtbl
    where
        Identity: IConstructReplicaKeyMap_Impl,
    {
        unsafe extern "system" fn FindOrAddReplica<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::HRESULT
        where
            Identity: IConstructReplicaKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConstructReplicaKeyMap_Impl::FindOrAddReplica(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindOrAddReplica: FindOrAddReplica::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConstructReplicaKeyMap as windows_core::Interface>::IID
    }
}
pub trait ICoreFragment_Impl: Sized {
    fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> windows_core::Result<()>;
    fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut Option<IClockVector>) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetColumnCount(&self, pcolumncount: *mut u32) -> windows_core::Result<()>;
    fn GetRangeCount(&self, prangecount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreFragment {}
impl ICoreFragment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreFragment_Vtbl
    where
        Identity: ICoreFragment_Impl,
    {
        unsafe extern "system" fn NextColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICoreFragment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::NextColumn(this, core::mem::transmute_copy(&pchangeunitid), core::mem::transmute_copy(&pchangeunitidsize)).into()
        }
        unsafe extern "system" fn NextRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreFragment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::NextRange(this, core::mem::transmute_copy(&pitemid), core::mem::transmute_copy(&pitemidsize), core::mem::transmute_copy(&piclockvector)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreFragment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetColumnCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolumncount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICoreFragment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::GetColumnCount(this, core::mem::transmute_copy(&pcolumncount)).into()
        }
        unsafe extern "system" fn GetRangeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICoreFragment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragment_Impl::GetRangeCount(this, core::mem::transmute_copy(&prangecount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NextColumn: NextColumn::<Identity, OFFSET>,
            NextRange: NextRange::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetRangeCount: GetRangeCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreFragment as windows_core::Interface>::IID
    }
}
pub trait ICoreFragmentInspector_Impl: Sized {
    fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut Option<ICoreFragment>, pfetchedcount: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreFragmentInspector {}
impl ICoreFragmentInspector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICoreFragmentInspector_Vtbl
    where
        Identity: ICoreFragmentInspector_Impl,
    {
        unsafe extern "system" fn NextCoreFragments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut *mut core::ffi::c_void, pfetchedcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ICoreFragmentInspector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragmentInspector_Impl::NextCoreFragments(this, core::mem::transmute_copy(&requestedcount), core::mem::transmute_copy(&ppicorefragments), core::mem::transmute_copy(&pfetchedcount)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICoreFragmentInspector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFragmentInspector_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NextCoreFragments: NextCoreFragments::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreFragmentInspector as windows_core::Interface>::IID
    }
}
pub trait ICustomFilterInfo_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetSyncFilter(&self) -> windows_core::Result<ISyncFilter>;
}
impl windows_core::RuntimeName for ICustomFilterInfo {}
impl ICustomFilterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICustomFilterInfo_Vtbl
    where
        Identity: ICustomFilterInfo_Impl,
    {
        unsafe extern "system" fn GetSyncFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICustomFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICustomFilterInfo_Impl::GetSyncFilter(this) {
                Ok(ok__) => {
                    pisyncfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(), GetSyncFilter: GetSyncFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICustomFilterInfo as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
pub trait IDataRetrieverCallback_Impl: Sized {
    fn LoadChangeDataComplete(&self, punkdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn LoadChangeDataError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDataRetrieverCallback {}
impl IDataRetrieverCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataRetrieverCallback_Vtbl
    where
        Identity: IDataRetrieverCallback_Impl,
    {
        unsafe extern "system" fn LoadChangeDataComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataRetrieverCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataRetrieverCallback_Impl::LoadChangeDataComplete(this, windows_core::from_raw_borrowed(&punkdata)).into()
        }
        unsafe extern "system" fn LoadChangeDataError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IDataRetrieverCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataRetrieverCallback_Impl::LoadChangeDataError(this, core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadChangeDataComplete: LoadChangeDataComplete::<Identity, OFFSET>,
            LoadChangeDataError: LoadChangeDataError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataRetrieverCallback as windows_core::Interface>::IID
    }
}
pub trait IEnumChangeUnitExceptions_Impl: Sized {
    fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut Option<IChangeUnitException>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumChangeUnitExceptions>;
}
impl windows_core::RuntimeName for IEnumChangeUnitExceptions {}
impl IEnumChangeUnitExceptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumChangeUnitExceptions_Vtbl
    where
        Identity: IEnumChangeUnitExceptions_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumChangeUnitExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumChangeUnitExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&ppchangeunitexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT
        where
            Identity: IEnumChangeUnitExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumChangeUnitExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumChangeUnitExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumChangeUnitExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumChangeUnitExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumChangeUnitExceptions_Impl::Clone(this) {
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
        iid == &<IEnumChangeUnitExceptions as windows_core::Interface>::IID
    }
}
pub trait IEnumClockVector_Impl: Sized {
    fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut Option<IClockVectorElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, csyncversions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumClockVector>;
}
impl windows_core::RuntimeName for IEnumClockVector {}
impl IEnumClockVector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumClockVector_Vtbl
    where
        Identity: IEnumClockVector_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumClockVector_Impl::Next(this, core::mem::transmute_copy(&cclockvectorelements), core::mem::transmute_copy(&ppiclockvectorelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, csyncversions: u32) -> windows_core::HRESULT
        where
            Identity: IEnumClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumClockVector_Impl::Skip(this, core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumClockVector_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumClockVector_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
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
        iid == &<IEnumClockVector as windows_core::Interface>::IID
    }
}
pub trait IEnumFeedClockVector_Impl: Sized {
    fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, csyncversions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFeedClockVector>;
}
impl windows_core::RuntimeName for IEnumFeedClockVector {}
impl IEnumFeedClockVector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumFeedClockVector_Vtbl
    where
        Identity: IEnumFeedClockVector_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumFeedClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFeedClockVector_Impl::Next(this, core::mem::transmute_copy(&cclockvectorelements), core::mem::transmute_copy(&ppiclockvectorelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, csyncversions: u32) -> windows_core::HRESULT
        where
            Identity: IEnumFeedClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFeedClockVector_Impl::Skip(this, core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumFeedClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFeedClockVector_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumFeedClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumFeedClockVector_Impl::Clone(this) {
                Ok(ok__) => {
                    ppienum.write(core::mem::transmute(ok__));
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
        iid == &<IEnumFeedClockVector as windows_core::Interface>::IID
    }
}
pub trait IEnumItemIds_Impl: Sized {
    fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumItemIds {}
impl IEnumItemIds_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumItemIds_Vtbl
    where
        Identity: IEnumItemIds_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumItemIds_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumItemIds_Impl::Next(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumItemIds as windows_core::Interface>::IID
    }
}
pub trait IEnumRangeExceptions_Impl: Sized {
    fn Next(&self, cexceptions: u32, pprangeexception: *mut Option<IRangeException>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumRangeExceptions>;
}
impl windows_core::RuntimeName for IEnumRangeExceptions {}
impl IEnumRangeExceptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumRangeExceptions_Vtbl
    where
        Identity: IEnumRangeExceptions_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, pprangeexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumRangeExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRangeExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&pprangeexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT
        where
            Identity: IEnumRangeExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRangeExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumRangeExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumRangeExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumRangeExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumRangeExceptions_Impl::Clone(this) {
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
        iid == &<IEnumRangeExceptions as windows_core::Interface>::IID
    }
}
pub trait IEnumSingleItemExceptions_Impl: Sized {
    fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut Option<ISingleItemException>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cexceptions: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSingleItemExceptions>;
}
impl windows_core::RuntimeName for IEnumSingleItemExceptions {}
impl IEnumSingleItemExceptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSingleItemExceptions_Vtbl
    where
        Identity: IEnumSingleItemExceptions_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSingleItemExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSingleItemExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&ppsingleitemexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSingleItemExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSingleItemExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSingleItemExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSingleItemExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSingleItemExceptions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSingleItemExceptions_Impl::Clone(this) {
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
        iid == &<IEnumSingleItemExceptions as windows_core::Interface>::IID
    }
}
pub trait IEnumSyncChangeUnits_Impl: Sized {
    fn Next(&self, cchanges: u32, ppchangeunit: *mut Option<ISyncChangeUnit>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cchanges: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncChangeUnits>;
}
impl windows_core::RuntimeName for IEnumSyncChangeUnits {}
impl IEnumSyncChangeUnits_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSyncChangeUnits_Vtbl
    where
        Identity: IEnumSyncChangeUnits_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, ppchangeunit: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChangeUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChangeUnits_Impl::Next(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&ppchangeunit), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChangeUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChangeUnits_Impl::Skip(this, core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChangeUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChangeUnits_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChangeUnits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncChangeUnits_Impl::Clone(this) {
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
        iid == &<IEnumSyncChangeUnits as windows_core::Interface>::IID
    }
}
pub trait IEnumSyncChanges_Impl: Sized {
    fn Next(&self, cchanges: u32, ppchange: *mut Option<ISyncChange>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cchanges: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncChanges>;
}
impl windows_core::RuntimeName for IEnumSyncChanges {}
impl IEnumSyncChanges_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSyncChanges_Vtbl
    where
        Identity: IEnumSyncChanges_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, ppchange: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChanges_Impl::Next(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&ppchange), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChanges_Impl::Skip(this, core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncChanges_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncChanges_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncChanges_Impl::Clone(this) {
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
        iid == &<IEnumSyncChanges as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderConfigUIInfos_Impl: Sized {
    fn Next(&self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cfactories: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncProviderConfigUIInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IEnumSyncProviderConfigUIInfos {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderConfigUIInfos_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSyncProviderConfigUIInfos_Vtbl
    where
        Identity: IEnumSyncProviderConfigUIInfos_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderConfigUIInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderConfigUIInfos_Impl::Next(this, core::mem::transmute_copy(&cfactories), core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfactories: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderConfigUIInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderConfigUIInfos_Impl::Skip(this, core::mem::transmute_copy(&cfactories)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderConfigUIInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderConfigUIInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderConfigUIInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncProviderConfigUIInfos_Impl::Clone(this) {
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
        iid == &<IEnumSyncProviderConfigUIInfos as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IEnumSyncProviderInfos_Impl: Sized {
    fn Next(&self, cinstances: u32, ppsyncproviderinfo: *mut Option<ISyncProviderInfo>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cinstances: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSyncProviderInfos>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IEnumSyncProviderInfos {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IEnumSyncProviderInfos_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSyncProviderInfos_Vtbl
    where
        Identity: IEnumSyncProviderInfos_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderInfos_Impl::Next(this, core::mem::transmute_copy(&cinstances), core::mem::transmute_copy(&ppsyncproviderinfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinstances: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderInfos_Impl::Skip(this, core::mem::transmute_copy(&cinstances)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSyncProviderInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSyncProviderInfos_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSyncProviderInfos_Impl::Clone(this) {
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
        iid == &<IEnumSyncProviderInfos as windows_core::Interface>::IID
    }
}
pub trait IFeedClockVector_Impl: Sized + IClockVector_Impl {
    fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> windows_core::Result<()>;
    fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFeedClockVector {}
impl IFeedClockVector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedClockVector_Vtbl
    where
        Identity: IFeedClockVector_Impl,
    {
        unsafe extern "system" fn GetUpdateCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwupdatecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFeedClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVector_Impl::GetUpdateCount(this, core::mem::transmute_copy(&pdwupdatecount)).into()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedClockVector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVector_Impl::IsNoConflictsSpecified(this, core::mem::transmute_copy(&pfisnoconflictsspecified)).into()
        }
        Self {
            base__: IClockVector_Vtbl::new::<Identity, OFFSET>(),
            GetUpdateCount: GetUpdateCount::<Identity, OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedClockVector as windows_core::Interface>::IID || iid == &<IClockVector as windows_core::Interface>::IID
    }
}
pub trait IFeedClockVectorElement_Impl: Sized + IClockVectorElement_Impl {
    fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> windows_core::Result<()>;
    fn GetFlags(&self, pbflags: *mut u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFeedClockVectorElement {}
impl IFeedClockVectorElement_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedClockVectorElement_Vtbl
    where
        Identity: IFeedClockVectorElement_Impl,
    {
        unsafe extern "system" fn GetSyncTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynctime: *mut SYNC_TIME) -> windows_core::HRESULT
        where
            Identity: IFeedClockVectorElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVectorElement_Impl::GetSyncTime(this, core::mem::transmute_copy(&psynctime)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbflags: *mut u8) -> windows_core::HRESULT
        where
            Identity: IFeedClockVectorElement_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedClockVectorElement_Impl::GetFlags(this, core::mem::transmute_copy(&pbflags)).into()
        }
        Self {
            base__: IClockVectorElement_Vtbl::new::<Identity, OFFSET>(),
            GetSyncTime: GetSyncTime::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedClockVectorElement as windows_core::Interface>::IID || iid == &<IClockVectorElement as windows_core::Interface>::IID
    }
}
pub trait IFilterKeyMap_Impl: Sized {
    fn GetCount(&self, pdwcount: *mut u32) -> windows_core::Result<()>;
    fn AddFilter(&self, pisyncfilter: Option<&ISyncFilter>, pdwfilterkey: *mut u32) -> windows_core::Result<()>;
    fn GetFilter(&self, dwfilterkey: u32) -> windows_core::Result<ISyncFilter>;
    fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterKeyMap {}
impl IFilterKeyMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFilterKeyMap_Vtbl
    where
        Identity: IFilterKeyMap_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterKeyMap_Impl::GetCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn AddFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncfilter: *mut core::ffi::c_void, pdwfilterkey: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterKeyMap_Impl::AddFilter(this, windows_core::from_raw_borrowed(&pisyncfilter), core::mem::transmute_copy(&pdwfilterkey)).into()
        }
        unsafe extern "system" fn GetFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFilterKeyMap_Impl::GetFilter(this, core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    ppisyncfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> windows_core::HRESULT
        where
            Identity: IFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterKeyMap_Impl::Serialize(this, core::mem::transmute_copy(&pbfilterkeymap), core::mem::transmute_copy(&pcbfilterkeymap)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            AddFilter: AddFilter::<Identity, OFFSET>,
            GetFilter: GetFilter::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterKeyMap as windows_core::Interface>::IID
    }
}
pub trait IFilterRequestCallback_Impl: Sized {
    fn RequestFilter(&self, pfilter: Option<&windows_core::IUnknown>, filteringtype: FILTERING_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterRequestCallback {}
impl IFilterRequestCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFilterRequestCallback_Vtbl
    where
        Identity: IFilterRequestCallback_Impl,
    {
        unsafe extern "system" fn RequestFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void, filteringtype: FILTERING_TYPE) -> windows_core::HRESULT
        where
            Identity: IFilterRequestCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterRequestCallback_Impl::RequestFilter(this, windows_core::from_raw_borrowed(&pfilter), core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestFilter: RequestFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterRequestCallback as windows_core::Interface>::IID
    }
}
pub trait IFilterTrackingProvider_Impl: Sized {
    fn SpecifyTrackedFilters(&self, pcallback: Option<&IFilterTrackingRequestCallback>) -> windows_core::Result<()>;
    fn AddTrackedFilter(&self, pfilter: Option<&ISyncFilter>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterTrackingProvider {}
impl IFilterTrackingProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFilterTrackingProvider_Vtbl
    where
        Identity: IFilterTrackingProvider_Impl,
    {
        unsafe extern "system" fn SpecifyTrackedFilters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFilterTrackingProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingProvider_Impl::SpecifyTrackedFilters(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn AddTrackedFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFilterTrackingProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingProvider_Impl::AddTrackedFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SpecifyTrackedFilters: SpecifyTrackedFilters::<Identity, OFFSET>,
            AddTrackedFilter: AddTrackedFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterTrackingProvider as windows_core::Interface>::IID
    }
}
pub trait IFilterTrackingRequestCallback_Impl: Sized {
    fn RequestTrackedFilter(&self, pfilter: Option<&ISyncFilter>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterTrackingRequestCallback {}
impl IFilterTrackingRequestCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFilterTrackingRequestCallback_Vtbl
    where
        Identity: IFilterTrackingRequestCallback_Impl,
    {
        unsafe extern "system" fn RequestTrackedFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFilterTrackingRequestCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingRequestCallback_Impl::RequestTrackedFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestTrackedFilter: RequestTrackedFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterTrackingRequestCallback as windows_core::Interface>::IID
    }
}
pub trait IFilterTrackingSyncChangeBuilder_Impl: Sized {
    fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> windows_core::Result<()>;
    fn SetAllChangeUnitsPresentFlag(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFilterTrackingSyncChangeBuilder {}
impl IFilterTrackingSyncChangeBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFilterTrackingSyncChangeBuilder_Vtbl
    where
        Identity: IFilterTrackingSyncChangeBuilder_Impl,
    {
        unsafe extern "system" fn AddFilterChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> windows_core::HRESULT
        where
            Identity: IFilterTrackingSyncChangeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingSyncChangeBuilder_Impl::AddFilterChange(this, core::mem::transmute_copy(&dwfilterkey), core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFilterTrackingSyncChangeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilterTrackingSyncChangeBuilder_Impl::SetAllChangeUnitsPresentFlag(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFilterChange: AddFilterChange::<Identity, OFFSET>,
            SetAllChangeUnitsPresentFlag: SetAllChangeUnitsPresentFlag::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilterTrackingSyncChangeBuilder as windows_core::Interface>::IID
    }
}
pub trait IForgottenKnowledge_Impl: Sized + ISyncKnowledge_Impl {
    fn ForgetToVersion(&self, pknowledge: Option<&ISyncKnowledge>, pversion: *const SYNC_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IForgottenKnowledge {}
impl IForgottenKnowledge_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IForgottenKnowledge_Vtbl
    where
        Identity: IForgottenKnowledge_Impl,
    {
        unsafe extern "system" fn ForgetToVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pversion: *const SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: IForgottenKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IForgottenKnowledge_Impl::ForgetToVersion(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pversion)).into()
        }
        Self { base__: ISyncKnowledge_Vtbl::new::<Identity, OFFSET>(), ForgetToVersion: ForgetToVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IForgottenKnowledge as windows_core::Interface>::IID || iid == &<ISyncKnowledge as windows_core::Interface>::IID
    }
}
pub trait IKnowledgeSyncProvider_Impl: Sized + ISyncProvider_Impl {
    fn BeginSession(&self, role: SYNC_PROVIDER_ROLE, psessionstate: Option<&ISyncSessionState>) -> windows_core::Result<()>;
    fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeBatch(&self, dwbatchsize: u32, psyncknowledge: Option<&ISyncKnowledge>, ppsyncchangebatch: *mut Option<ISyncChangeBatch>, ppunkdataretriever: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetFullEnumerationChangeBatch(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: Option<&ISyncKnowledge>, ppsyncchangebatch: *mut Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ProcessChangeBatch(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Option<&ISyncChangeBatch>, punkdataretriever: Option<&windows_core::IUnknown>, pcallback: Option<&ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::Result<()>;
    fn ProcessFullEnumerationChangeBatch(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Option<&ISyncFullEnumerationChangeBatch>, punkdataretriever: Option<&windows_core::IUnknown>, pcallback: Option<&ISyncCallback>, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::Result<()>;
    fn EndSession(&self, psessionstate: Option<&ISyncSessionState>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKnowledgeSyncProvider {}
impl IKnowledgeSyncProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IKnowledgeSyncProvider_Vtbl
    where
        Identity: IKnowledgeSyncProvider_Impl,
    {
        unsafe extern "system" fn BeginSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::BeginSession(this, core::mem::transmute_copy(&role), windows_core::from_raw_borrowed(&psessionstate)).into()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncknowledge: *mut *mut core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::GetSyncBatchParameters(this, core::mem::transmute_copy(&ppsyncknowledge), core::mem::transmute_copy(&pdwrequestedbatchsize)).into()
        }
        unsafe extern "system" fn GetChangeBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatchsize: u32, psyncknowledge: *mut core::ffi::c_void, ppsyncchangebatch: *mut *mut core::ffi::c_void, ppunkdataretriever: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::GetChangeBatch(this, core::mem::transmute_copy(&dwbatchsize), windows_core::from_raw_borrowed(&psyncknowledge), core::mem::transmute_copy(&ppsyncchangebatch), core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut core::ffi::c_void, ppsyncchangebatch: *mut *mut core::ffi::c_void, ppunkdataretriever: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::GetFullEnumerationChangeBatch(this, core::mem::transmute_copy(&dwbatchsize), core::mem::transmute_copy(&pblowerenumerationbound), windows_core::from_raw_borrowed(&psyncknowledge), core::mem::transmute_copy(&ppsyncchangebatch), core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn ProcessChangeBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut core::ffi::c_void, punkdataretriever: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::ProcessChangeBatch(this, core::mem::transmute_copy(&resolutionpolicy), windows_core::from_raw_borrowed(&psourcechangebatch), windows_core::from_raw_borrowed(&punkdataretriever), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut core::ffi::c_void, punkdataretriever: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::ProcessFullEnumerationChangeBatch(this, core::mem::transmute_copy(&resolutionpolicy), windows_core::from_raw_borrowed(&psourcechangebatch), windows_core::from_raw_borrowed(&punkdataretriever), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn EndSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IKnowledgeSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IKnowledgeSyncProvider_Impl::EndSession(this, windows_core::from_raw_borrowed(&psessionstate)).into()
        }
        Self {
            base__: ISyncProvider_Vtbl::new::<Identity, OFFSET>(),
            BeginSession: BeginSession::<Identity, OFFSET>,
            GetSyncBatchParameters: GetSyncBatchParameters::<Identity, OFFSET>,
            GetChangeBatch: GetChangeBatch::<Identity, OFFSET>,
            GetFullEnumerationChangeBatch: GetFullEnumerationChangeBatch::<Identity, OFFSET>,
            ProcessChangeBatch: ProcessChangeBatch::<Identity, OFFSET>,
            ProcessFullEnumerationChangeBatch: ProcessFullEnumerationChangeBatch::<Identity, OFFSET>,
            EndSession: EndSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKnowledgeSyncProvider as windows_core::Interface>::IID || iid == &<ISyncProvider as windows_core::Interface>::IID
    }
}
pub trait ILoadChangeContext_Impl: Sized {
    fn GetSyncChange(&self) -> windows_core::Result<ISyncChange>;
    fn SetRecoverableErrorOnChange(&self, hrerror: windows_core::HRESULT, perrordata: Option<&IRecoverableErrorData>) -> windows_core::Result<()>;
    fn SetRecoverableErrorOnChangeUnit(&self, hrerror: windows_core::HRESULT, pchangeunit: Option<&ISyncChangeUnit>, perrordata: Option<&IRecoverableErrorData>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ILoadChangeContext {}
impl ILoadChangeContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILoadChangeContext_Vtbl
    where
        Identity: ILoadChangeContext_Impl,
    {
        unsafe extern "system" fn GetSyncChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadChangeContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ILoadChangeContext_Impl::GetSyncChange(this) {
                Ok(ok__) => {
                    ppsyncchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, perrordata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadChangeContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadChangeContext_Impl::SetRecoverableErrorOnChange(this, core::mem::transmute_copy(&hrerror), windows_core::from_raw_borrowed(&perrordata)).into()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, pchangeunit: *mut core::ffi::c_void, perrordata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadChangeContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadChangeContext_Impl::SetRecoverableErrorOnChangeUnit(this, core::mem::transmute_copy(&hrerror), windows_core::from_raw_borrowed(&pchangeunit), windows_core::from_raw_borrowed(&perrordata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSyncChange: GetSyncChange::<Identity, OFFSET>,
            SetRecoverableErrorOnChange: SetRecoverableErrorOnChange::<Identity, OFFSET>,
            SetRecoverableErrorOnChangeUnit: SetRecoverableErrorOnChangeUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoadChangeContext as windows_core::Interface>::IID
    }
}
pub trait IProviderConverter_Impl: Sized {
    fn Initialize(&self, pisyncprovider: Option<&ISyncProvider>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IProviderConverter {}
impl IProviderConverter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProviderConverter_Vtbl
    where
        Identity: IProviderConverter_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProviderConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IProviderConverter_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisyncprovider)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProviderConverter as windows_core::Interface>::IID
    }
}
pub trait IRangeException_Impl: Sized {
    fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRangeException {}
impl IRangeException_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRangeException_Vtbl
    where
        Identity: IRangeException_Impl,
    {
        unsafe extern "system" fn GetClosedRangeStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRangeException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRangeException_Impl::GetClosedRangeStart(this, core::mem::transmute_copy(&pbclosedrangestart), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRangeException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRangeException_Impl::GetClosedRangeEnd(this, core::mem::transmute_copy(&pbclosedrangeend), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRangeException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRangeException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClosedRangeStart: GetClosedRangeStart::<Identity, OFFSET>,
            GetClosedRangeEnd: GetClosedRangeEnd::<Identity, OFFSET>,
            GetClockVector: GetClockVector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeException as windows_core::Interface>::IID
    }
}
pub trait IRecoverableError_Impl: Sized {
    fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> windows_core::Result<()>;
    fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> windows_core::Result<()>;
    fn GetChangeWithRecoverableError(&self) -> windows_core::Result<ISyncChange>;
    fn GetRecoverableErrorDataForChange(&self, phrerror: *mut windows_core::HRESULT, pperrordata: *mut Option<IRecoverableErrorData>) -> windows_core::Result<()>;
    fn GetRecoverableErrorDataForChangeUnit(&self, pchangeunit: Option<&ISyncChangeUnit>, phrerror: *mut windows_core::HRESULT, pperrordata: *mut Option<IRecoverableErrorData>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRecoverableError {}
impl IRecoverableError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRecoverableError_Vtbl
    where
        Identity: IRecoverableError_Impl,
    {
        unsafe extern "system" fn GetStage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> windows_core::HRESULT
        where
            Identity: IRecoverableError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetStage(this, core::mem::transmute_copy(&pstage)).into()
        }
        unsafe extern "system" fn GetProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> windows_core::HRESULT
        where
            Identity: IRecoverableError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetProvider(this, core::mem::transmute_copy(&pproviderrole)).into()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppchangewithrecoverableerror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecoverableError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRecoverableError_Impl::GetChangeWithRecoverableError(this) {
                Ok(ok__) => {
                    ppchangewithrecoverableerror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT, pperrordata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecoverableError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetRecoverableErrorDataForChange(this, core::mem::transmute_copy(&phrerror), core::mem::transmute_copy(&pperrordata)).into()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT, pperrordata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRecoverableError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableError_Impl::GetRecoverableErrorDataForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&phrerror), core::mem::transmute_copy(&pperrordata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStage: GetStage::<Identity, OFFSET>,
            GetProvider: GetProvider::<Identity, OFFSET>,
            GetChangeWithRecoverableError: GetChangeWithRecoverableError::<Identity, OFFSET>,
            GetRecoverableErrorDataForChange: GetRecoverableErrorDataForChange::<Identity, OFFSET>,
            GetRecoverableErrorDataForChangeUnit: GetRecoverableErrorDataForChangeUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecoverableError as windows_core::Interface>::IID
    }
}
pub trait IRecoverableErrorData_Impl: Sized {
    fn Initialize(&self, pcszitemdisplayname: &windows_core::PCWSTR, pcszerrordescription: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetItemDisplayName(&self, pszitemdisplayname: &windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> windows_core::Result<()>;
    fn GetErrorDescription(&self, pszerrordescription: &windows_core::PCWSTR, pccherrordescription: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRecoverableErrorData {}
impl IRecoverableErrorData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRecoverableErrorData_Vtbl
    where
        Identity: IRecoverableErrorData_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszitemdisplayname: windows_core::PCWSTR, pcszerrordescription: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IRecoverableErrorData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableErrorData_Impl::Initialize(this, core::mem::transmute(&pcszitemdisplayname), core::mem::transmute(&pcszerrordescription)).into()
        }
        unsafe extern "system" fn GetItemDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitemdisplayname: windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRecoverableErrorData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableErrorData_Impl::GetItemDisplayName(this, core::mem::transmute(&pszitemdisplayname), core::mem::transmute_copy(&pcchitemdisplayname)).into()
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszerrordescription: windows_core::PCWSTR, pccherrordescription: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRecoverableErrorData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRecoverableErrorData_Impl::GetErrorDescription(this, core::mem::transmute(&pszerrordescription), core::mem::transmute_copy(&pccherrordescription)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetItemDisplayName: GetItemDisplayName::<Identity, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRecoverableErrorData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IRegisteredSyncProvider_Impl: Sized {
    fn Init(&self, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pcontextpropertystore: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn GetInstanceId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IRegisteredSyncProvider {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IRegisteredSyncProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRegisteredSyncProvider_Vtbl
    where
        Identity: IRegisteredSyncProvider_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pcontextpropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredSyncProvider_Impl::Init(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&pguidcontenttype), windows_core::from_raw_borrowed(&pcontextpropertystore)).into()
        }
        unsafe extern "system" fn GetInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRegisteredSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisteredSyncProvider_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    pguidinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRegisteredSyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisteredSyncProvider_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredSyncProvider as windows_core::Interface>::IID
    }
}
pub trait IReplicaKeyMap_Impl: Sized {
    fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::Result<()>;
    fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReplicaKeyMap {}
impl IReplicaKeyMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReplicaKeyMap_Vtbl
    where
        Identity: IReplicaKeyMap_Impl,
    {
        unsafe extern "system" fn LookupReplicaKey<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::HRESULT
        where
            Identity: IReplicaKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReplicaKeyMap_Impl::LookupReplicaKey(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn LookupReplicaId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IReplicaKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReplicaKeyMap_Impl::LookupReplicaId(this, core::mem::transmute_copy(&dwreplicakey), core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> windows_core::HRESULT
        where
            Identity: IReplicaKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReplicaKeyMap_Impl::Serialize(this, core::mem::transmute_copy(&pbreplicakeymap), core::mem::transmute_copy(&pcbreplicakeymap)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LookupReplicaKey: LookupReplicaKey::<Identity, OFFSET>,
            LookupReplicaId: LookupReplicaId::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReplicaKeyMap as windows_core::Interface>::IID
    }
}
pub trait IRequestFilteredSync_Impl: Sized {
    fn SpecifyFilter(&self, pcallback: Option<&IFilterRequestCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRequestFilteredSync {}
impl IRequestFilteredSync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRequestFilteredSync_Vtbl
    where
        Identity: IRequestFilteredSync_Impl,
    {
        unsafe extern "system" fn SpecifyFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRequestFilteredSync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRequestFilteredSync_Impl::SpecifyFilter(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SpecifyFilter: SpecifyFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRequestFilteredSync as windows_core::Interface>::IID
    }
}
pub trait ISingleItemException_Impl: Sized {
    fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClockVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISingleItemException {}
impl ISingleItemException_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISingleItemException_Vtbl
    where
        Identity: ISingleItemException_Impl,
    {
        unsafe extern "system" fn GetItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISingleItemException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISingleItemException_Impl::GetItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISingleItemException_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISingleItemException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, OFFSET>,
            GetClockVector: GetClockVector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISingleItemException as windows_core::Interface>::IID
    }
}
pub trait ISupportFilteredSync_Impl: Sized {
    fn AddFilter(&self, pfilter: Option<&windows_core::IUnknown>, filteringtype: FILTERING_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISupportFilteredSync {}
impl ISupportFilteredSync_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISupportFilteredSync_Vtbl
    where
        Identity: ISupportFilteredSync_Impl,
    {
        unsafe extern "system" fn AddFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void, filteringtype: FILTERING_TYPE) -> windows_core::HRESULT
        where
            Identity: ISupportFilteredSync_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportFilteredSync_Impl::AddFilter(this, windows_core::from_raw_borrowed(&pfilter), core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddFilter: AddFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportFilteredSync as windows_core::Interface>::IID
    }
}
pub trait ISupportLastWriteTime_Impl: Sized {
    fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> windows_core::Result<()>;
    fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISupportLastWriteTime {}
impl ISupportLastWriteTime_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISupportLastWriteTime_Vtbl
    where
        Identity: ISupportLastWriteTime_Impl,
    {
        unsafe extern "system" fn GetItemChangeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> windows_core::HRESULT
        where
            Identity: ISupportLastWriteTime_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportLastWriteTime_Impl::GetItemChangeTime(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pulltimestamp)).into()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> windows_core::HRESULT
        where
            Identity: ISupportLastWriteTime_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISupportLastWriteTime_Impl::GetChangeUnitChangeTime(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pulltimestamp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemChangeTime: GetItemChangeTime::<Identity, OFFSET>,
            GetChangeUnitChangeTime: GetChangeUnitChangeTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISupportLastWriteTime as windows_core::Interface>::IID
    }
}
pub trait ISyncCallback_Impl: Sized {
    fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::Result<()>;
    fn OnChange(&self, psyncchange: Option<&ISyncChange>) -> windows_core::Result<()>;
    fn OnConflict(&self, pconflict: Option<&IChangeConflict>) -> windows_core::Result<()>;
    fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::Result<()>;
    fn OnRecoverableError(&self, precoverableerror: Option<&IRecoverableError>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncCallback {}
impl ISyncCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncCallback_Vtbl
    where
        Identity: ISyncCallback_Impl,
    {
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::HRESULT
        where
            Identity: ISyncCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnProgress(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&syncstage), core::mem::transmute_copy(&dwcompletedwork), core::mem::transmute_copy(&dwtotalwork)).into()
        }
        unsafe extern "system" fn OnChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncchange: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnChange(this, windows_core::from_raw_borrowed(&psyncchange)).into()
        }
        unsafe extern "system" fn OnConflict<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconflict: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnConflict(this, windows_core::from_raw_borrowed(&pconflict)).into()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::HRESULT
        where
            Identity: ISyncCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnFullEnumerationNeeded(this, core::mem::transmute_copy(&pfullenumerationaction)).into()
        }
        unsafe extern "system" fn OnRecoverableError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, precoverableerror: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback_Impl::OnRecoverableError(this, windows_core::from_raw_borrowed(&precoverableerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnChange: OnChange::<Identity, OFFSET>,
            OnConflict: OnConflict::<Identity, OFFSET>,
            OnFullEnumerationNeeded: OnFullEnumerationNeeded::<Identity, OFFSET>,
            OnRecoverableError: OnRecoverableError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncCallback as windows_core::Interface>::IID
    }
}
pub trait ISyncCallback2_Impl: Sized + ISyncCallback_Impl {
    fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::Result<()>;
    fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncCallback2 {}
impl ISyncCallback2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncCallback2_Vtbl
    where
        Identity: ISyncCallback2_Impl,
    {
        unsafe extern "system" fn OnChangeApplied<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::HRESULT
        where
            Identity: ISyncCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback2_Impl::OnChangeApplied(this, core::mem::transmute_copy(&dwchangesapplied), core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        unsafe extern "system" fn OnChangeFailed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::HRESULT
        where
            Identity: ISyncCallback2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncCallback2_Impl::OnChangeFailed(this, core::mem::transmute_copy(&dwchangesapplied), core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        Self {
            base__: ISyncCallback_Vtbl::new::<Identity, OFFSET>(),
            OnChangeApplied: OnChangeApplied::<Identity, OFFSET>,
            OnChangeFailed: OnChangeFailed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncCallback2 as windows_core::Interface>::IID || iid == &<ISyncCallback as windows_core::Interface>::IID
    }
}
pub trait ISyncChange_Impl: Sized {
    fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()>;
    fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()>;
    fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
    fn GetWorkEstimate(&self, pdwwork: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnits(&self) -> windows_core::Result<IEnumSyncChangeUnits>;
    fn GetMadeWithKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn SetWorkEstimate(&self, dwwork: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChange {}
impl ISyncChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChange_Vtbl
    where
        Identity: ISyncChange_Impl,
    {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetOwnerReplicaId(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetRootItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetRootItemId(this, core::mem::transmute_copy(&pbrootitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetChangeVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetCreationVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetCreationVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetWorkEstimate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwork: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::GetWorkEstimate(this, core::mem::transmute_copy(&pdwwork)).into()
        }
        unsafe extern "system" fn GetChangeUnits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChange_Impl::GetChangeUnits(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmadewithknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChange_Impl::GetMadeWithKnowledge(this) {
                Ok(ok__) => {
                    ppmadewithknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChange_Impl::GetLearnedKnowledge(this) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwork: u32) -> windows_core::HRESULT
        where
            Identity: ISyncChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChange_Impl::SetWorkEstimate(this, core::mem::transmute_copy(&dwwork)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, OFFSET>,
            GetRootItemId: GetRootItemId::<Identity, OFFSET>,
            GetChangeVersion: GetChangeVersion::<Identity, OFFSET>,
            GetCreationVersion: GetCreationVersion::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetWorkEstimate: GetWorkEstimate::<Identity, OFFSET>,
            GetChangeUnits: GetChangeUnits::<Identity, OFFSET>,
            GetMadeWithKnowledge: GetMadeWithKnowledge::<Identity, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, OFFSET>,
            SetWorkEstimate: SetWorkEstimate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChange as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn BeginUnorderedGroup(&self) -> windows_core::Result<()>;
    fn EndUnorderedGroup(&self, pmadewithknowledge: Option<&ISyncKnowledge>, fallchangesforknowledge: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AddLoggedConflict(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncChangeBuilder>;
}
impl windows_core::RuntimeName for ISyncChangeBatch {}
impl ISyncChangeBatch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatch_Vtbl
    where
        Identity: ISyncChangeBatch_Impl,
    {
        unsafe extern "system" fn BeginUnorderedGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatch_Impl::BeginUnorderedGroup(this).into()
        }
        unsafe extern "system" fn EndUnorderedGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmadewithknowledge: *mut core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatch_Impl::EndUnorderedGroup(this, windows_core::from_raw_borrowed(&pmadewithknowledge), core::mem::transmute_copy(&fallchangesforknowledge)).into()
        }
        unsafe extern "system" fn AddLoggedConflict<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut core::ffi::c_void, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatch_Impl::AddLoggedConflict(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwworkforchange), windows_core::from_raw_borrowed(&pconflictknowledge)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(),
            BeginUnorderedGroup: BeginUnorderedGroup::<Identity, OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Identity, OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatch as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatch2_Impl: Sized + ISyncChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder>;
    fn AddMergeTombstoneLoggedConflict(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncChangeBuilder>;
}
impl windows_core::RuntimeName for ISyncChangeBatch2 {}
impl ISyncChangeBatch2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatch2_Vtbl
    where
        Identity: ISyncChangeBatch2_Impl,
    {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatch2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatch2_Impl::AddMergeTombstoneMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut core::ffi::c_void, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatch2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatch2_Impl::AddMergeTombstoneLoggedConflict(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange), windows_core::from_raw_borrowed(&pconflictknowledge)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatch_Vtbl::new::<Identity, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatch2 as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID || iid == &<ISyncChangeBatch as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatchAdvanced_Impl: Sized {
    fn GetFilterInfo(&self) -> windows_core::Result<ISyncFilterInfo>;
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> windows_core::Result<ISyncChangeBatch>;
    fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBatchAdvanced {}
impl ISyncChangeBatchAdvanced_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatchAdvanced_Vtbl
    where
        Identity: ISyncChangeBatchAdvanced_Impl,
    {
        unsafe extern "system" fn GetFilterInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilterinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchAdvanced_Impl::GetFilterInfo(this) {
                Ok(ok__) => {
                    ppfilterinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppchangebatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchAdvanced_Impl::ConvertFullEnumerationChangeBatchToRegularChangeBatch(this) {
                Ok(ok__) => {
                    ppchangebatch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchAdvanced_Impl::GetUpperBoundItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchAdvanced_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchAdvanced_Impl::GetBatchLevelKnowledgeShouldBeApplied(this, core::mem::transmute_copy(&pfbatchknowledgeshouldbeapplied)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterInfo: GetFilterInfo::<Identity, OFFSET>,
            ConvertFullEnumerationChangeBatchToRegularChangeBatch: ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Identity, OFFSET>,
            GetUpperBoundItemId: GetUpperBoundItemId::<Identity, OFFSET>,
            GetBatchLevelKnowledgeShouldBeApplied: GetBatchLevelKnowledgeShouldBeApplied::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchAdvanced as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatchBase_Impl: Sized {
    fn GetChangeEnumerator(&self) -> windows_core::Result<IEnumSyncChanges>;
    fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> windows_core::Result<()>;
    fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> windows_core::Result<()>;
    fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> windows_core::Result<()>;
    fn EndOrderedGroup(&self, pbupperbound: *const u8, pmadewithknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder>;
    fn GetLearnedKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetPrerequisiteKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetSourceForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge>;
    fn SetLastBatch(&self) -> windows_core::Result<()>;
    fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> windows_core::Result<()>;
    fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> windows_core::Result<()>;
    fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBatchBase {}
impl ISyncChangeBatchBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatchBase_Vtbl
    where
        Identity: ISyncChangeBatchBase_Impl,
    {
        unsafe extern "system" fn GetChangeEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetChangeEnumerator(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::GetIsLastBatch(this, core::mem::transmute_copy(&pflastbatch)).into()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwworkforbatch: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::GetWorkEstimateForBatch(this, core::mem::transmute_copy(&pdwworkforbatch)).into()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::GetRemainingWorkEstimateForSession(this, core::mem::transmute_copy(&pdwremainingworkforsession)).into()
        }
        unsafe extern "system" fn BeginOrderedGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblowerbound: *const u8) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::BeginOrderedGroup(this, core::mem::transmute_copy(&pblowerbound)).into()
        }
        unsafe extern "system" fn EndOrderedGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::EndOrderedGroup(this, core::mem::transmute_copy(&pbupperbound), windows_core::from_raw_borrowed(&pmadewithknowledge)).into()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::AddItemMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetLearnedKnowledge(this) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprerequisteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetPrerequisiteKnowledge(this) {
                Ok(ok__) => {
                    ppprerequisteknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsourceforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchBase_Impl::GetSourceForgottenKnowledge(this) {
                Ok(ok__) => {
                    ppsourceforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::SetLastBatch(this).into()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwworkforbatch: u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::SetWorkEstimateForBatch(this, core::mem::transmute_copy(&dwworkforbatch)).into()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwremainingworkforsession: u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::SetRemainingWorkEstimateForSession(this, core::mem::transmute_copy(&dwremainingworkforsession)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase_Impl::Serialize(this, core::mem::transmute_copy(&pbchangebatch), core::mem::transmute_copy(&pcbchangebatch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChangeEnumerator: GetChangeEnumerator::<Identity, OFFSET>,
            GetIsLastBatch: GetIsLastBatch::<Identity, OFFSET>,
            GetWorkEstimateForBatch: GetWorkEstimateForBatch::<Identity, OFFSET>,
            GetRemainingWorkEstimateForSession: GetRemainingWorkEstimateForSession::<Identity, OFFSET>,
            BeginOrderedGroup: BeginOrderedGroup::<Identity, OFFSET>,
            EndOrderedGroup: EndOrderedGroup::<Identity, OFFSET>,
            AddItemMetadataToGroup: AddItemMetadataToGroup::<Identity, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, OFFSET>,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, OFFSET>,
            GetSourceForgottenKnowledge: GetSourceForgottenKnowledge::<Identity, OFFSET>,
            SetLastBatch: SetLastBatch::<Identity, OFFSET>,
            SetWorkEstimateForBatch: SetWorkEstimateForBatch::<Identity, OFFSET>,
            SetRemainingWorkEstimateForSession: SetRemainingWorkEstimateForSession::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatchBase2_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBatchBase2 {}
impl ISyncChangeBatchBase2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatchBase2_Vtbl
    where
        Identity: ISyncChangeBatchBase2_Impl,
    {
        unsafe extern "system" fn SerializeWithOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchBase2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchBase2_Impl::SerializeWithOptions(this, core::mem::transmute_copy(&targetformatversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        Self { base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(), SerializeWithOptions: SerializeWithOptions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchBase2 as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatchWithFilterKeyMap_Impl: Sized {
    fn GetFilterKeyMap(&self) -> windows_core::Result<IFilterKeyMap>;
    fn SetFilterKeyMap(&self, pifilterkeymap: Option<&IFilterKeyMap>) -> windows_core::Result<()>;
    fn SetFilterForgottenKnowledge(&self, dwfilterkey: u32, pfilterforgottenknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn GetFilteredReplicaLearnedKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeBatchWithFilterKeyMap {}
impl ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMap_Vtbl
    where
        Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
    {
        unsafe extern "system" fn GetFilterKeyMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifilterkeymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilterKeyMap(this) {
                Ok(ok__) => {
                    ppifilterkeymap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifilterkeymap: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchWithFilterKeyMap_Impl::SetFilterKeyMap(this, windows_core::from_raw_borrowed(&pifilterkeymap)).into()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchWithFilterKeyMap_Impl::SetFilterForgottenKnowledge(this, core::mem::transmute_copy(&dwfilterkey), windows_core::from_raw_borrowed(&pfilterforgottenknowledge)).into()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterKeyMap: GetFilterKeyMap::<Identity, OFFSET>,
            SetFilterKeyMap: SetFilterKeyMap::<Identity, OFFSET>,
            SetFilterForgottenKnowledge: SetFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithFilterKeyMap as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBatchWithPrerequisite_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn SetPrerequisiteKnowledge(&self, pprerequisiteknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn GetLearnedKnowledgeWithPrerequisite(&self, pdestinationknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeBatchWithPrerequisite {}
impl ISyncChangeBatchWithPrerequisite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBatchWithPrerequisite_Vtbl
    where
        Identity: ISyncChangeBatchWithPrerequisite_Impl,
    {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprerequisiteknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithPrerequisite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBatchWithPrerequisite_Impl::SetPrerequisiteKnowledge(this, windows_core::from_raw_borrowed(&pprerequisiteknowledge)).into()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithPrerequisite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithPrerequisite_Impl::GetLearnedKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                Ok(ok__) => {
                    pplearnedwithprerequisiteknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBatchWithPrerequisite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeBatchWithPrerequisite_Impl::GetLearnedForgottenKnowledge(this) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(),
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Identity, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBatchWithPrerequisite as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeBuilder_Impl: Sized {
    fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeBuilder {}
impl ISyncChangeBuilder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeBuilder_Vtbl
    where
        Identity: ISyncChangeBuilder_Impl,
    {
        unsafe extern "system" fn AddChangeUnitMetadata<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncChangeBuilder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeBuilder_Impl::AddChangeUnitMetadata(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pchangeunitversion)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddChangeUnitMetadata: AddChangeUnitMetadata::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeBuilder as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeUnit_Impl: Sized {
    fn GetItemChange(&self) -> windows_core::Result<ISyncChange>;
    fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncChangeUnit {}
impl ISyncChangeUnit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeUnit_Vtbl
    where
        Identity: ISyncChangeUnit_Impl,
    {
        unsafe extern "system" fn GetItemChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeUnit_Impl::GetItemChange(this) {
                Ok(ok__) => {
                    ppsyncchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeUnit_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncChangeUnit_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeUnit_Impl::GetChangeUnitVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemChange: GetItemChange::<Identity, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, OFFSET>,
            GetChangeUnitVersion: GetChangeUnitVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeUnit as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeWithFilterKeyMap_Impl: Sized {
    fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> windows_core::Result<()>;
    fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> windows_core::Result<()>;
    fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledge(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(&self, pdestinationknowledge: Option<&ISyncKnowledge>, pnewmoveins: Option<&IEnumItemIds>, dwfilterkey: u32) -> windows_core::Result<ISyncKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeWithFilterKeyMap {}
impl ISyncChangeWithFilterKeyMap_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeWithFilterKeyMap_Vtbl
    where
        Identity: ISyncChangeWithFilterKeyMap_Impl,
    {
        unsafe extern "system" fn GetFilterCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfiltercount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeWithFilterKeyMap_Impl::GetFilterCount(this, core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeWithFilterKeyMap_Impl::GetFilterChange(this, core::mem::transmute_copy(&dwfilterkey), core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncChangeWithFilterKeyMap_Impl::GetAllChangeUnitsPresentFlag(this, core::mem::transmute_copy(&pfallchangeunitspresent)).into()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilterForgottenKnowledge(this, core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    ppifilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithFilterKeyMap_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    pplearnedfilterforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, OFFSET>,
            GetFilterChange: GetFilterChange::<Identity, OFFSET>,
            GetAllChangeUnitsPresentFlag: GetAllChangeUnitsPresentFlag::<Identity, OFFSET>,
            GetFilterForgottenKnowledge: GetFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeWithFilterKeyMap as windows_core::Interface>::IID
    }
}
pub trait ISyncChangeWithPrerequisite_Impl: Sized {
    fn GetPrerequisiteKnowledge(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedKnowledgeWithPrerequisite(&self, pdestinationknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
}
impl windows_core::RuntimeName for ISyncChangeWithPrerequisite {}
impl ISyncChangeWithPrerequisite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncChangeWithPrerequisite_Vtbl
    where
        Identity: ISyncChangeWithPrerequisite_Impl,
    {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprerequisiteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithPrerequisite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithPrerequisite_Impl::GetPrerequisiteKnowledge(this) {
                Ok(ok__) => {
                    ppprerequisiteknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncChangeWithPrerequisite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncChangeWithPrerequisite_Impl::GetLearnedKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                Ok(ok__) => {
                    pplearnedknowledgewithprerequisite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncChangeWithPrerequisite as windows_core::Interface>::IID
    }
}
pub trait ISyncConstraintCallback_Impl: Sized {
    fn OnConstraintConflict(&self, pconflict: Option<&IConstraintConflict>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncConstraintCallback {}
impl ISyncConstraintCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncConstraintCallback_Vtbl
    where
        Identity: ISyncConstraintCallback_Impl,
    {
        unsafe extern "system" fn OnConstraintConflict<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconflict: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncConstraintCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncConstraintCallback_Impl::OnConstraintConflict(this, windows_core::from_raw_borrowed(&pconflict)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConstraintConflict: OnConstraintConflict::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncConstraintCallback as windows_core::Interface>::IID
    }
}
pub trait ISyncDataConverter_Impl: Sized {
    fn ConvertDataRetrieverFromProviderFormat(&self, punkdataretrieverin: Option<&windows_core::IUnknown>, penumsyncchanges: Option<&IEnumSyncChanges>) -> windows_core::Result<windows_core::IUnknown>;
    fn ConvertDataRetrieverToProviderFormat(&self, punkdataretrieverin: Option<&windows_core::IUnknown>, penumsyncchanges: Option<&IEnumSyncChanges>) -> windows_core::Result<windows_core::IUnknown>;
    fn ConvertDataFromProviderFormat(&self, pdatacontext: Option<&ILoadChangeContext>, punkdatain: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn ConvertDataToProviderFormat(&self, pdatacontext: Option<&ILoadChangeContext>, punkdataout: Option<&windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ISyncDataConverter {}
impl ISyncDataConverter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncDataConverter_Vtbl
    where
        Identity: ISyncDataConverter_Impl,
    {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdataretrieverin: *mut core::ffi::c_void, penumsyncchanges: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncDataConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataRetrieverFromProviderFormat(this, windows_core::from_raw_borrowed(&punkdataretrieverin), windows_core::from_raw_borrowed(&penumsyncchanges)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdataretrieverin: *mut core::ffi::c_void, penumsyncchanges: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncDataConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataRetrieverToProviderFormat(this, windows_core::from_raw_borrowed(&punkdataretrieverin), windows_core::from_raw_borrowed(&penumsyncchanges)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatacontext: *mut core::ffi::c_void, punkdatain: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncDataConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataFromProviderFormat(this, windows_core::from_raw_borrowed(&pdatacontext), windows_core::from_raw_borrowed(&punkdatain)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatacontext: *mut core::ffi::c_void, punkdataout: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncDataConverter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncDataConverter_Impl::ConvertDataToProviderFormat(this, windows_core::from_raw_borrowed(&pdatacontext), windows_core::from_raw_borrowed(&punkdataout)) {
                Ok(ok__) => {
                    ppunkdataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConvertDataRetrieverFromProviderFormat: ConvertDataRetrieverFromProviderFormat::<Identity, OFFSET>,
            ConvertDataRetrieverToProviderFormat: ConvertDataRetrieverToProviderFormat::<Identity, OFFSET>,
            ConvertDataFromProviderFormat: ConvertDataFromProviderFormat::<Identity, OFFSET>,
            ConvertDataToProviderFormat: ConvertDataToProviderFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncDataConverter as windows_core::Interface>::IID
    }
}
pub trait ISyncFilter_Impl: Sized {
    fn IsIdentical(&self, psyncfilter: Option<&ISyncFilter>) -> windows_core::Result<()>;
    fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFilter {}
impl ISyncFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFilter_Vtbl
    where
        Identity: ISyncFilter_Impl,
    {
        unsafe extern "system" fn IsIdentical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncfilter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilter_Impl::IsIdentical(this, windows_core::from_raw_borrowed(&psyncfilter)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilter_Impl::Serialize(this, core::mem::transmute_copy(&pbsyncfilter), core::mem::transmute_copy(&pcbsyncfilter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsIdentical: IsIdentical::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilter as windows_core::Interface>::IID
    }
}
pub trait ISyncFilterDeserializer_Impl: Sized {
    fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> windows_core::Result<ISyncFilter>;
}
impl windows_core::RuntimeName for ISyncFilterDeserializer {}
impl ISyncFilterDeserializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFilterDeserializer_Vtbl
    where
        Identity: ISyncFilterDeserializer_Impl,
    {
        unsafe extern "system" fn DeserializeSyncFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncFilterDeserializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFilterDeserializer_Impl::DeserializeSyncFilter(this, core::mem::transmute_copy(&pbsyncfilter), core::mem::transmute_copy(&dwcbsyncfilter)) {
                Ok(ok__) => {
                    ppisyncfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeserializeSyncFilter: DeserializeSyncFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilterDeserializer as windows_core::Interface>::IID
    }
}
pub trait ISyncFilterInfo_Impl: Sized {
    fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFilterInfo {}
impl ISyncFilterInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFilterInfo_Vtbl
    where
        Identity: ISyncFilterInfo_Impl,
    {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncFilterInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilterInfo_Impl::Serialize(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pcbbuffer)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Serialize: Serialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
pub trait ISyncFilterInfo2_Impl: Sized + ISyncFilterInfo_Impl {
    fn GetFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFilterInfo2 {}
impl ISyncFilterInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFilterInfo2_Vtbl
    where
        Identity: ISyncFilterInfo2_Impl,
    {
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncFilterInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFilterInfo2_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: ISyncFilterInfo_Vtbl::new::<Identity, OFFSET>(), GetFlags: GetFlags::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFilterInfo2 as windows_core::Interface>::IID || iid == &<ISyncFilterInfo as windows_core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChange_Impl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetLearnedForgottenKnowledge(&self) -> windows_core::Result<IForgottenKnowledge>;
}
impl windows_core::RuntimeName for ISyncFullEnumerationChange {}
impl ISyncFullEnumerationChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFullEnumerationChange_Vtbl
    where
        Identity: ISyncFullEnumerationChange_Impl,
    {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncFullEnumerationChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChange_Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                Ok(ok__) => {
                    pplearnedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncFullEnumerationChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChange_Impl::GetLearnedForgottenKnowledge(this) {
                Ok(ok__) => {
                    pplearnedforgottenknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChange as windows_core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChangeBatch_Impl: Sized + ISyncChangeBatchBase_Impl {
    fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> windows_core::Result<ISyncKnowledge>;
    fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncFullEnumerationChangeBatch {}
impl ISyncFullEnumerationChangeBatch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch_Vtbl
    where
        Identity: ISyncFullEnumerationChangeBatch_Impl,
    {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncFullEnumerationChangeBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChangeBatch_Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                Ok(ok__) => {
                    pplearnedknowledgeafterrecoverycomplete.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncFullEnumerationChangeBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFullEnumerationChangeBatch_Impl::GetClosedLowerBoundItemId(this, core::mem::transmute_copy(&pbclosedlowerbounditemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncFullEnumerationChangeBatch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncFullEnumerationChangeBatch_Impl::GetClosedUpperBoundItemId(this, core::mem::transmute_copy(&pbclosedupperbounditemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Identity, OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID
    }
}
pub trait ISyncFullEnumerationChangeBatch2_Impl: Sized + ISyncFullEnumerationChangeBatch_Impl {
    fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> windows_core::Result<ISyncChangeBuilder>;
}
impl windows_core::RuntimeName for ISyncFullEnumerationChangeBatch2 {}
impl ISyncFullEnumerationChangeBatch2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2_Vtbl
    where
        Identity: ISyncFullEnumerationChangeBatch2_Impl,
    {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncFullEnumerationChangeBatch2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncFullEnumerationChangeBatch2_Impl::AddMergeTombstoneMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    ppchangebuilder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncFullEnumerationChangeBatch_Vtbl::new::<Identity, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncFullEnumerationChangeBatch2 as windows_core::Interface>::IID || iid == &<ISyncChangeBatchBase as windows_core::Interface>::IID || iid == &<ISyncFullEnumerationChangeBatch as windows_core::Interface>::IID
    }
}
pub trait ISyncKnowledge_Impl: Sized {
    fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
    fn Serialize(&self, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> windows_core::Result<()>;
    fn SetLocalTickCount(&self, ulltickcount: u64) -> windows_core::Result<()>;
    fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::Result<()>;
    fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::Result<()>;
    fn GetScopeVector(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetReplicaKeyMap(&self) -> windows_core::Result<IReplicaKeyMap>;
    fn Clone(&self) -> windows_core::Result<ISyncKnowledge>;
    fn ConvertVersion(&self, pknowledgein: Option<&ISyncKnowledge>, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> windows_core::Result<()>;
    fn MapRemoteToLocal(&self, premoteknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn Union(&self, pknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn ProjectOntoItem(&self, pbitemid: *const u8) -> windows_core::Result<ISyncKnowledge>;
    fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<ISyncKnowledge>;
    fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> windows_core::Result<ISyncKnowledge>;
    fn ExcludeItem(&self, pbitemid: *const u8) -> windows_core::Result<()>;
    fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<()>;
    fn ContainsKnowledge(&self, pknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> windows_core::Result<()>;
    fn GetRangeExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSingleItemExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetChangeUnitExceptions(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVersion(&self, pdwversion: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncKnowledge {}
impl ISyncKnowledge_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncKnowledge_Vtbl
    where
        Identity: ISyncKnowledge_Impl,
    {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetOwnerReplicaId(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::Serialize(this, core::mem::transmute_copy(&fserializereplicakeymap), core::mem::transmute_copy(&pbknowledge), core::mem::transmute_copy(&pcbknowledge)).into()
        }
        unsafe extern "system" fn SetLocalTickCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulltickcount: u64) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::SetLocalTickCount(this, core::mem::transmute_copy(&ulltickcount)).into()
        }
        unsafe extern "system" fn ContainsChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ContainsChange(this, core::mem::transmute_copy(&pbversionownerreplicaid), core::mem::transmute_copy(&pgiditemid), core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn ContainsChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ContainsChangeUnit(this, core::mem::transmute_copy(&pbversionownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn GetScopeVector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetScopeVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreplicakeymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::GetReplicaKeyMap(this) {
                Ok(ok__) => {
                    ppreplicakeymap.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclonedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::Clone(this) {
                Ok(ok__) => {
                    ppclonedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledgein: *mut core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ConvertVersion(this, windows_core::from_raw_borrowed(&pknowledgein), core::mem::transmute_copy(&pbcurrentownerid), core::mem::transmute_copy(&pversionin), core::mem::transmute_copy(&pbnewownerid), core::mem::transmute_copy(&pcbidsize), core::mem::transmute_copy(&pversionout)).into()
        }
        unsafe extern "system" fn MapRemoteToLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteknowledge: *mut core::ffi::c_void, ppmappedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::MapRemoteToLocal(this, windows_core::from_raw_borrowed(&premoteknowledge)) {
                Ok(ok__) => {
                    ppmappedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::Union(this, windows_core::from_raw_borrowed(&pknowledge)).into()
        }
        unsafe extern "system" fn ProjectOntoItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::ProjectOntoItem(this, core::mem::transmute_copy(&pbitemid)) {
                Ok(ok__) => {
                    ppknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::ProjectOntoChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)) {
                Ok(ok__) => {
                    ppknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge_Impl::ProjectOntoRange(this, core::mem::transmute_copy(&psrngsyncrange)) {
                Ok(ok__) => {
                    ppknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ExcludeItem(this, core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ExcludeChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ContainsKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::ContainsKnowledge(this, windows_core::from_raw_borrowed(&pknowledge)).into()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::FindMinTickCountForReplica(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pullreplicatickcount)).into()
        }
        unsafe extern "system" fn GetRangeExceptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetRangeExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetSingleItemExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetChangeUnitExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::FindClockVectorForItem(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::FindClockVectorForChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge_Impl::GetVersion(this, core::mem::transmute_copy(&pdwversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            SetLocalTickCount: SetLocalTickCount::<Identity, OFFSET>,
            ContainsChange: ContainsChange::<Identity, OFFSET>,
            ContainsChangeUnit: ContainsChangeUnit::<Identity, OFFSET>,
            GetScopeVector: GetScopeVector::<Identity, OFFSET>,
            GetReplicaKeyMap: GetReplicaKeyMap::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            ConvertVersion: ConvertVersion::<Identity, OFFSET>,
            MapRemoteToLocal: MapRemoteToLocal::<Identity, OFFSET>,
            Union: Union::<Identity, OFFSET>,
            ProjectOntoItem: ProjectOntoItem::<Identity, OFFSET>,
            ProjectOntoChangeUnit: ProjectOntoChangeUnit::<Identity, OFFSET>,
            ProjectOntoRange: ProjectOntoRange::<Identity, OFFSET>,
            ExcludeItem: ExcludeItem::<Identity, OFFSET>,
            ExcludeChangeUnit: ExcludeChangeUnit::<Identity, OFFSET>,
            ContainsKnowledge: ContainsKnowledge::<Identity, OFFSET>,
            FindMinTickCountForReplica: FindMinTickCountForReplica::<Identity, OFFSET>,
            GetRangeExceptions: GetRangeExceptions::<Identity, OFFSET>,
            GetSingleItemExceptions: GetSingleItemExceptions::<Identity, OFFSET>,
            GetChangeUnitExceptions: GetChangeUnitExceptions::<Identity, OFFSET>,
            FindClockVectorForItem: FindClockVectorForItem::<Identity, OFFSET>,
            FindClockVectorForChangeUnit: FindClockVectorForChangeUnit::<Identity, OFFSET>,
            GetVersion: GetVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncKnowledge as windows_core::Interface>::IID
    }
}
pub trait ISyncKnowledge2_Impl: Sized + ISyncKnowledge_Impl {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> windows_core::Result<ISyncKnowledge2>;
    fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::Result<()>;
    fn GetLowestUncontainedId(&self, pisyncknowledge: Option<&ISyncKnowledge2>, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()>;
    fn GetInspector(&self, riid: *const windows_core::GUID, ppiinspector: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> windows_core::Result<()>;
    fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> windows_core::Result<()>;
    fn ContainsKnowledgeForItem(&self, pknowledge: Option<&ISyncKnowledge>, pbitemid: *const u8) -> windows_core::Result<()>;
    fn ContainsKnowledgeForChangeUnit(&self, pknowledge: Option<&ISyncKnowledge>, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::Result<()>;
    fn ProjectOntoKnowledgeWithPrerequisite(&self, pprerequisiteknowledge: Option<&ISyncKnowledge>, ptemplateknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn Complement(&self, psyncknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<ISyncKnowledge>;
    fn IntersectsWithKnowledge(&self, psyncknowledge: Option<&ISyncKnowledge>) -> windows_core::Result<()>;
    fn GetKnowledgeCookie(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn CompareToKnowledgeCookie(&self, pknowledgecookie: Option<&windows_core::IUnknown>, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncKnowledge2 {}
impl ISyncKnowledge2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncKnowledge2_Vtbl
    where
        Identity: ISyncKnowledge2_Impl,
    {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::ProjectOntoColumnSet(this, core::mem::transmute_copy(&ppcolumns), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    ppiknowledgeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::SerializeWithOptions(this, core::mem::transmute_copy(&targetformatversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncknowledge: *mut core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetLowestUncontainedId(this, windows_core::from_raw_borrowed(&pisyncknowledge), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        unsafe extern "system" fn GetInspector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppiinspector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetInspector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppiinspector)).into()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetMinimumSupportedVersion(this, core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::GetStatistics(this, core::mem::transmute_copy(&which), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pbitemid: *const u8) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::ContainsKnowledgeForItem(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::ContainsKnowledgeForChangeUnit(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprerequisiteknowledge: *mut core::ffi::c_void, ptemplateknowledge: *mut core::ffi::c_void, ppprojectedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::ProjectOntoKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pprerequisiteknowledge), windows_core::from_raw_borrowed(&ptemplateknowledge)) {
                Ok(ok__) => {
                    ppprojectedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncknowledge: *mut core::ffi::c_void, ppcomplementedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::Complement(this, windows_core::from_raw_borrowed(&psyncknowledge)) {
                Ok(ok__) => {
                    ppcomplementedknowledge.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::IntersectsWithKnowledge(this, windows_core::from_raw_borrowed(&psyncknowledge)).into()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppknowledgecookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncKnowledge2_Impl::GetKnowledgeCookie(this) {
                Ok(ok__) => {
                    ppknowledgecookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledgecookie: *mut core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::HRESULT
        where
            Identity: ISyncKnowledge2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncKnowledge2_Impl::CompareToKnowledgeCookie(this, windows_core::from_raw_borrowed(&pknowledgecookie), core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base__: ISyncKnowledge_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, OFFSET>,
            ProjectOntoColumnSet: ProjectOntoColumnSet::<Identity, OFFSET>,
            SerializeWithOptions: SerializeWithOptions::<Identity, OFFSET>,
            GetLowestUncontainedId: GetLowestUncontainedId::<Identity, OFFSET>,
            GetInspector: GetInspector::<Identity, OFFSET>,
            GetMinimumSupportedVersion: GetMinimumSupportedVersion::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            ContainsKnowledgeForItem: ContainsKnowledgeForItem::<Identity, OFFSET>,
            ContainsKnowledgeForChangeUnit: ContainsKnowledgeForChangeUnit::<Identity, OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite: ProjectOntoKnowledgeWithPrerequisite::<Identity, OFFSET>,
            Complement: Complement::<Identity, OFFSET>,
            IntersectsWithKnowledge: IntersectsWithKnowledge::<Identity, OFFSET>,
            GetKnowledgeCookie: GetKnowledgeCookie::<Identity, OFFSET>,
            CompareToKnowledgeCookie: CompareToKnowledgeCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncKnowledge2 as windows_core::Interface>::IID || iid == &<ISyncKnowledge as windows_core::Interface>::IID
    }
}
pub trait ISyncMergeTombstoneChange_Impl: Sized {
    fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncMergeTombstoneChange {}
impl ISyncMergeTombstoneChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncMergeTombstoneChange_Vtbl
    where
        Identity: ISyncMergeTombstoneChange_Impl,
    {
        unsafe extern "system" fn GetWinnerItemId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncMergeTombstoneChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncMergeTombstoneChange_Impl::GetWinnerItemId(this, core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWinnerItemId: GetWinnerItemId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncMergeTombstoneChange as windows_core::Interface>::IID
    }
}
pub trait ISyncProvider_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncProvider {}
impl ISyncProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncProvider_Vtbl
    where
        Identity: ISyncProvider_Impl,
    {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: ISyncProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProvider_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdParameters: GetIdParameters::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderConfigUI_Impl: Sized {
    fn Init(&self, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pconfigurationproperties: Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> windows_core::Result<()>;
    fn GetRegisteredProperties(&self) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateAndRegisterNewSyncProvider(&self, hwndparent: super::super::Foundation::HWND, punkcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<ISyncProviderInfo>;
    fn ModifySyncProvider(&self, hwndparent: super::super::Foundation::HWND, punkcontext: Option<&windows_core::IUnknown>, pproviderinfo: Option<&ISyncProviderInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISyncProviderConfigUI {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncProviderConfigUI_Vtbl
    where
        Identity: ISyncProviderConfigUI_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pconfigurationproperties: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderConfigUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderConfigUI_Impl::Init(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&pguidcontenttype), windows_core::from_raw_borrowed(&pconfigurationproperties)).into()
        }
        unsafe extern "system" fn GetRegisteredProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconfiguiproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderConfigUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderConfigUI_Impl::GetRegisteredProperties(this) {
                Ok(ok__) => {
                    ppconfiguiproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut core::ffi::c_void, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderConfigUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderConfigUI_Impl::CreateAndRegisterNewSyncProvider(this, core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&punkcontext)) {
                Ok(ok__) => {
                    ppproviderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut core::ffi::c_void, pproviderinfo: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderConfigUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderConfigUI_Impl::ModifySyncProvider(this, core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&punkcontext), windows_core::from_raw_borrowed(&pproviderinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetRegisteredProperties: GetRegisteredProperties::<Identity, OFFSET>,
            CreateAndRegisterNewSyncProvider: CreateAndRegisterNewSyncProvider::<Identity, OFFSET>,
            ModifySyncProvider: ModifySyncProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderConfigUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderConfigUIInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> windows_core::Result<ISyncProviderConfigUI>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISyncProviderConfigUIInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUIInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncProviderConfigUIInfo_Vtbl
    where
        Identity: ISyncProviderConfigUIInfo_Impl,
    {
        unsafe extern "system" fn GetSyncProviderConfigUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderConfigUIInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderConfigUIInfo_Impl::GetSyncProviderConfigUI(this, core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppsyncproviderconfigui.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, OFFSET>(),
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderConfigUIInfo as windows_core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderInfo_Impl: Sized + super::super::UI::Shell::PropertiesSystem::IPropertyStore_Impl {
    fn GetSyncProvider(&self, dwclscontext: u32) -> windows_core::Result<IRegisteredSyncProvider>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISyncProviderInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncProviderInfo_Vtbl
    where
        Identity: ISyncProviderInfo_Impl,
    {
        unsafe extern "system" fn GetSyncProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderInfo_Impl::GetSyncProvider(this, core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppsyncprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, OFFSET>(),
            GetSyncProvider: GetSyncProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderInfo as windows_core::Interface>::IID || iid == &<super::super::UI::Shell::PropertiesSystem::IPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderRegistration_Impl: Sized {
    fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> windows_core::Result<ISyncProviderConfigUIInfo>;
    fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: *const windows_core::GUID, dwsupportedarchitecture: u32) -> windows_core::Result<IEnumSyncProviderConfigUIInfos>;
    fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> windows_core::Result<ISyncProviderInfo>;
    fn UnregisterSyncProvider(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderConfigUIInfo>;
    fn EnumerateSyncProviders(&self, pguidcontenttype: *const windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const windows_core::GUID, dwsupportedarchitecture: u32) -> windows_core::Result<IEnumSyncProviderInfos>;
    fn GetSyncProviderInfo(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderInfo>;
    fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32) -> windows_core::Result<IRegisteredSyncProvider>;
    fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<ISyncProviderConfigUIInfo>;
    fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32) -> windows_core::Result<ISyncProviderConfigUI>;
    fn GetSyncProviderState(&self, pguidinstanceid: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn SetSyncProviderState(&self, pguidinstanceid: *const windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> windows_core::Result<()>;
    fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn RevokeEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetChange(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<ISyncRegistrationChange>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISyncProviderRegistration {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderRegistration_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncProviderRegistration_Vtbl
    where
        Identity: ISyncProviderRegistration_Impl,
    {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::CreateSyncProviderConfigUIRegistrationInstance(this, core::mem::transmute_copy(&pconfiguiconfig)) {
                Ok(ok__) => {
                    ppconfiguiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::UnregisterSyncProviderConfigUI(this, core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontenttype: *const windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::EnumerateSyncProviderConfigUIs(this, core::mem::transmute_copy(&pguidcontenttype), core::mem::transmute_copy(&dwsupportedarchitecture)) {
                Ok(ok__) => {
                    ppenumsyncproviderconfiguiinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::CreateSyncProviderRegistrationInstance(this, core::mem::transmute_copy(&pproviderconfiguration)) {
                Ok(ok__) => {
                    ppproviderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::UnregisterSyncProvider(this, core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidproviderinstanceid: *const windows_core::GUID, ppproviderconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIInfoforProvider(this, core::mem::transmute_copy(&pguidproviderinstanceid)) {
                Ok(ok__) => {
                    ppproviderconfiguiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontenttype: *const windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::EnumerateSyncProviders(this, core::mem::transmute_copy(&pguidcontenttype), core::mem::transmute_copy(&dwstateflagstofiltermask), core::mem::transmute_copy(&dwstateflagstofilter), core::mem::transmute_copy(&refproviderclsid), core::mem::transmute_copy(&dwsupportedarchitecture)) {
                Ok(ok__) => {
                    ppenumsyncproviderinfos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderInfo(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    ppproviderinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderFromInstanceId(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppsyncprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, ppconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIInfo(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    ppconfiguiinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32, ppconfigui: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIFromInstanceId(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    ppconfigui.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pdwstateflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetSyncProviderState(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    pdwstateflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::SetSyncProviderState(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwstateflagsmask), core::mem::transmute_copy(&dwstateflags)).into()
        }
        unsafe extern "system" fn RegisterForEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::RegisterForEvent(this, core::mem::transmute_copy(&phevent)).into()
        }
        unsafe extern "system" fn RevokeEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncProviderRegistration_Impl::RevokeEvent(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncProviderRegistration_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncProviderRegistration_Impl::GetChange(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    ppchange.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSyncProviderConfigUIRegistrationInstance: CreateSyncProviderConfigUIRegistrationInstance::<Identity, OFFSET>,
            UnregisterSyncProviderConfigUI: UnregisterSyncProviderConfigUI::<Identity, OFFSET>,
            EnumerateSyncProviderConfigUIs: EnumerateSyncProviderConfigUIs::<Identity, OFFSET>,
            CreateSyncProviderRegistrationInstance: CreateSyncProviderRegistrationInstance::<Identity, OFFSET>,
            UnregisterSyncProvider: UnregisterSyncProvider::<Identity, OFFSET>,
            GetSyncProviderConfigUIInfoforProvider: GetSyncProviderConfigUIInfoforProvider::<Identity, OFFSET>,
            EnumerateSyncProviders: EnumerateSyncProviders::<Identity, OFFSET>,
            GetSyncProviderInfo: GetSyncProviderInfo::<Identity, OFFSET>,
            GetSyncProviderFromInstanceId: GetSyncProviderFromInstanceId::<Identity, OFFSET>,
            GetSyncProviderConfigUIInfo: GetSyncProviderConfigUIInfo::<Identity, OFFSET>,
            GetSyncProviderConfigUIFromInstanceId: GetSyncProviderConfigUIFromInstanceId::<Identity, OFFSET>,
            GetSyncProviderState: GetSyncProviderState::<Identity, OFFSET>,
            SetSyncProviderState: SetSyncProviderState::<Identity, OFFSET>,
            RegisterForEvent: RegisterForEvent::<Identity, OFFSET>,
            RevokeEvent: RevokeEvent::<Identity, OFFSET>,
            GetChange: GetChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncProviderRegistration as windows_core::Interface>::IID
    }
}
pub trait ISyncRegistrationChange_Impl: Sized {
    fn GetEvent(&self) -> windows_core::Result<SYNC_REGISTRATION_EVENT>;
    fn GetInstanceId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for ISyncRegistrationChange {}
impl ISyncRegistrationChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncRegistrationChange_Vtbl
    where
        Identity: ISyncRegistrationChange_Impl,
    {
        unsafe extern "system" fn GetEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> windows_core::HRESULT
        where
            Identity: ISyncRegistrationChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncRegistrationChange_Impl::GetEvent(this) {
                Ok(ok__) => {
                    psreevent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISyncRegistrationChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncRegistrationChange_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    pguidinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncRegistrationChange as windows_core::Interface>::IID
    }
}
pub trait ISyncSessionExtendedErrorInfo_Impl: Sized {
    fn GetSyncProviderWithError(&self) -> windows_core::Result<ISyncProvider>;
}
impl windows_core::RuntimeName for ISyncSessionExtendedErrorInfo {}
impl ISyncSessionExtendedErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncSessionExtendedErrorInfo_Vtbl
    where
        Identity: ISyncSessionExtendedErrorInfo_Impl,
    {
        unsafe extern "system" fn GetSyncProviderWithError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproviderwitherror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISyncSessionExtendedErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISyncSessionExtendedErrorInfo_Impl::GetSyncProviderWithError(this) {
                Ok(ok__) => {
                    ppproviderwitherror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSyncProviderWithError: GetSyncProviderWithError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncSessionExtendedErrorInfo as windows_core::Interface>::IID
    }
}
pub trait ISyncSessionState_Impl: Sized {
    fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> windows_core::Result<()>;
    fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> windows_core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> windows_core::Result<()>;
    fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> windows_core::Result<()>;
    fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> windows_core::Result<()>;
    fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncSessionState {}
impl ISyncSessionState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncSessionState_Vtbl
    where
        Identity: ISyncSessionState_Impl,
    {
        unsafe extern "system" fn IsCanceled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::IsCanceled(this, core::mem::transmute_copy(&pfiscanceled)).into()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::GetInfoForChangeApplication(this, core::mem::transmute_copy(&pbchangeapplierinfo), core::mem::transmute_copy(&pcbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::LoadInfoFromChangeApplication(this, core::mem::transmute_copy(&pbchangeapplierinfo), core::mem::transmute_copy(&cbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::GetForgottenKnowledgeRecoveryRangeStart(this, core::mem::transmute_copy(&pbrangestart), core::mem::transmute_copy(&pcbrangestart)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::GetForgottenKnowledgeRecoveryRangeEnd(this, core::mem::transmute_copy(&pbrangeend), core::mem::transmute_copy(&pcbrangeend)).into()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *const SYNC_RANGE) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::SetForgottenKnowledgeRecoveryRange(this, core::mem::transmute_copy(&prange)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState_Impl::OnProgress(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&syncstage), core::mem::transmute_copy(&dwcompletedwork), core::mem::transmute_copy(&dwtotalwork)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsCanceled: IsCanceled::<Identity, OFFSET>,
            GetInfoForChangeApplication: GetInfoForChangeApplication::<Identity, OFFSET>,
            LoadInfoFromChangeApplication: LoadInfoFromChangeApplication::<Identity, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart: GetForgottenKnowledgeRecoveryRangeStart::<Identity, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd: GetForgottenKnowledgeRecoveryRangeEnd::<Identity, OFFSET>,
            SetForgottenKnowledgeRecoveryRange: SetForgottenKnowledgeRecoveryRange::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncSessionState as windows_core::Interface>::IID
    }
}
pub trait ISyncSessionState2_Impl: Sized + ISyncSessionState_Impl {
    fn SetProviderWithError(&self, fself: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSessionErrorStatus(&self, phrsessionerror: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISyncSessionState2 {}
impl ISyncSessionState2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISyncSessionState2_Vtbl
    where
        Identity: ISyncSessionState2_Impl,
    {
        unsafe extern "system" fn SetProviderWithError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fself: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState2_Impl::SetProviderWithError(this, core::mem::transmute_copy(&fself)).into()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrsessionerror: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ISyncSessionState2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISyncSessionState2_Impl::GetSessionErrorStatus(this, core::mem::transmute_copy(&phrsessionerror)).into()
        }
        Self {
            base__: ISyncSessionState_Vtbl::new::<Identity, OFFSET>(),
            SetProviderWithError: SetProviderWithError::<Identity, OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISyncSessionState2 as windows_core::Interface>::IID || iid == &<ISyncSessionState as windows_core::Interface>::IID
    }
}
pub trait ISynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: Option<&ILoadChangeContext>) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ISynchronousDataRetriever {}
impl ISynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISynchronousDataRetriever_Vtbl
    where
        Identity: ISynchronousDataRetriever_Impl,
    {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT
        where
            Identity: ISynchronousDataRetriever_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISynchronousDataRetriever_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploadchangecontext: *mut core::ffi::c_void, ppunkdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISynchronousDataRetriever_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISynchronousDataRetriever_Impl::LoadChangeData(this, windows_core::from_raw_borrowed(&ploadchangecontext)) {
                Ok(ok__) => {
                    ppunkdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronousDataRetriever as windows_core::Interface>::IID
    }
}
