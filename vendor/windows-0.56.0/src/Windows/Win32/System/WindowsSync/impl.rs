pub trait IAsynchronousDataRetriever_Impl: Sized {
    fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> windows_core::Result<()>;
    fn RegisterCallback(&self, pdataretrievercallback: Option<&IDataRetrieverCallback>) -> windows_core::Result<()>;
    fn RevokeCallback(&self, pdataretrievercallback: Option<&IDataRetrieverCallback>) -> windows_core::Result<()>;
    fn LoadChangeData(&self, ploadchangecontext: Option<&ILoadChangeContext>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsynchronousDataRetriever {}
impl IAsynchronousDataRetriever_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>() -> IAsynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAsynchronousDataRetriever_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn RegisterCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataretrievercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAsynchronousDataRetriever_Impl::RegisterCallback(this, windows_core::from_raw_borrowed(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn RevokeCallback<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataretrievercallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAsynchronousDataRetriever_Impl::RevokeCallback(this, windows_core::from_raw_borrowed(&pdataretrievercallback)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploadchangecontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IAsynchronousDataRetriever_Impl::LoadChangeData(this, windows_core::from_raw_borrowed(&ploadchangecontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            RevokeCallback: RevokeCallback::<Identity, Impl, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>() -> IChangeConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IChangeConflict_Impl::GetDestinationProviderConflictingChange(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IChangeConflict_Impl::GetSourceProviderConflictingChange(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IChangeConflict_Impl::GetDestinationProviderConflictingData(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IChangeConflict_Impl::GetSourceProviderConflictingData(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResolveActionForChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeConflict_Impl::GetResolveActionForChange(this, core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeConflict_Impl::SetResolveActionForChange(this, core::mem::transmute_copy(&resolveaction)).into()
        }
        unsafe extern "system" fn GetResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeConflict_Impl::GetResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&presolveaction)).into()
        }
        unsafe extern "system" fn SetResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeConflict_Impl::SetResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&resolveaction)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, Impl, OFFSET>,
            GetResolveActionForChange: GetResolveActionForChange::<Identity, Impl, OFFSET>,
            SetResolveActionForChange: SetResolveActionForChange::<Identity, Impl, OFFSET>,
            GetResolveActionForChangeUnit: GetResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            SetResolveActionForChangeUnit: SetResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: isize>() -> IChangeUnitException_Vtbl {
        unsafe extern "system" fn GetItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeUnitException_Impl::GetItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeUnitException_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeUnitException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>() -> IChangeUnitListFilterInfo_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeUnitListFilterInfo_Impl::Initialize(this, core::mem::transmute_copy(&ppbchangeunitids), core::mem::transmute_copy(&dwchangeunitcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitIdCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeUnitListFilterInfo_Impl::GetChangeUnitIdCount(this, core::mem::transmute_copy(&pdwchangeunitidcount)).into()
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IChangeUnitListFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IChangeUnitListFilterInfo_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&dwchangeunitidindex), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base__: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetChangeUnitIdCount: GetChangeUnitIdCount::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IClockVector_Impl, const OFFSET: isize>() -> IClockVector_Vtbl {
        unsafe extern "system" fn GetClockVectorElements<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppienumclockvector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IClockVector_Impl::GetClockVectorElements(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppienumclockvector)).into()
        }
        unsafe extern "system" fn GetClockVectorElementCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IClockVector_Impl::GetClockVectorElementCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClockVectorElements: GetClockVectorElements::<Identity, Impl, OFFSET>,
            GetClockVectorElementCount: GetClockVectorElementCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IClockVectorElement_Impl, const OFFSET: isize>() -> IClockVectorElement_Vtbl {
        unsafe extern "system" fn GetReplicaKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwreplicakey: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IClockVectorElement_Impl::GetReplicaKey(this, core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn GetTickCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulltickcount: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IClockVectorElement_Impl::GetTickCount(this, core::mem::transmute_copy(&pulltickcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetReplicaKey: GetReplicaKey::<Identity, Impl, OFFSET>,
            GetTickCount: GetTickCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>() -> ICombinedFilterInfo_Vtbl {
        unsafe extern "system" fn GetFilterCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfiltercount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICombinedFilterInfo_Impl::GetFilterCount(this, core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICombinedFilterInfo_Impl::GetFilterInfo(this, core::mem::transmute_copy(&dwfilterindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppifilterinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilterCombinationType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICombinedFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICombinedFilterInfo_Impl::GetFilterCombinationType(this, core::mem::transmute_copy(&pfiltercombinationtype)).into()
        }
        Self {
            base__: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, Impl, OFFSET>,
            GetFilterInfo: GetFilterInfo::<Identity, Impl, OFFSET>,
            GetFilterCombinationType: GetFilterCombinationType::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>() -> IConstraintConflict_Vtbl {
        unsafe extern "system" fn GetDestinationProviderConflictingChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IConstraintConflict_Impl::GetDestinationProviderConflictingChange(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IConstraintConflict_Impl::GetSourceProviderConflictingChange(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginalchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IConstraintConflict_Impl::GetDestinationProviderOriginalChange(this) {
                Ok(ok__) => {
                    core::ptr::write(pporiginalchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderConflictingData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IConstraintConflict_Impl::GetDestinationProviderConflictingData(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceProviderConflictingData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconflictingdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IConstraintConflict_Impl::GetSourceProviderConflictingData(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconflictingdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestinationProviderOriginalData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporiginaldata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IConstraintConflict_Impl::GetDestinationProviderOriginalData(this) {
                Ok(ok__) => {
                    core::ptr::write(pporiginaldata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConstraintResolveActionForChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstraintConflict_Impl::GetConstraintResolveActionForChange(this, core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstraintConflict_Impl::SetConstraintResolveActionForChange(this, core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstraintConflict_Impl::GetConstraintResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&pconstraintresolveaction)).into()
        }
        unsafe extern "system" fn SetConstraintResolveActionForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstraintConflict_Impl::SetConstraintResolveActionForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&constraintresolveaction)).into()
        }
        unsafe extern "system" fn GetConstraintConflictReason<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstraintConflict_Impl::GetConstraintConflictReason(this, core::mem::transmute_copy(&pconstraintconflictreason)).into()
        }
        unsafe extern "system" fn IsTemporary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstraintConflict_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstraintConflict_Impl::IsTemporary(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDestinationProviderConflictingChange: GetDestinationProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingChange: GetSourceProviderConflictingChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderOriginalChange: GetDestinationProviderOriginalChange::<Identity, Impl, OFFSET>,
            GetDestinationProviderConflictingData: GetDestinationProviderConflictingData::<Identity, Impl, OFFSET>,
            GetSourceProviderConflictingData: GetSourceProviderConflictingData::<Identity, Impl, OFFSET>,
            GetDestinationProviderOriginalData: GetDestinationProviderOriginalData::<Identity, Impl, OFFSET>,
            GetConstraintResolveActionForChange: GetConstraintResolveActionForChange::<Identity, Impl, OFFSET>,
            SetConstraintResolveActionForChange: SetConstraintResolveActionForChange::<Identity, Impl, OFFSET>,
            GetConstraintResolveActionForChangeUnit: GetConstraintResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            SetConstraintResolveActionForChangeUnit: SetConstraintResolveActionForChangeUnit::<Identity, Impl, OFFSET>,
            GetConstraintConflictReason: GetConstraintConflictReason::<Identity, Impl, OFFSET>,
            IsTemporary: IsTemporary::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstructReplicaKeyMap_Impl, const OFFSET: isize>() -> IConstructReplicaKeyMap_Vtbl {
        unsafe extern "system" fn FindOrAddReplica<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IConstructReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IConstructReplicaKeyMap_Impl::FindOrAddReplica(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindOrAddReplica: FindOrAddReplica::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: isize>() -> ICoreFragment_Vtbl {
        unsafe extern "system" fn NextColumn<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragment_Impl::NextColumn(this, core::mem::transmute_copy(&pchangeunitid), core::mem::transmute_copy(&pchangeunitidsize)).into()
        }
        unsafe extern "system" fn NextRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragment_Impl::NextRange(this, core::mem::transmute_copy(&pitemid), core::mem::transmute_copy(&pitemidsize), core::mem::transmute_copy(&piclockvector)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragment_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetColumnCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolumncount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragment_Impl::GetColumnCount(this, core::mem::transmute_copy(&pcolumncount)).into()
        }
        unsafe extern "system" fn GetRangeCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prangecount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragment_Impl::GetRangeCount(this, core::mem::transmute_copy(&prangecount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NextColumn: NextColumn::<Identity, Impl, OFFSET>,
            NextRange: NextRange::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetRangeCount: GetRangeCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>() -> ICoreFragmentInspector_Vtbl {
        unsafe extern "system" fn NextCoreFragments<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut *mut core::ffi::c_void, pfetchedcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragmentInspector_Impl::NextCoreFragments(this, core::mem::transmute_copy(&requestedcount), core::mem::transmute_copy(&ppicorefragments), core::mem::transmute_copy(&pfetchedcount)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFragmentInspector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ICoreFragmentInspector_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NextCoreFragments: NextCoreFragments::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomFilterInfo_Impl, const OFFSET: isize>() -> ICustomFilterInfo_Vtbl {
        unsafe extern "system" fn GetSyncFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ICustomFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ICustomFilterInfo_Impl::GetSyncFilter(this) {
                Ok(ok__) => {
                    core::ptr::write(pisyncfilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetSyncFilter: GetSyncFilter::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>() -> IDataRetrieverCallback_Vtbl {
        unsafe extern "system" fn LoadChangeDataComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataRetrieverCallback_Impl::LoadChangeDataComplete(this, windows_core::from_raw_borrowed(&punkdata)).into()
        }
        unsafe extern "system" fn LoadChangeDataError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataRetrieverCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataRetrieverCallback_Impl::LoadChangeDataError(this, core::mem::transmute_copy(&hrerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadChangeDataComplete: LoadChangeDataComplete::<Identity, Impl, OFFSET>,
            LoadChangeDataError: LoadChangeDataError::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>() -> IEnumChangeUnitExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumChangeUnitExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&ppchangeunitexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumChangeUnitExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumChangeUnitExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumChangeUnitExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumChangeUnitExceptions_Impl::Clone(this) {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: isize>() -> IEnumClockVector_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumClockVector_Impl::Next(this, core::mem::transmute_copy(&cclockvectorelements), core::mem::transmute_copy(&ppiclockvectorelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csyncversions: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumClockVector_Impl::Skip(this, core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumClockVector_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumClockVector_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppienum, core::mem::transmute(ok__));
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>() -> IEnumFeedClockVector_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumFeedClockVector_Impl::Next(this, core::mem::transmute_copy(&cclockvectorelements), core::mem::transmute_copy(&ppiclockvectorelements), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csyncversions: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumFeedClockVector_Impl::Skip(this, core::mem::transmute_copy(&csyncversions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumFeedClockVector_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumFeedClockVector_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppienum, core::mem::transmute(ok__));
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
        iid == &<IEnumFeedClockVector as windows_core::Interface>::IID
    }
}
pub trait IEnumItemIds_Impl: Sized {
    fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumItemIds {}
impl IEnumItemIds_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemIds_Impl, const OFFSET: isize>() -> IEnumItemIds_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumItemIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumItemIds_Impl::Next(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>() -> IEnumRangeExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, pprangeexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumRangeExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&pprangeexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumRangeExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumRangeExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumRangeExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumRangeExceptions_Impl::Clone(this) {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>() -> IEnumSingleItemExceptions_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSingleItemExceptions_Impl::Next(this, core::mem::transmute_copy(&cexceptions), core::mem::transmute_copy(&ppsingleitemexception), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexceptions: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSingleItemExceptions_Impl::Skip(this, core::mem::transmute_copy(&cexceptions)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSingleItemExceptions_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSingleItemExceptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSingleItemExceptions_Impl::Clone(this) {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>() -> IEnumSyncChangeUnits_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, ppchangeunit: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncChangeUnits_Impl::Next(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&ppchangeunit), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncChangeUnits_Impl::Skip(this, core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncChangeUnits_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChangeUnits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSyncChangeUnits_Impl::Clone(this) {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>() -> IEnumSyncChanges_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32, ppchange: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncChanges_Impl::Next(this, core::mem::transmute_copy(&cchanges), core::mem::transmute_copy(&ppchange), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchanges: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncChanges_Impl::Skip(this, core::mem::transmute_copy(&cchanges)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncChanges_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncChanges_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSyncChanges_Impl::Clone(this) {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>() -> IEnumSyncProviderConfigUIInfos_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncProviderConfigUIInfos_Impl::Next(this, core::mem::transmute_copy(&cfactories), core::mem::transmute_copy(&ppsyncproviderconfiguiinfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cfactories: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncProviderConfigUIInfos_Impl::Skip(this, core::mem::transmute_copy(&cfactories)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncProviderConfigUIInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderConfigUIInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSyncProviderConfigUIInfos_Impl::Clone(this) {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>() -> IEnumSyncProviderInfos_Vtbl {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncProviderInfos_Impl::Next(this, core::mem::transmute_copy(&cinstances), core::mem::transmute_copy(&ppsyncproviderinfo), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinstances: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncProviderInfos_Impl::Skip(this, core::mem::transmute_copy(&cinstances)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumSyncProviderInfos_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSyncProviderInfos_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumSyncProviderInfos_Impl::Clone(this) {
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
        iid == &<IEnumSyncProviderInfos as windows_core::Interface>::IID
    }
}
pub trait IFeedClockVector_Impl: Sized + IClockVector_Impl {
    fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> windows_core::Result<()>;
    fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFeedClockVector {}
impl IFeedClockVector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFeedClockVector_Impl, const OFFSET: isize>() -> IFeedClockVector_Vtbl {
        unsafe extern "system" fn GetUpdateCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwupdatecount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFeedClockVector_Impl::GetUpdateCount(this, core::mem::transmute_copy(&pdwupdatecount)).into()
        }
        unsafe extern "system" fn IsNoConflictsSpecified<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFeedClockVector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFeedClockVector_Impl::IsNoConflictsSpecified(this, core::mem::transmute_copy(&pfisnoconflictsspecified)).into()
        }
        Self {
            base__: IClockVector_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetUpdateCount: GetUpdateCount::<Identity, Impl, OFFSET>,
            IsNoConflictsSpecified: IsNoConflictsSpecified::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>() -> IFeedClockVectorElement_Vtbl {
        unsafe extern "system" fn GetSyncTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psynctime: *mut SYNC_TIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFeedClockVectorElement_Impl::GetSyncTime(this, core::mem::transmute_copy(&psynctime)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFeedClockVectorElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbflags: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFeedClockVectorElement_Impl::GetFlags(this, core::mem::transmute_copy(&pbflags)).into()
        }
        Self {
            base__: IClockVectorElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncTime: GetSyncTime::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: isize>() -> IFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterKeyMap_Impl::GetCount(this, core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn AddFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncfilter: *mut core::ffi::c_void, pdwfilterkey: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterKeyMap_Impl::AddFilter(this, windows_core::from_raw_borrowed(&pisyncfilter), core::mem::transmute_copy(&pdwfilterkey)).into()
        }
        unsafe extern "system" fn GetFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IFilterKeyMap_Impl::GetFilter(this, core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    core::ptr::write(ppisyncfilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterKeyMap_Impl::Serialize(this, core::mem::transmute_copy(&pbfilterkeymap), core::mem::transmute_copy(&pcbfilterkeymap)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddFilter: AddFilter::<Identity, Impl, OFFSET>,
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterRequestCallback_Impl, const OFFSET: isize>() -> IFilterRequestCallback_Vtbl {
        unsafe extern "system" fn RequestFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterRequestCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void, filteringtype: FILTERING_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterRequestCallback_Impl::RequestFilter(this, windows_core::from_raw_borrowed(&pfilter), core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestFilter: RequestFilter::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>() -> IFilterTrackingProvider_Vtbl {
        unsafe extern "system" fn SpecifyTrackedFilters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterTrackingProvider_Impl::SpecifyTrackedFilters(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn AddTrackedFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterTrackingProvider_Impl::AddTrackedFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SpecifyTrackedFilters: SpecifyTrackedFilters::<Identity, Impl, OFFSET>,
            AddTrackedFilter: AddTrackedFilter::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>() -> IFilterTrackingRequestCallback_Vtbl {
        unsafe extern "system" fn RequestTrackedFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingRequestCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterTrackingRequestCallback_Impl::RequestTrackedFilter(this, windows_core::from_raw_borrowed(&pfilter)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestTrackedFilter: RequestTrackedFilter::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>() -> IFilterTrackingSyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddFilterChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterTrackingSyncChangeBuilder_Impl::AddFilterChange(this, core::mem::transmute_copy(&dwfilterkey), core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn SetAllChangeUnitsPresentFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFilterTrackingSyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFilterTrackingSyncChangeBuilder_Impl::SetAllChangeUnitsPresentFlag(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddFilterChange: AddFilterChange::<Identity, Impl, OFFSET>,
            SetAllChangeUnitsPresentFlag: SetAllChangeUnitsPresentFlag::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IForgottenKnowledge_Impl, const OFFSET: isize>() -> IForgottenKnowledge_Vtbl {
        unsafe extern "system" fn ForgetToVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IForgottenKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IForgottenKnowledge_Impl::ForgetToVersion(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pversion)).into()
        }
        Self { base__: ISyncKnowledge_Vtbl::new::<Identity, Impl, OFFSET>(), ForgetToVersion: ForgetToVersion::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>() -> IKnowledgeSyncProvider_Vtbl {
        unsafe extern "system" fn BeginSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::BeginSession(this, core::mem::transmute_copy(&role), windows_core::from_raw_borrowed(&psessionstate)).into()
        }
        unsafe extern "system" fn GetSyncBatchParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncknowledge: *mut *mut core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::GetSyncBatchParameters(this, core::mem::transmute_copy(&ppsyncknowledge), core::mem::transmute_copy(&pdwrequestedbatchsize)).into()
        }
        unsafe extern "system" fn GetChangeBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatchsize: u32, psyncknowledge: *mut core::ffi::c_void, ppsyncchangebatch: *mut *mut core::ffi::c_void, ppunkdataretriever: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::GetChangeBatch(this, core::mem::transmute_copy(&dwbatchsize), windows_core::from_raw_borrowed(&psyncknowledge), core::mem::transmute_copy(&ppsyncchangebatch), core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn GetFullEnumerationChangeBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut core::ffi::c_void, ppsyncchangebatch: *mut *mut core::ffi::c_void, ppunkdataretriever: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::GetFullEnumerationChangeBatch(this, core::mem::transmute_copy(&dwbatchsize), core::mem::transmute_copy(&pblowerenumerationbound), windows_core::from_raw_borrowed(&psyncknowledge), core::mem::transmute_copy(&ppsyncchangebatch), core::mem::transmute_copy(&ppunkdataretriever)).into()
        }
        unsafe extern "system" fn ProcessChangeBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut core::ffi::c_void, punkdataretriever: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::ProcessChangeBatch(this, core::mem::transmute_copy(&resolutionpolicy), windows_core::from_raw_borrowed(&psourcechangebatch), windows_core::from_raw_borrowed(&punkdataretriever), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn ProcessFullEnumerationChangeBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut core::ffi::c_void, punkdataretriever: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::ProcessFullEnumerationChangeBatch(this, core::mem::transmute_copy(&resolutionpolicy), windows_core::from_raw_borrowed(&psourcechangebatch), windows_core::from_raw_borrowed(&punkdataretriever), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psyncsessionstatistics)).into()
        }
        unsafe extern "system" fn EndSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKnowledgeSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKnowledgeSyncProvider_Impl::EndSession(this, windows_core::from_raw_borrowed(&psessionstate)).into()
        }
        Self {
            base__: ISyncProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginSession: BeginSession::<Identity, Impl, OFFSET>,
            GetSyncBatchParameters: GetSyncBatchParameters::<Identity, Impl, OFFSET>,
            GetChangeBatch: GetChangeBatch::<Identity, Impl, OFFSET>,
            GetFullEnumerationChangeBatch: GetFullEnumerationChangeBatch::<Identity, Impl, OFFSET>,
            ProcessChangeBatch: ProcessChangeBatch::<Identity, Impl, OFFSET>,
            ProcessFullEnumerationChangeBatch: ProcessFullEnumerationChangeBatch::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: isize>() -> ILoadChangeContext_Vtbl {
        unsafe extern "system" fn GetSyncChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ILoadChangeContext_Impl::GetSyncChange(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsyncchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoverableErrorOnChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, perrordata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoadChangeContext_Impl::SetRecoverableErrorOnChange(this, core::mem::transmute_copy(&hrerror), windows_core::from_raw_borrowed(&perrordata)).into()
        }
        unsafe extern "system" fn SetRecoverableErrorOnChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoadChangeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, pchangeunit: *mut core::ffi::c_void, perrordata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ILoadChangeContext_Impl::SetRecoverableErrorOnChangeUnit(this, core::mem::transmute_copy(&hrerror), windows_core::from_raw_borrowed(&pchangeunit), windows_core::from_raw_borrowed(&perrordata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSyncChange: GetSyncChange::<Identity, Impl, OFFSET>,
            SetRecoverableErrorOnChange: SetRecoverableErrorOnChange::<Identity, Impl, OFFSET>,
            SetRecoverableErrorOnChangeUnit: SetRecoverableErrorOnChangeUnit::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderConverter_Impl, const OFFSET: isize>() -> IProviderConverter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IProviderConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IProviderConverter_Impl::Initialize(this, windows_core::from_raw_borrowed(&pisyncprovider)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: isize>() -> IRangeException_Vtbl {
        unsafe extern "system" fn GetClosedRangeStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRangeException_Impl::GetClosedRangeStart(this, core::mem::transmute_copy(&pbclosedrangestart), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedRangeEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRangeException_Impl::GetClosedRangeEnd(this, core::mem::transmute_copy(&pbclosedrangeend), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRangeException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRangeException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClosedRangeStart: GetClosedRangeStart::<Identity, Impl, OFFSET>,
            GetClosedRangeEnd: GetClosedRangeEnd::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: isize>() -> IRecoverableError_Vtbl {
        unsafe extern "system" fn GetStage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableError_Impl::GetStage(this, core::mem::transmute_copy(&pstage)).into()
        }
        unsafe extern "system" fn GetProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableError_Impl::GetProvider(this, core::mem::transmute_copy(&pproviderrole)).into()
        }
        unsafe extern "system" fn GetChangeWithRecoverableError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppchangewithrecoverableerror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRecoverableError_Impl::GetChangeWithRecoverableError(this) {
                Ok(ok__) => {
                    core::ptr::write(ppchangewithrecoverableerror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT, pperrordata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableError_Impl::GetRecoverableErrorDataForChange(this, core::mem::transmute_copy(&phrerror), core::mem::transmute_copy(&pperrordata)).into()
        }
        unsafe extern "system" fn GetRecoverableErrorDataForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchangeunit: *mut core::ffi::c_void, phrerror: *mut windows_core::HRESULT, pperrordata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableError_Impl::GetRecoverableErrorDataForChangeUnit(this, windows_core::from_raw_borrowed(&pchangeunit), core::mem::transmute_copy(&phrerror), core::mem::transmute_copy(&pperrordata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStage: GetStage::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetChangeWithRecoverableError: GetChangeWithRecoverableError::<Identity, Impl, OFFSET>,
            GetRecoverableErrorDataForChange: GetRecoverableErrorDataForChange::<Identity, Impl, OFFSET>,
            GetRecoverableErrorDataForChangeUnit: GetRecoverableErrorDataForChangeUnit::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>() -> IRecoverableErrorData_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcszitemdisplayname: windows_core::PCWSTR, pcszerrordescription: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableErrorData_Impl::Initialize(this, core::mem::transmute(&pcszitemdisplayname), core::mem::transmute(&pcszerrordescription)).into()
        }
        unsafe extern "system" fn GetItemDisplayName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszitemdisplayname: windows_core::PCWSTR, pcchitemdisplayname: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableErrorData_Impl::GetItemDisplayName(this, core::mem::transmute(&pszitemdisplayname), core::mem::transmute_copy(&pcchitemdisplayname)).into()
        }
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRecoverableErrorData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszerrordescription: windows_core::PCWSTR, pccherrordescription: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRecoverableErrorData_Impl::GetErrorDescription(this, core::mem::transmute(&pszerrordescription), core::mem::transmute_copy(&pccherrordescription)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetItemDisplayName: GetItemDisplayName::<Identity, Impl, OFFSET>,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>() -> IRegisteredSyncProvider_Vtbl {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pcontextpropertystore: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRegisteredSyncProvider_Impl::Init(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&pguidcontenttype), windows_core::from_raw_borrowed(&pcontextpropertystore)).into()
        }
        unsafe extern "system" fn GetInstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRegisteredSyncProvider_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pguidinstanceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRegisteredSyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRegisteredSyncProvider_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>() -> IReplicaKeyMap_Vtbl {
        unsafe extern "system" fn LookupReplicaKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IReplicaKeyMap_Impl::LookupReplicaKey(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pdwreplicakey)).into()
        }
        unsafe extern "system" fn LookupReplicaId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IReplicaKeyMap_Impl::LookupReplicaId(this, core::mem::transmute_copy(&dwreplicakey), core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IReplicaKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IReplicaKeyMap_Impl::Serialize(this, core::mem::transmute_copy(&pbreplicakeymap), core::mem::transmute_copy(&pcbreplicakeymap)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LookupReplicaKey: LookupReplicaKey::<Identity, Impl, OFFSET>,
            LookupReplicaId: LookupReplicaId::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRequestFilteredSync_Impl, const OFFSET: isize>() -> IRequestFilteredSync_Vtbl {
        unsafe extern "system" fn SpecifyFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRequestFilteredSync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRequestFilteredSync_Impl::SpecifyFilter(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SpecifyFilter: SpecifyFilter::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISingleItemException_Impl, const OFFSET: isize>() -> ISingleItemException_Vtbl {
        unsafe extern "system" fn GetItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISingleItemException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISingleItemException_Impl::GetItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClockVector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISingleItemException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISingleItemException_Impl::GetClockVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemId: GetItemId::<Identity, Impl, OFFSET>,
            GetClockVector: GetClockVector::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportFilteredSync_Impl, const OFFSET: isize>() -> ISupportFilteredSync_Vtbl {
        unsafe extern "system" fn AddFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportFilteredSync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut core::ffi::c_void, filteringtype: FILTERING_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISupportFilteredSync_Impl::AddFilter(this, windows_core::from_raw_borrowed(&pfilter), core::mem::transmute_copy(&filteringtype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddFilter: AddFilter::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>() -> ISupportLastWriteTime_Vtbl {
        unsafe extern "system" fn GetItemChangeTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISupportLastWriteTime_Impl::GetItemChangeTime(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pulltimestamp)).into()
        }
        unsafe extern "system" fn GetChangeUnitChangeTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISupportLastWriteTime_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISupportLastWriteTime_Impl::GetChangeUnitChangeTime(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pulltimestamp)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemChangeTime: GetItemChangeTime::<Identity, Impl, OFFSET>,
            GetChangeUnitChangeTime: GetChangeUnitChangeTime::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: isize>() -> ISyncCallback_Vtbl {
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback_Impl::OnProgress(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&syncstage), core::mem::transmute_copy(&dwcompletedwork), core::mem::transmute_copy(&dwtotalwork)).into()
        }
        unsafe extern "system" fn OnChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncchange: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback_Impl::OnChange(this, windows_core::from_raw_borrowed(&psyncchange)).into()
        }
        unsafe extern "system" fn OnConflict<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconflict: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback_Impl::OnConflict(this, windows_core::from_raw_borrowed(&pconflict)).into()
        }
        unsafe extern "system" fn OnFullEnumerationNeeded<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback_Impl::OnFullEnumerationNeeded(this, core::mem::transmute_copy(&pfullenumerationaction)).into()
        }
        unsafe extern "system" fn OnRecoverableError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precoverableerror: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback_Impl::OnRecoverableError(this, windows_core::from_raw_borrowed(&precoverableerror)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnChange: OnChange::<Identity, Impl, OFFSET>,
            OnConflict: OnConflict::<Identity, Impl, OFFSET>,
            OnFullEnumerationNeeded: OnFullEnumerationNeeded::<Identity, Impl, OFFSET>,
            OnRecoverableError: OnRecoverableError::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback2_Impl, const OFFSET: isize>() -> ISyncCallback2_Vtbl {
        unsafe extern "system" fn OnChangeApplied<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback2_Impl::OnChangeApplied(this, core::mem::transmute_copy(&dwchangesapplied), core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        unsafe extern "system" fn OnChangeFailed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncCallback2_Impl::OnChangeFailed(this, core::mem::transmute_copy(&dwchangesapplied), core::mem::transmute_copy(&dwchangesfailed)).into()
        }
        Self {
            base__: ISyncCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnChangeApplied: OnChangeApplied::<Identity, Impl, OFFSET>,
            OnChangeFailed: OnChangeFailed::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>() -> ISyncChange_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::GetOwnerReplicaId(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetRootItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::GetRootItemId(this, core::mem::transmute_copy(&pbrootitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::GetChangeVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetCreationVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::GetCreationVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetWorkEstimate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwork: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::GetWorkEstimate(this, core::mem::transmute_copy(&pdwwork)).into()
        }
        unsafe extern "system" fn GetChangeUnits<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChange_Impl::GetChangeUnits(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMadeWithKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmadewithknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChange_Impl::GetMadeWithKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(ppmadewithknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChange_Impl::GetLearnedKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkEstimate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwork: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChange_Impl::SetWorkEstimate(this, core::mem::transmute_copy(&dwwork)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, Impl, OFFSET>,
            GetRootItemId: GetRootItemId::<Identity, Impl, OFFSET>,
            GetChangeVersion: GetChangeVersion::<Identity, Impl, OFFSET>,
            GetCreationVersion: GetCreationVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetWorkEstimate: GetWorkEstimate::<Identity, Impl, OFFSET>,
            GetChangeUnits: GetChangeUnits::<Identity, Impl, OFFSET>,
            GetMadeWithKnowledge: GetMadeWithKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, Impl, OFFSET>,
            SetWorkEstimate: SetWorkEstimate::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>() -> ISyncChangeBatch_Vtbl {
        unsafe extern "system" fn BeginUnorderedGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatch_Impl::BeginUnorderedGroup(this).into()
        }
        unsafe extern "system" fn EndUnorderedGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmadewithknowledge: *mut core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatch_Impl::EndUnorderedGroup(this, windows_core::from_raw_borrowed(&pmadewithknowledge), core::mem::transmute_copy(&fallchangesforknowledge)).into()
        }
        unsafe extern "system" fn AddLoggedConflict<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut core::ffi::c_void, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatch_Impl::AddLoggedConflict(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwworkforchange), windows_core::from_raw_borrowed(&pconflictknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(ppchangebuilder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginUnorderedGroup: BeginUnorderedGroup::<Identity, Impl, OFFSET>,
            EndUnorderedGroup: EndUnorderedGroup::<Identity, Impl, OFFSET>,
            AddLoggedConflict: AddLoggedConflict::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>() -> ISyncChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatch2_Impl::AddMergeTombstoneMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    core::ptr::write(ppchangebuilder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMergeTombstoneLoggedConflict<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut core::ffi::c_void, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatch2_Impl::AddMergeTombstoneLoggedConflict(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange), windows_core::from_raw_borrowed(&pconflictknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(ppchangebuilder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, Impl, OFFSET>,
            AddMergeTombstoneLoggedConflict: AddMergeTombstoneLoggedConflict::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>() -> ISyncChangeBatchAdvanced_Vtbl {
        unsafe extern "system" fn GetFilterInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilterinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchAdvanced_Impl::GetFilterInfo(this) {
                Ok(ok__) => {
                    core::ptr::write(ppfilterinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertFullEnumerationChangeBatchToRegularChangeBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppchangebatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchAdvanced_Impl::ConvertFullEnumerationChangeBatchToRegularChangeBatch(this) {
                Ok(ok__) => {
                    core::ptr::write(ppchangebatch, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperBoundItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchAdvanced_Impl::GetUpperBoundItemId(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetBatchLevelKnowledgeShouldBeApplied<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchAdvanced_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchAdvanced_Impl::GetBatchLevelKnowledgeShouldBeApplied(this, core::mem::transmute_copy(&pfbatchknowledgeshouldbeapplied)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterInfo: GetFilterInfo::<Identity, Impl, OFFSET>,
            ConvertFullEnumerationChangeBatchToRegularChangeBatch: ConvertFullEnumerationChangeBatchToRegularChangeBatch::<Identity, Impl, OFFSET>,
            GetUpperBoundItemId: GetUpperBoundItemId::<Identity, Impl, OFFSET>,
            GetBatchLevelKnowledgeShouldBeApplied: GetBatchLevelKnowledgeShouldBeApplied::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>() -> ISyncChangeBatchBase_Vtbl {
        unsafe extern "system" fn GetChangeEnumerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchBase_Impl::GetChangeEnumerator(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsLastBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::GetIsLastBatch(this, core::mem::transmute_copy(&pflastbatch)).into()
        }
        unsafe extern "system" fn GetWorkEstimateForBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwworkforbatch: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::GetWorkEstimateForBatch(this, core::mem::transmute_copy(&pdwworkforbatch)).into()
        }
        unsafe extern "system" fn GetRemainingWorkEstimateForSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::GetRemainingWorkEstimateForSession(this, core::mem::transmute_copy(&pdwremainingworkforsession)).into()
        }
        unsafe extern "system" fn BeginOrderedGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblowerbound: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::BeginOrderedGroup(this, core::mem::transmute_copy(&pblowerbound)).into()
        }
        unsafe extern "system" fn EndOrderedGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::EndOrderedGroup(this, core::mem::transmute_copy(&pbupperbound), windows_core::from_raw_borrowed(&pmadewithknowledge)).into()
        }
        unsafe extern "system" fn AddItemMetadataToGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchBase_Impl::AddItemMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    core::ptr::write(ppchangebuilder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchBase_Impl::GetLearnedKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprerequisteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchBase_Impl::GetPrerequisiteKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(ppprerequisteknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsourceforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchBase_Impl::GetSourceForgottenKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsourceforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::SetLastBatch(this).into()
        }
        unsafe extern "system" fn SetWorkEstimateForBatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwworkforbatch: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::SetWorkEstimateForBatch(this, core::mem::transmute_copy(&dwworkforbatch)).into()
        }
        unsafe extern "system" fn SetRemainingWorkEstimateForSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwremainingworkforsession: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::SetRemainingWorkEstimateForSession(this, core::mem::transmute_copy(&dwremainingworkforsession)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase_Impl::Serialize(this, core::mem::transmute_copy(&pbchangebatch), core::mem::transmute_copy(&pcbchangebatch)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChangeEnumerator: GetChangeEnumerator::<Identity, Impl, OFFSET>,
            GetIsLastBatch: GetIsLastBatch::<Identity, Impl, OFFSET>,
            GetWorkEstimateForBatch: GetWorkEstimateForBatch::<Identity, Impl, OFFSET>,
            GetRemainingWorkEstimateForSession: GetRemainingWorkEstimateForSession::<Identity, Impl, OFFSET>,
            BeginOrderedGroup: BeginOrderedGroup::<Identity, Impl, OFFSET>,
            EndOrderedGroup: EndOrderedGroup::<Identity, Impl, OFFSET>,
            AddItemMetadataToGroup: AddItemMetadataToGroup::<Identity, Impl, OFFSET>,
            GetLearnedKnowledge: GetLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetSourceForgottenKnowledge: GetSourceForgottenKnowledge::<Identity, Impl, OFFSET>,
            SetLastBatch: SetLastBatch::<Identity, Impl, OFFSET>,
            SetWorkEstimateForBatch: SetWorkEstimateForBatch::<Identity, Impl, OFFSET>,
            SetRemainingWorkEstimateForSession: SetRemainingWorkEstimateForSession::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase2_Impl, const OFFSET: isize>() -> ISyncChangeBatchBase2_Vtbl {
        unsafe extern "system" fn SerializeWithOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchBase2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchBase2_Impl::SerializeWithOptions(this, core::mem::transmute_copy(&targetformatversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        Self { base__: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(), SerializeWithOptions: SerializeWithOptions::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>() -> ISyncChangeBatchWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterKeyMap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppifilterkeymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilterKeyMap(this) {
                Ok(ok__) => {
                    core::ptr::write(ppifilterkeymap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterKeyMap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pifilterkeymap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchWithFilterKeyMap_Impl::SetFilterKeyMap(this, windows_core::from_raw_borrowed(&pifilterkeymap)).into()
        }
        unsafe extern "system" fn SetFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchWithFilterKeyMap_Impl::SetFilterForgottenKnowledge(this, core::mem::transmute_copy(&dwfilterkey), windows_core::from_raw_borrowed(&pfilterforgottenknowledge)).into()
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedfilterforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedfilterforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterKeyMap: GetFilterKeyMap::<Identity, Impl, OFFSET>,
            SetFilterKeyMap: SetFilterKeyMap::<Identity, Impl, OFFSET>,
            SetFilterForgottenKnowledge: SetFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>() -> ISyncChangeBatchWithPrerequisite_Vtbl {
        unsafe extern "system" fn SetPrerequisiteKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprerequisiteknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBatchWithPrerequisite_Impl::SetPrerequisiteKnowledge(this, windows_core::from_raw_borrowed(&pprerequisiteknowledge)).into()
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithPrerequisite_Impl::GetLearnedKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedwithprerequisiteknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBatchWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeBatchWithPrerequisite_Impl::GetLearnedForgottenKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPrerequisiteKnowledge: SetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBuilder_Impl, const OFFSET: isize>() -> ISyncChangeBuilder_Vtbl {
        unsafe extern "system" fn AddChangeUnitMetadata<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeBuilder_Impl::AddChangeUnitMetadata(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pchangeunitversion)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddChangeUnitMetadata: AddChangeUnitMetadata::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>() -> ISyncChangeUnit_Vtbl {
        unsafe extern "system" fn GetItemChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeUnit_Impl::GetItemChange(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsyncchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChangeUnitId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeUnit_Impl::GetChangeUnitId(this, core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetChangeUnitVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeUnit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeUnit_Impl::GetChangeUnitVersion(this, core::mem::transmute_copy(&pbcurrentreplicaid), core::mem::transmute_copy(&pversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItemChange: GetItemChange::<Identity, Impl, OFFSET>,
            GetChangeUnitId: GetChangeUnitId::<Identity, Impl, OFFSET>,
            GetChangeUnitVersion: GetChangeUnitVersion::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>() -> ISyncChangeWithFilterKeyMap_Vtbl {
        unsafe extern "system" fn GetFilterCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfiltercount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeWithFilterKeyMap_Impl::GetFilterCount(this, core::mem::transmute_copy(&pdwfiltercount)).into()
        }
        unsafe extern "system" fn GetFilterChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeWithFilterKeyMap_Impl::GetFilterChange(this, core::mem::transmute_copy(&dwfilterkey), core::mem::transmute_copy(&pfilterchange)).into()
        }
        unsafe extern "system" fn GetAllChangeUnitsPresentFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncChangeWithFilterKeyMap_Impl::GetAllChangeUnitsPresentFlag(this, core::mem::transmute_copy(&pfallchangeunitspresent)).into()
        }
        unsafe extern "system" fn GetFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithFilterKeyMap_Impl::GetFilterForgottenKnowledge(this, core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    core::ptr::write(ppifilterforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedfilterforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledge(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithFilterKeyMap_Impl::GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithFilterKeyMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pnewmoveins: *mut core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithFilterKeyMap_Impl::GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete(this, windows_core::from_raw_borrowed(&pdestinationknowledge), windows_core::from_raw_borrowed(&pnewmoveins), core::mem::transmute_copy(&dwfilterkey)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedfilterforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilterCount: GetFilterCount::<Identity, Impl, OFFSET>,
            GetFilterChange: GetFilterChange::<Identity, Impl, OFFSET>,
            GetAllChangeUnitsPresentFlag: GetAllChangeUnitsPresentFlag::<Identity, Impl, OFFSET>,
            GetFilterForgottenKnowledge: GetFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedKnowledge: GetFilteredReplicaLearnedKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledge: GetLearnedFilterForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledge: GetFilteredReplicaLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
            GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>() -> ISyncChangeWithPrerequisite_Vtbl {
        unsafe extern "system" fn GetPrerequisiteKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprerequisiteknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithPrerequisite_Impl::GetPrerequisiteKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(ppprerequisiteknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedKnowledgeWithPrerequisite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncChangeWithPrerequisite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationknowledge: *mut core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncChangeWithPrerequisite_Impl::GetLearnedKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pdestinationknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedknowledgewithprerequisite, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPrerequisiteKnowledge: GetPrerequisiteKnowledge::<Identity, Impl, OFFSET>,
            GetLearnedKnowledgeWithPrerequisite: GetLearnedKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncConstraintCallback_Impl, const OFFSET: isize>() -> ISyncConstraintCallback_Vtbl {
        unsafe extern "system" fn OnConstraintConflict<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncConstraintCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconflict: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncConstraintCallback_Impl::OnConstraintConflict(this, windows_core::from_raw_borrowed(&pconflict)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConstraintConflict: OnConstraintConflict::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: isize>() -> ISyncDataConverter_Vtbl {
        unsafe extern "system" fn ConvertDataRetrieverFromProviderFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdataretrieverin: *mut core::ffi::c_void, penumsyncchanges: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncDataConverter_Impl::ConvertDataRetrieverFromProviderFormat(this, windows_core::from_raw_borrowed(&punkdataretrieverin), windows_core::from_raw_borrowed(&penumsyncchanges)) {
                Ok(ok__) => {
                    core::ptr::write(ppunkdataout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataRetrieverToProviderFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdataretrieverin: *mut core::ffi::c_void, penumsyncchanges: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncDataConverter_Impl::ConvertDataRetrieverToProviderFormat(this, windows_core::from_raw_borrowed(&punkdataretrieverin), windows_core::from_raw_borrowed(&penumsyncchanges)) {
                Ok(ok__) => {
                    core::ptr::write(ppunkdataout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataFromProviderFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatacontext: *mut core::ffi::c_void, punkdatain: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncDataConverter_Impl::ConvertDataFromProviderFormat(this, windows_core::from_raw_borrowed(&pdatacontext), windows_core::from_raw_borrowed(&punkdatain)) {
                Ok(ok__) => {
                    core::ptr::write(ppunkdataout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDataToProviderFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncDataConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatacontext: *mut core::ffi::c_void, punkdataout: *mut core::ffi::c_void, ppunkdataout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncDataConverter_Impl::ConvertDataToProviderFormat(this, windows_core::from_raw_borrowed(&pdatacontext), windows_core::from_raw_borrowed(&punkdataout)) {
                Ok(ok__) => {
                    core::ptr::write(ppunkdataout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConvertDataRetrieverFromProviderFormat: ConvertDataRetrieverFromProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataRetrieverToProviderFormat: ConvertDataRetrieverToProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataFromProviderFormat: ConvertDataFromProviderFormat::<Identity, Impl, OFFSET>,
            ConvertDataToProviderFormat: ConvertDataToProviderFormat::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilter_Impl, const OFFSET: isize>() -> ISyncFilter_Vtbl {
        unsafe extern "system" fn IsIdentical<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncFilter_Impl::IsIdentical(this, windows_core::from_raw_borrowed(&psyncfilter)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncFilter_Impl::Serialize(this, core::mem::transmute_copy(&pbsyncfilter), core::mem::transmute_copy(&pcbsyncfilter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsIdentical: IsIdentical::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilterDeserializer_Impl, const OFFSET: isize>() -> ISyncFilterDeserializer_Vtbl {
        unsafe extern "system" fn DeserializeSyncFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilterDeserializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncFilterDeserializer_Impl::DeserializeSyncFilter(this, core::mem::transmute_copy(&pbsyncfilter), core::mem::transmute_copy(&dwcbsyncfilter)) {
                Ok(ok__) => {
                    core::ptr::write(ppisyncfilter, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DeserializeSyncFilter: DeserializeSyncFilter::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilterInfo_Impl, const OFFSET: isize>() -> ISyncFilterInfo_Vtbl {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilterInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncFilterInfo_Impl::Serialize(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pcbbuffer)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Serialize: Serialize::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilterInfo2_Impl, const OFFSET: isize>() -> ISyncFilterInfo2_Vtbl {
        unsafe extern "system" fn GetFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFilterInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncFilterInfo2_Impl::GetFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: ISyncFilterInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetFlags: GetFlags::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChange_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncFullEnumerationChange_Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLearnedForgottenKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncFullEnumerationChange_Impl::GetLearnedForgottenKnowledge(this) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedforgottenknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetLearnedForgottenKnowledge: GetLearnedForgottenKnowledge::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch_Vtbl {
        unsafe extern "system" fn GetLearnedKnowledgeAfterRecoveryComplete<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncFullEnumerationChangeBatch_Impl::GetLearnedKnowledgeAfterRecoveryComplete(this) {
                Ok(ok__) => {
                    core::ptr::write(pplearnedknowledgeafterrecoverycomplete, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClosedLowerBoundItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncFullEnumerationChangeBatch_Impl::GetClosedLowerBoundItemId(this, core::mem::transmute_copy(&pbclosedlowerbounditemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn GetClosedUpperBoundItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncFullEnumerationChangeBatch_Impl::GetClosedUpperBoundItemId(this, core::mem::transmute_copy(&pbclosedupperbounditemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self {
            base__: ISyncChangeBatchBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLearnedKnowledgeAfterRecoveryComplete: GetLearnedKnowledgeAfterRecoveryComplete::<Identity, Impl, OFFSET>,
            GetClosedLowerBoundItemId: GetClosedLowerBoundItemId::<Identity, Impl, OFFSET>,
            GetClosedUpperBoundItemId: GetClosedUpperBoundItemId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>() -> ISyncFullEnumerationChangeBatch2_Vtbl {
        unsafe extern "system" fn AddMergeTombstoneMetadataToGroup<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncFullEnumerationChangeBatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncFullEnumerationChangeBatch2_Impl::AddMergeTombstoneMetadataToGroup(this, core::mem::transmute_copy(&pbownerreplicaid), core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pchangeversion), core::mem::transmute_copy(&pcreationversion), core::mem::transmute_copy(&dwworkforchange)) {
                Ok(ok__) => {
                    core::ptr::write(ppchangebuilder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISyncFullEnumerationChangeBatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddMergeTombstoneMetadataToGroup: AddMergeTombstoneMetadataToGroup::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>() -> ISyncKnowledge_Vtbl {
        unsafe extern "system" fn GetOwnerReplicaId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::GetOwnerReplicaId(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::Serialize(this, core::mem::transmute_copy(&fserializereplicakeymap), core::mem::transmute_copy(&pbknowledge), core::mem::transmute_copy(&pcbknowledge)).into()
        }
        unsafe extern "system" fn SetLocalTickCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulltickcount: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::SetLocalTickCount(this, core::mem::transmute_copy(&ulltickcount)).into()
        }
        unsafe extern "system" fn ContainsChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::ContainsChange(this, core::mem::transmute_copy(&pbversionownerreplicaid), core::mem::transmute_copy(&pgiditemid), core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn ContainsChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::ContainsChangeUnit(this, core::mem::transmute_copy(&pbversionownerreplicaid), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&psyncversion)).into()
        }
        unsafe extern "system" fn GetScopeVector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::GetScopeVector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetReplicaKeyMap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppreplicakeymap: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge_Impl::GetReplicaKeyMap(this) {
                Ok(ok__) => {
                    core::ptr::write(ppreplicakeymap, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclonedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppclonedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledgein: *mut core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::ConvertVersion(this, windows_core::from_raw_borrowed(&pknowledgein), core::mem::transmute_copy(&pbcurrentownerid), core::mem::transmute_copy(&pversionin), core::mem::transmute_copy(&pbnewownerid), core::mem::transmute_copy(&pcbidsize), core::mem::transmute_copy(&pversionout)).into()
        }
        unsafe extern "system" fn MapRemoteToLocal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteknowledge: *mut core::ffi::c_void, ppmappedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge_Impl::MapRemoteToLocal(this, windows_core::from_raw_borrowed(&premoteknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(ppmappedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Union<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::Union(this, windows_core::from_raw_borrowed(&pknowledge)).into()
        }
        unsafe extern "system" fn ProjectOntoItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge_Impl::ProjectOntoItem(this, core::mem::transmute_copy(&pbitemid)) {
                Ok(ok__) => {
                    core::ptr::write(ppknowledgeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge_Impl::ProjectOntoChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)) {
                Ok(ok__) => {
                    core::ptr::write(ppknowledgeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge_Impl::ProjectOntoRange(this, core::mem::transmute_copy(&psrngsyncrange)) {
                Ok(ok__) => {
                    core::ptr::write(ppknowledgeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExcludeItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::ExcludeItem(this, core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ExcludeChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::ExcludeChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ContainsKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::ContainsKnowledge(this, windows_core::from_raw_borrowed(&pknowledge)).into()
        }
        unsafe extern "system" fn FindMinTickCountForReplica<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::FindMinTickCountForReplica(this, core::mem::transmute_copy(&pbreplicaid), core::mem::transmute_copy(&pullreplicatickcount)).into()
        }
        unsafe extern "system" fn GetRangeExceptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::GetRangeExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetSingleItemExceptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::GetSingleItemExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetChangeUnitExceptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::GetChangeUnitExceptions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::FindClockVectorForItem(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn FindClockVectorForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::FindClockVectorForChangeUnit(this, core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge_Impl::GetVersion(this, core::mem::transmute_copy(&pdwversion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOwnerReplicaId: GetOwnerReplicaId::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            SetLocalTickCount: SetLocalTickCount::<Identity, Impl, OFFSET>,
            ContainsChange: ContainsChange::<Identity, Impl, OFFSET>,
            ContainsChangeUnit: ContainsChangeUnit::<Identity, Impl, OFFSET>,
            GetScopeVector: GetScopeVector::<Identity, Impl, OFFSET>,
            GetReplicaKeyMap: GetReplicaKeyMap::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            ConvertVersion: ConvertVersion::<Identity, Impl, OFFSET>,
            MapRemoteToLocal: MapRemoteToLocal::<Identity, Impl, OFFSET>,
            Union: Union::<Identity, Impl, OFFSET>,
            ProjectOntoItem: ProjectOntoItem::<Identity, Impl, OFFSET>,
            ProjectOntoChangeUnit: ProjectOntoChangeUnit::<Identity, Impl, OFFSET>,
            ProjectOntoRange: ProjectOntoRange::<Identity, Impl, OFFSET>,
            ExcludeItem: ExcludeItem::<Identity, Impl, OFFSET>,
            ExcludeChangeUnit: ExcludeChangeUnit::<Identity, Impl, OFFSET>,
            ContainsKnowledge: ContainsKnowledge::<Identity, Impl, OFFSET>,
            FindMinTickCountForReplica: FindMinTickCountForReplica::<Identity, Impl, OFFSET>,
            GetRangeExceptions: GetRangeExceptions::<Identity, Impl, OFFSET>,
            GetSingleItemExceptions: GetSingleItemExceptions::<Identity, Impl, OFFSET>,
            GetChangeUnitExceptions: GetChangeUnitExceptions::<Identity, Impl, OFFSET>,
            FindClockVectorForItem: FindClockVectorForItem::<Identity, Impl, OFFSET>,
            FindClockVectorForChangeUnit: FindClockVectorForChangeUnit::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>() -> ISyncKnowledge2_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn ProjectOntoColumnSet<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge2_Impl::ProjectOntoColumnSet(this, core::mem::transmute_copy(&ppcolumns), core::mem::transmute_copy(&count)) {
                Ok(ok__) => {
                    core::ptr::write(ppiknowledgeout, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerializeWithOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::SerializeWithOptions(this, core::mem::transmute_copy(&targetformatversion), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwserializedsize)).into()
        }
        unsafe extern "system" fn GetLowestUncontainedId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisyncknowledge: *mut core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::GetLowestUncontainedId(this, windows_core::from_raw_borrowed(&pisyncknowledge), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pcbitemidsize)).into()
        }
        unsafe extern "system" fn GetInspector<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppiinspector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::GetInspector(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppiinspector)).into()
        }
        unsafe extern "system" fn GetMinimumSupportedVersion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::GetMinimumSupportedVersion(this, core::mem::transmute_copy(&pversion)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::GetStatistics(this, core::mem::transmute_copy(&which), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pbitemid: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::ContainsKnowledgeForItem(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pbitemid)).into()
        }
        unsafe extern "system" fn ContainsKnowledgeForChangeUnit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledge: *mut core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::ContainsKnowledgeForChangeUnit(this, windows_core::from_raw_borrowed(&pknowledge), core::mem::transmute_copy(&pbitemid), core::mem::transmute_copy(&pbchangeunitid)).into()
        }
        unsafe extern "system" fn ProjectOntoKnowledgeWithPrerequisite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprerequisiteknowledge: *mut core::ffi::c_void, ptemplateknowledge: *mut core::ffi::c_void, ppprojectedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge2_Impl::ProjectOntoKnowledgeWithPrerequisite(this, windows_core::from_raw_borrowed(&pprerequisiteknowledge), windows_core::from_raw_borrowed(&ptemplateknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(ppprojectedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complement<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncknowledge: *mut core::ffi::c_void, ppcomplementedknowledge: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge2_Impl::Complement(this, windows_core::from_raw_borrowed(&psyncknowledge)) {
                Ok(ok__) => {
                    core::ptr::write(ppcomplementedknowledge, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntersectsWithKnowledge<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncknowledge: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::IntersectsWithKnowledge(this, windows_core::from_raw_borrowed(&psyncknowledge)).into()
        }
        unsafe extern "system" fn GetKnowledgeCookie<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppknowledgecookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncKnowledge2_Impl::GetKnowledgeCookie(this) {
                Ok(ok__) => {
                    core::ptr::write(ppknowledgecookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareToKnowledgeCookie<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncKnowledge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pknowledgecookie: *mut core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncKnowledge2_Impl::CompareToKnowledgeCookie(this, windows_core::from_raw_borrowed(&pknowledgecookie), core::mem::transmute_copy(&presult)).into()
        }
        Self {
            base__: ISyncKnowledge_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            ProjectOntoColumnSet: ProjectOntoColumnSet::<Identity, Impl, OFFSET>,
            SerializeWithOptions: SerializeWithOptions::<Identity, Impl, OFFSET>,
            GetLowestUncontainedId: GetLowestUncontainedId::<Identity, Impl, OFFSET>,
            GetInspector: GetInspector::<Identity, Impl, OFFSET>,
            GetMinimumSupportedVersion: GetMinimumSupportedVersion::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            ContainsKnowledgeForItem: ContainsKnowledgeForItem::<Identity, Impl, OFFSET>,
            ContainsKnowledgeForChangeUnit: ContainsKnowledgeForChangeUnit::<Identity, Impl, OFFSET>,
            ProjectOntoKnowledgeWithPrerequisite: ProjectOntoKnowledgeWithPrerequisite::<Identity, Impl, OFFSET>,
            Complement: Complement::<Identity, Impl, OFFSET>,
            IntersectsWithKnowledge: IntersectsWithKnowledge::<Identity, Impl, OFFSET>,
            GetKnowledgeCookie: GetKnowledgeCookie::<Identity, Impl, OFFSET>,
            CompareToKnowledgeCookie: CompareToKnowledgeCookie::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>() -> ISyncMergeTombstoneChange_Vtbl {
        unsafe extern "system" fn GetWinnerItemId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncMergeTombstoneChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncMergeTombstoneChange_Impl::GetWinnerItemId(this, core::mem::transmute_copy(&pbwinneritemid), core::mem::transmute_copy(&pcbidsize)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWinnerItemId: GetWinnerItemId::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProvider_Impl, const OFFSET: isize>() -> ISyncProvider_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProvider_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>() -> ISyncProviderConfigUI_Vtbl {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pguidcontenttype: *const windows_core::GUID, pconfigurationproperties: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderConfigUI_Impl::Init(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&pguidcontenttype), windows_core::from_raw_borrowed(&pconfigurationproperties)).into()
        }
        unsafe extern "system" fn GetRegisteredProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconfiguiproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderConfigUI_Impl::GetRegisteredProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(ppconfiguiproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndRegisterNewSyncProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut core::ffi::c_void, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderConfigUI_Impl::CreateAndRegisterNewSyncProvider(this, core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&punkcontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppproviderinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifySyncProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut core::ffi::c_void, pproviderinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderConfigUI_Impl::ModifySyncProvider(this, core::mem::transmute_copy(&hwndparent), windows_core::from_raw_borrowed(&punkcontext), windows_core::from_raw_borrowed(&pproviderinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetRegisteredProperties: GetRegisteredProperties::<Identity, Impl, OFFSET>,
            CreateAndRegisterNewSyncProvider: CreateAndRegisterNewSyncProvider::<Identity, Impl, OFFSET>,
            ModifySyncProvider: ModifySyncProvider::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>() -> ISyncProviderConfigUIInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderConfigUI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderConfigUIInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderConfigUIInfo_Impl::GetSyncProviderConfigUI(this, core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppsyncproviderconfigui, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncProviderConfigUI: GetSyncProviderConfigUI::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderInfo_Impl, const OFFSET: isize>() -> ISyncProviderInfo_Vtbl {
        unsafe extern "system" fn GetSyncProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderInfo_Impl::GetSyncProvider(this, core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppsyncprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSyncProvider: GetSyncProvider::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>() -> ISyncProviderRegistration_Vtbl {
        unsafe extern "system" fn CreateSyncProviderConfigUIRegistrationInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::CreateSyncProviderConfigUIRegistrationInstance(this, core::mem::transmute_copy(&pconfiguiconfig)) {
                Ok(ok__) => {
                    core::ptr::write(ppconfiguiinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProviderConfigUI<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderRegistration_Impl::UnregisterSyncProviderConfigUI(this, core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn EnumerateSyncProviderConfigUIs<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontenttype: *const windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::EnumerateSyncProviderConfigUIs(this, core::mem::transmute_copy(&pguidcontenttype), core::mem::transmute_copy(&dwsupportedarchitecture)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumsyncproviderconfiguiinfos, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSyncProviderRegistrationInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::CreateSyncProviderRegistrationInstance(this, core::mem::transmute_copy(&pproviderconfiguration)) {
                Ok(ok__) => {
                    core::ptr::write(ppproviderinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSyncProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderRegistration_Impl::UnregisterSyncProvider(this, core::mem::transmute_copy(&pguidinstanceid)).into()
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfoforProvider<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidproviderinstanceid: *const windows_core::GUID, ppproviderconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIInfoforProvider(this, core::mem::transmute_copy(&pguidproviderinstanceid)) {
                Ok(ok__) => {
                    core::ptr::write(ppproviderconfiguiinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSyncProviders<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidcontenttype: *const windows_core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const windows_core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::EnumerateSyncProviders(this, core::mem::transmute_copy(&pguidcontenttype), core::mem::transmute_copy(&dwstateflagstofiltermask), core::mem::transmute_copy(&dwstateflagstofilter), core::mem::transmute_copy(&refproviderclsid), core::mem::transmute_copy(&dwsupportedarchitecture)) {
                Ok(ok__) => {
                    core::ptr::write(ppenumsyncproviderinfos, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, ppproviderinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetSyncProviderInfo(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    core::ptr::write(ppproviderinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderFromInstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetSyncProviderFromInstanceId(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppsyncprovider, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, ppconfiguiinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIInfo(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    core::ptr::write(ppconfiguiinfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderConfigUIFromInstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwclscontext: u32, ppconfigui: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetSyncProviderConfigUIFromInstanceId(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwclscontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppconfigui, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSyncProviderState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, pdwstateflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetSyncProviderState(this, core::mem::transmute_copy(&pguidinstanceid)) {
                Ok(ok__) => {
                    core::ptr::write(pdwstateflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncProviderState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *const windows_core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderRegistration_Impl::SetSyncProviderState(this, core::mem::transmute_copy(&pguidinstanceid), core::mem::transmute_copy(&dwstateflagsmask), core::mem::transmute_copy(&dwstateflags)).into()
        }
        unsafe extern "system" fn RegisterForEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderRegistration_Impl::RegisterForEvent(this, core::mem::transmute_copy(&phevent)).into()
        }
        unsafe extern "system" fn RevokeEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncProviderRegistration_Impl::RevokeEvent(this, core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn GetChange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncProviderRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncProviderRegistration_Impl::GetChange(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(ppchange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSyncProviderConfigUIRegistrationInstance: CreateSyncProviderConfigUIRegistrationInstance::<Identity, Impl, OFFSET>,
            UnregisterSyncProviderConfigUI: UnregisterSyncProviderConfigUI::<Identity, Impl, OFFSET>,
            EnumerateSyncProviderConfigUIs: EnumerateSyncProviderConfigUIs::<Identity, Impl, OFFSET>,
            CreateSyncProviderRegistrationInstance: CreateSyncProviderRegistrationInstance::<Identity, Impl, OFFSET>,
            UnregisterSyncProvider: UnregisterSyncProvider::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIInfoforProvider: GetSyncProviderConfigUIInfoforProvider::<Identity, Impl, OFFSET>,
            EnumerateSyncProviders: EnumerateSyncProviders::<Identity, Impl, OFFSET>,
            GetSyncProviderInfo: GetSyncProviderInfo::<Identity, Impl, OFFSET>,
            GetSyncProviderFromInstanceId: GetSyncProviderFromInstanceId::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIInfo: GetSyncProviderConfigUIInfo::<Identity, Impl, OFFSET>,
            GetSyncProviderConfigUIFromInstanceId: GetSyncProviderConfigUIFromInstanceId::<Identity, Impl, OFFSET>,
            GetSyncProviderState: GetSyncProviderState::<Identity, Impl, OFFSET>,
            SetSyncProviderState: SetSyncProviderState::<Identity, Impl, OFFSET>,
            RegisterForEvent: RegisterForEvent::<Identity, Impl, OFFSET>,
            RevokeEvent: RevokeEvent::<Identity, Impl, OFFSET>,
            GetChange: GetChange::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>() -> ISyncRegistrationChange_Vtbl {
        unsafe extern "system" fn GetEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncRegistrationChange_Impl::GetEvent(this) {
                Ok(ok__) => {
                    core::ptr::write(psreevent, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncRegistrationChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidinstanceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncRegistrationChange_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    core::ptr::write(pguidinstanceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>() -> ISyncSessionExtendedErrorInfo_Vtbl {
        unsafe extern "system" fn GetSyncProviderWithError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionExtendedErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproviderwitherror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISyncSessionExtendedErrorInfo_Impl::GetSyncProviderWithError(this) {
                Ok(ok__) => {
                    core::ptr::write(ppproviderwitherror, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSyncProviderWithError: GetSyncProviderWithError::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>() -> ISyncSessionState_Vtbl {
        unsafe extern "system" fn IsCanceled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::IsCanceled(this, core::mem::transmute_copy(&pfiscanceled)).into()
        }
        unsafe extern "system" fn GetInfoForChangeApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::GetInfoForChangeApplication(this, core::mem::transmute_copy(&pbchangeapplierinfo), core::mem::transmute_copy(&pcbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn LoadInfoFromChangeApplication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::LoadInfoFromChangeApplication(this, core::mem::transmute_copy(&pbchangeapplierinfo), core::mem::transmute_copy(&cbchangeapplierinfo)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeStart<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::GetForgottenKnowledgeRecoveryRangeStart(this, core::mem::transmute_copy(&pbrangestart), core::mem::transmute_copy(&pcbrangestart)).into()
        }
        unsafe extern "system" fn GetForgottenKnowledgeRecoveryRangeEnd<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::GetForgottenKnowledgeRecoveryRangeEnd(this, core::mem::transmute_copy(&pbrangeend), core::mem::transmute_copy(&pcbrangeend)).into()
        }
        unsafe extern "system" fn SetForgottenKnowledgeRecoveryRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prange: *const SYNC_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::SetForgottenKnowledgeRecoveryRange(this, core::mem::transmute_copy(&prange)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState_Impl::OnProgress(this, core::mem::transmute_copy(&provider), core::mem::transmute_copy(&syncstage), core::mem::transmute_copy(&dwcompletedwork), core::mem::transmute_copy(&dwtotalwork)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsCanceled: IsCanceled::<Identity, Impl, OFFSET>,
            GetInfoForChangeApplication: GetInfoForChangeApplication::<Identity, Impl, OFFSET>,
            LoadInfoFromChangeApplication: LoadInfoFromChangeApplication::<Identity, Impl, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeStart: GetForgottenKnowledgeRecoveryRangeStart::<Identity, Impl, OFFSET>,
            GetForgottenKnowledgeRecoveryRangeEnd: GetForgottenKnowledgeRecoveryRangeEnd::<Identity, Impl, OFFSET>,
            SetForgottenKnowledgeRecoveryRange: SetForgottenKnowledgeRecoveryRange::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState2_Impl, const OFFSET: isize>() -> ISyncSessionState2_Vtbl {
        unsafe extern "system" fn SetProviderWithError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fself: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState2_Impl::SetProviderWithError(this, core::mem::transmute_copy(&fself)).into()
        }
        unsafe extern "system" fn GetSessionErrorStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISyncSessionState2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrsessionerror: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISyncSessionState2_Impl::GetSessionErrorStatus(this, core::mem::transmute_copy(&phrsessionerror)).into()
        }
        Self {
            base__: ISyncSessionState_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProviderWithError: SetProviderWithError::<Identity, Impl, OFFSET>,
            GetSessionErrorStatus: GetSessionErrorStatus::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>() -> ISynchronousDataRetriever_Vtbl {
        unsafe extern "system" fn GetIdParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ISynchronousDataRetriever_Impl::GetIdParameters(this, core::mem::transmute_copy(&pidparameters)).into()
        }
        unsafe extern "system" fn LoadChangeData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ISynchronousDataRetriever_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploadchangecontext: *mut core::ffi::c_void, ppunkdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ISynchronousDataRetriever_Impl::LoadChangeData(this, windows_core::from_raw_borrowed(&ploadchangecontext)) {
                Ok(ok__) => {
                    core::ptr::write(ppunkdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdParameters: GetIdParameters::<Identity, Impl, OFFSET>,
            LoadChangeData: LoadChangeData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronousDataRetriever as windows_core::Interface>::IID
    }
}
