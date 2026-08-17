#[cfg(feature = "Win32_System_Com")]
pub trait ISdo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetPropertyInfo(&self, id: i32) -> windows_core::Result<windows_core::IUnknown>;
    fn GetProperty(&self, id: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn PutProperty(&self, id: i32, pvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn ResetProperty(&self, id: i32) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISdo {}
#[cfg(feature = "Win32_System_Com")]
impl ISdo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISdo_Vtbl
    where
        Identity: ISdo_Impl,
    {
        unsafe extern "system" fn GetPropertyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdo_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    pppropertyinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdo_Impl::GetProperty(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32, pvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdo_Impl::PutProperty(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ResetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: i32) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdo_Impl::ResetProperty(this, core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Apply<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdo_Impl::Apply(this).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdo_Impl::Restore(this).into()
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdo_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            PutProperty: PutProperty::<Identity, OFFSET>,
            ResetProperty: ResetProperty::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, bstrname: &windows_core::BSTR, ppitem: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn Remove(&self, pitem: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn Reload(&self) -> windows_core::Result<()>;
    fn IsNameUnique(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Item(&self, name: *const windows_core::VARIANT) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISdoCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ISdoCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISdoCollection_Vtbl
    where
        Identity: ISdoCollection_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoCollection_Impl::Add(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoCollection_Impl::Remove(this, windows_core::from_raw_borrowed(&pitem)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoCollection_Impl::RemoveAll(this).into()
        }
        unsafe extern "system" fn Reload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoCollection_Impl::Reload(this).into()
        }
        unsafe extern "system" fn IsNameUnique<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoCollection_Impl::IsNameUnique(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    pbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const core::mem::MaybeUninit<windows_core::VARIANT>, pitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoCollection_Impl::Item(this, core::mem::transmute_copy(&name)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenumvariant.write(core::mem::transmute(ok__));
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
            RemoveAll: RemoveAll::<Identity, OFFSET>,
            Reload: Reload::<Identity, OFFSET>,
            IsNameUnique: IsNameUnique::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoDictionaryOld_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumAttributes(&self, id: *mut windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn GetAttributeInfo(&self, id: ATTRIBUTEID, pinfoids: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn EnumAttributeValues(&self, id: ATTRIBUTEID, pvalueids: *mut windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn CreateAttribute(&self, id: ATTRIBUTEID) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetAttributeID(&self, bstrattributename: &windows_core::BSTR) -> windows_core::Result<ATTRIBUTEID>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISdoDictionaryOld {}
#[cfg(feature = "Win32_System_Com")]
impl ISdoDictionaryOld_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISdoDictionaryOld_Vtbl
    where
        Identity: ISdoDictionaryOld_Impl,
    {
        unsafe extern "system" fn EnumAttributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pvalues: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISdoDictionaryOld_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoDictionaryOld_Impl::EnumAttributes(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    pvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const core::mem::MaybeUninit<windows_core::VARIANT>, pinfovalues: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISdoDictionaryOld_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoDictionaryOld_Impl::GetAttributeInfo(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pinfoids)) {
                Ok(ok__) => {
                    pinfovalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttributeValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut core::mem::MaybeUninit<windows_core::VARIANT>, pvaluesdesc: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISdoDictionaryOld_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoDictionaryOld_Impl::EnumAttributeValues(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pvalueids)) {
                Ok(ok__) => {
                    pvaluesdesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoDictionaryOld_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoDictionaryOld_Impl::CreateAttribute(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    ppattributeobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattributename: core::mem::MaybeUninit<windows_core::BSTR>, pid: *mut ATTRIBUTEID) -> windows_core::HRESULT
        where
            Identity: ISdoDictionaryOld_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoDictionaryOld_Impl::GetAttributeID(this, core::mem::transmute(&bstrattributename)) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, OFFSET>,
            EnumAttributeValues: EnumAttributeValues::<Identity, OFFSET>,
            CreateAttribute: CreateAttribute::<Identity, OFFSET>,
            GetAttributeID: GetAttributeID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoDictionaryOld as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoMachine_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attach(&self, bstrcomputername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDictionarySDO(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetServiceSDO(&self, edatastore: IASDATASTORE, bstrservicename: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetUserSDO(&self, edatastore: IASDATASTORE, bstrusername: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn GetOSType(&self) -> windows_core::Result<IASOSTYPE>;
    fn GetDomainType(&self) -> windows_core::Result<IASDOMAINTYPE>;
    fn IsDirectoryAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAttachedComputer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSDOSchema(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISdoMachine {}
#[cfg(feature = "Win32_System_Com")]
impl ISdoMachine_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISdoMachine_Vtbl
    where
        Identity: ISdoMachine_Impl,
    {
        unsafe extern "system" fn Attach<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcomputername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoMachine_Impl::Attach(this, core::mem::transmute(&bstrcomputername)).into()
        }
        unsafe extern "system" fn GetDictionarySDO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdictionarysdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetDictionarySDO(this) {
                Ok(ok__) => {
                    ppdictionarysdo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceSDO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: core::mem::MaybeUninit<windows_core::BSTR>, ppservicesdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetServiceSDO(this, core::mem::transmute_copy(&edatastore), core::mem::transmute(&bstrservicename)) {
                Ok(ok__) => {
                    ppservicesdo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSDO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: core::mem::MaybeUninit<windows_core::BSTR>, ppusersdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetUserSDO(this, core::mem::transmute_copy(&edatastore), core::mem::transmute(&bstrusername)) {
                Ok(ok__) => {
                    ppusersdo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eostype: *mut IASOSTYPE) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetOSType(this) {
                Ok(ok__) => {
                    eostype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetDomainType(this) {
                Ok(ok__) => {
                    edomaintype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirectoryAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, booldirectoryavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::IsDirectoryAvailable(this) {
                Ok(ok__) => {
                    booldirectoryavailable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedComputer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcomputername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetAttachedComputer(this) {
                Ok(ok__) => {
                    bstrcomputername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSDOSchema<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsdoschema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine_Impl::GetSDOSchema(this) {
                Ok(ok__) => {
                    ppsdoschema.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Attach: Attach::<Identity, OFFSET>,
            GetDictionarySDO: GetDictionarySDO::<Identity, OFFSET>,
            GetServiceSDO: GetServiceSDO::<Identity, OFFSET>,
            GetUserSDO: GetUserSDO::<Identity, OFFSET>,
            GetOSType: GetOSType::<Identity, OFFSET>,
            GetDomainType: GetDomainType::<Identity, OFFSET>,
            IsDirectoryAvailable: IsDirectoryAvailable::<Identity, OFFSET>,
            GetAttachedComputer: GetAttachedComputer::<Identity, OFFSET>,
            GetSDOSchema: GetSDOSchema::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoMachine as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoMachine2_Impl: Sized + ISdoMachine_Impl {
    fn GetTemplatesSDO(&self, bstrservicename: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn EnableTemplates(&self) -> windows_core::Result<()>;
    fn SyncConfigAgainstTemplates(&self, bstrservicename: &windows_core::BSTR, ppconfigroot: *mut Option<windows_core::IUnknown>, pptemplatesroot: *mut Option<windows_core::IUnknown>, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ImportRemoteTemplates(&self, plocaltemplatesroot: Option<&windows_core::IUnknown>, bstrremotemachinename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Reload(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISdoMachine2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISdoMachine2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISdoMachine2_Vtbl
    where
        Identity: ISdoMachine2_Impl,
    {
        unsafe extern "system" fn GetTemplatesSDO<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservicename: core::mem::MaybeUninit<windows_core::BSTR>, pptemplatessdo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoMachine2_Impl::GetTemplatesSDO(this, core::mem::transmute(&bstrservicename)) {
                Ok(ok__) => {
                    pptemplatessdo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTemplates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoMachine2_Impl::EnableTemplates(this).into()
        }
        unsafe extern "system" fn SyncConfigAgainstTemplates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservicename: core::mem::MaybeUninit<windows_core::BSTR>, ppconfigroot: *mut *mut core::ffi::c_void, pptemplatesroot: *mut *mut core::ffi::c_void, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: ISdoMachine2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoMachine2_Impl::SyncConfigAgainstTemplates(this, core::mem::transmute(&bstrservicename), core::mem::transmute_copy(&ppconfigroot), core::mem::transmute_copy(&pptemplatesroot), core::mem::transmute_copy(&bforcedsync)).into()
        }
        unsafe extern "system" fn ImportRemoteTemplates<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plocaltemplatesroot: *mut core::ffi::c_void, bstrremotemachinename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISdoMachine2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoMachine2_Impl::ImportRemoteTemplates(this, windows_core::from_raw_borrowed(&plocaltemplatesroot), core::mem::transmute(&bstrremotemachinename)).into()
        }
        unsafe extern "system" fn Reload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoMachine2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoMachine2_Impl::Reload(this).into()
        }
        Self {
            base__: ISdoMachine_Vtbl::new::<Identity, OFFSET>(),
            GetTemplatesSDO: GetTemplatesSDO::<Identity, OFFSET>,
            EnableTemplates: EnableTemplates::<Identity, OFFSET>,
            SyncConfigAgainstTemplates: SyncConfigAgainstTemplates::<Identity, OFFSET>,
            ImportRemoteTemplates: ImportRemoteTemplates::<Identity, OFFSET>,
            Reload: Reload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoMachine2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISdoMachine as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISdoServiceControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartService(&self) -> windows_core::Result<()>;
    fn StopService(&self) -> windows_core::Result<()>;
    fn GetServiceStatus(&self) -> windows_core::Result<i32>;
    fn ResetService(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISdoServiceControl {}
#[cfg(feature = "Win32_System_Com")]
impl ISdoServiceControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISdoServiceControl_Vtbl
    where
        Identity: ISdoServiceControl_Impl,
    {
        unsafe extern "system" fn StartService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoServiceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoServiceControl_Impl::StartService(this).into()
        }
        unsafe extern "system" fn StopService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoServiceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoServiceControl_Impl::StopService(this).into()
        }
        unsafe extern "system" fn GetServiceStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISdoServiceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISdoServiceControl_Impl::GetServiceStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISdoServiceControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISdoServiceControl_Impl::ResetService(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartService: StartService::<Identity, OFFSET>,
            StopService: StopService::<Identity, OFFSET>,
            GetServiceStatus: GetServiceStatus::<Identity, OFFSET>,
            ResetService: ResetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISdoServiceControl as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITemplateSdo_Impl: Sized + ISdo_Impl {
    fn AddToCollection(&self, bstrname: &windows_core::BSTR, pcollection: Option<&super::super::System::Com::IDispatch>, ppitem: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn AddToSdo(&self, bstrname: &windows_core::BSTR, psdotarget: Option<&super::super::System::Com::IDispatch>, ppitem: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn AddToSdoAsProperty(&self, psdotarget: Option<&super::super::System::Com::IDispatch>, id: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITemplateSdo {}
#[cfg(feature = "Win32_System_Com")]
impl ITemplateSdo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITemplateSdo_Vtbl
    where
        Identity: ITemplateSdo_Impl,
    {
        unsafe extern "system" fn AddToCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pcollection: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITemplateSdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITemplateSdo_Impl::AddToCollection(this, core::mem::transmute(&bstrname), windows_core::from_raw_borrowed(&pcollection), core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn AddToSdo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, psdotarget: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITemplateSdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITemplateSdo_Impl::AddToSdo(this, core::mem::transmute(&bstrname), windows_core::from_raw_borrowed(&psdotarget), core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn AddToSdoAsProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psdotarget: *mut core::ffi::c_void, id: i32) -> windows_core::HRESULT
        where
            Identity: ITemplateSdo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITemplateSdo_Impl::AddToSdoAsProperty(this, windows_core::from_raw_borrowed(&psdotarget), core::mem::transmute_copy(&id)).into()
        }
        Self {
            base__: ISdo_Vtbl::new::<Identity, OFFSET>(),
            AddToCollection: AddToCollection::<Identity, OFFSET>,
            AddToSdo: AddToSdo::<Identity, OFFSET>,
            AddToSdoAsProperty: AddToSdoAsProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITemplateSdo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISdo as windows_core::Interface>::IID
    }
}
