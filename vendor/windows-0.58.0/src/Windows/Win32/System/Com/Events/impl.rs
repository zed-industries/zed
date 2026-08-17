pub trait IDontSupportEventSubscription_Impl: Sized {}
impl windows_core::RuntimeName for IDontSupportEventSubscription {}
impl IDontSupportEventSubscription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDontSupportEventSubscription_Vtbl
    where
        Identity: IDontSupportEventSubscription_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDontSupportEventSubscription as windows_core::Interface>::IID
    }
}
pub trait IEnumEventObject_Impl: Sized {
    fn Clone(&self) -> windows_core::Result<IEnumEventObject>;
    fn Next(&self, creqelem: u32, ppinterface: *mut Option<windows_core::IUnknown>, cretelem: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::HRESULT;
    fn Skip(&self, cskipelem: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumEventObject {}
impl IEnumEventObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumEventObject_Vtbl
    where
        Identity: IEnumEventObject_Impl,
    {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumEventObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumEventObject_Impl::Clone(this) {
                Ok(ok__) => {
                    ppinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut core::ffi::c_void, cretelem: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumEventObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEventObject_Impl::Next(this, core::mem::transmute_copy(&creqelem), core::mem::transmute_copy(&ppinterface), core::mem::transmute_copy(&cretelem)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumEventObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEventObject_Impl::Reset(this)
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskipelem: u32) -> windows_core::HRESULT
        where
            Identity: IEnumEventObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEventObject_Impl::Skip(this, core::mem::transmute_copy(&cskipelem)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumEventObject as windows_core::Interface>::IID
    }
}
pub trait IEventClass_Impl: Sized + super::IDispatch_Impl {
    fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventClassName(&self, bstreventclassname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FiringInterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CustomConfigCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TypeLib(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTypeLib(&self, bstrtypelib: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventClass {}
impl IEventClass_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventClass_Vtbl
    where
        Identity: IEventClass_Impl,
    {
        unsafe extern "system" fn EventClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::EventClassID(this) {
                Ok(ok__) => {
                    pbstreventclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetEventClassID(this, core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn EventClassName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::EventClassName(this) {
                Ok(ok__) => {
                    pbstreventclassname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetEventClassName(this, core::mem::transmute(&bstreventclassname)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    pbstrownersid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn FiringInterfaceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfiringinterfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::FiringInterfaceID(this) {
                Ok(ok__) => {
                    pbstrfiringinterfaceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiringInterfaceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfiringinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetFiringInterfaceID(this, core::mem::transmute(&bstrfiringinterfaceid)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::Description(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn CustomConfigCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcustomconfigclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::CustomConfigCLSID(this) {
                Ok(ok__) => {
                    pbstrcustomconfigclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcustomconfigclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetCustomConfigCLSID(this, core::mem::transmute(&bstrcustomconfigclsid)).into()
        }
        unsafe extern "system" fn TypeLib<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtypelib: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::TypeLib(this) {
                Ok(ok__) => {
                    pbstrtypelib.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeLib<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypelib: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetTypeLib(this, core::mem::transmute(&bstrtypelib)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EventClassID: EventClassID::<Identity, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, OFFSET>,
            EventClassName: EventClassName::<Identity, OFFSET>,
            SetEventClassName: SetEventClassName::<Identity, OFFSET>,
            OwnerSID: OwnerSID::<Identity, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, OFFSET>,
            FiringInterfaceID: FiringInterfaceID::<Identity, OFFSET>,
            SetFiringInterfaceID: SetFiringInterfaceID::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            CustomConfigCLSID: CustomConfigCLSID::<Identity, OFFSET>,
            SetCustomConfigCLSID: SetCustomConfigCLSID::<Identity, OFFSET>,
            TypeLib: TypeLib::<Identity, OFFSET>,
            SetTypeLib: SetTypeLib::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventClass as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEventClass2_Impl: Sized + IEventClass_Impl {
    fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MultiInterfacePublisherFilterCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMultiInterfacePublisherFilterCLSID(&self, bstrpubfilclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventClass2 {}
impl IEventClass2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventClass2_Vtbl
    where
        Identity: IEventClass2_Impl,
    {
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::PublisherID(this) {
                Ok(ok__) => {
                    pbstrpublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpubfilclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::MultiInterfacePublisherFilterCLSID(this) {
                Ok(ok__) => {
                    pbstrpubfilclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpubfilclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetMultiInterfacePublisherFilterCLSID(this, core::mem::transmute(&bstrpubfilclsid)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    pfallowinprocactivation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::FireInParallel(this) {
                Ok(ok__) => {
                    pffireinparallel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventClass2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetFireInParallel(this, core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: IEventClass_Vtbl::new::<Identity, OFFSET>(),
            PublisherID: PublisherID::<Identity, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, OFFSET>,
            MultiInterfacePublisherFilterCLSID: MultiInterfacePublisherFilterCLSID::<Identity, OFFSET>,
            SetMultiInterfacePublisherFilterCLSID: SetMultiInterfacePublisherFilterCLSID::<Identity, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, OFFSET>,
            FireInParallel: FireInParallel::<Identity, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventClass2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IEventClass as windows_core::Interface>::IID
    }
}
pub trait IEventControl_Impl: Sized + super::IDispatch_Impl {
    fn SetPublisherFilter(&self, methodname: &windows_core::BSTR, ppublisherfilter: Option<&IPublisherFilter>) -> windows_core::Result<()>;
    fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSubscriptions(&self, methodname: &windows_core::BSTR, optionalcriteria: &windows_core::BSTR, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, methodname: &windows_core::BSTR, criteria: &windows_core::BSTR) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IEventControl {}
impl IEventControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventControl_Vtbl
    where
        Identity: IEventControl_Impl,
    {
        unsafe extern "system" fn SetPublisherFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, ppublisherfilter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventControl_Impl::SetPublisherFilter(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&ppublisherfilter)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventControl_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    pfallowinprocactivation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventControl_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, optionalcriteria: core::mem::MaybeUninit<windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventControl_Impl::GetSubscriptions(this, core::mem::transmute(&methodname), core::mem::transmute(&optionalcriteria), core::mem::transmute_copy(&optionalerrorindex)) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, criteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT
        where
            Identity: IEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventControl_Impl::SetDefaultQuery(this, core::mem::transmute(&methodname), core::mem::transmute(&criteria)) {
                Ok(ok__) => {
                    errorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetPublisherFilter: SetPublisherFilter::<Identity, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventControl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEventObjectChange_Impl: Sized {
    fn ChangedSubscription(&self, changetype: EOC_ChangeType, bstrsubscriptionid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangedEventClass(&self, changetype: EOC_ChangeType, bstreventclassid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangedPublisher(&self, changetype: EOC_ChangeType, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventObjectChange {}
impl IEventObjectChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventObjectChange_Vtbl
    where
        Identity: IEventObjectChange_Impl,
    {
        unsafe extern "system" fn ChangedSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventObjectChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange_Impl::ChangedSubscription(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventObjectChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange_Impl::ChangedEventClass(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn ChangedPublisher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventObjectChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange_Impl::ChangedPublisher(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstrpublisherid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, OFFSET>,
            ChangedPublisher: ChangedPublisher::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventObjectChange as windows_core::Interface>::IID
    }
}
pub trait IEventObjectChange2_Impl: Sized {
    fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()>;
    fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventObjectChange2 {}
impl IEventObjectChange2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventObjectChange2_Vtbl
    where
        Identity: IEventObjectChange2_Impl,
    {
        unsafe extern "system" fn ChangedSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT
        where
            Identity: IEventObjectChange2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange2_Impl::ChangedSubscription(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT
        where
            Identity: IEventObjectChange2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange2_Impl::ChangedEventClass(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventObjectChange2 as windows_core::Interface>::IID
    }
}
pub trait IEventObjectCollection_Impl: Sized + super::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, objectid: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn NewEnum(&self) -> windows_core::Result<IEnumEventObject>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, item: *const windows_core::VARIANT, objectid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, objectid: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventObjectCollection {}
impl IEventObjectCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventObjectCollection_Vtbl
    where
        Identity: IEventObjectCollection_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunkenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::get_Item(this, core::mem::transmute(&objectid)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IEventObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *const core::mem::MaybeUninit<windows_core::VARIANT>, objectid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectCollection_Impl::Add(this, core::mem::transmute_copy(&item), core::mem::transmute(&objectid)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventObjectCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectCollection_Impl::Remove(this, core::mem::transmute(&objectid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            NewEnum: NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventObjectCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEventProperty_Impl: Sized + super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventProperty {}
impl IEventProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventProperty_Vtbl
    where
        Identity: IEventProperty_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventProperty_Impl::Name(this) {
                Ok(ok__) => {
                    propertyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventProperty_Impl::SetName(this, core::mem::transmute(&propertyname)).into()
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventProperty_Impl::Value(this) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventProperty_Impl::SetValue(this, core::mem::transmute_copy(&propertyvalue)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEventPublisher_Impl: Sized + super::IDispatch_Impl {
    fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PublisherName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherName(&self, bstrpublishername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PublisherType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherType(&self, bstrpublishertype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDefaultProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn PutDefaultProperty(&self, bstrpropertyname: &windows_core::BSTR, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn RemoveDefaultProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDefaultPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection>;
}
impl windows_core::RuntimeName for IEventPublisher {}
impl IEventPublisher_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventPublisher_Vtbl
    where
        Identity: IEventPublisher_Impl,
    {
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::PublisherID(this) {
                Ok(ok__) => {
                    pbstrpublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn PublisherName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublishername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::PublisherName(this) {
                Ok(ok__) => {
                    pbstrpublishername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublishername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetPublisherName(this, core::mem::transmute(&bstrpublishername)).into()
        }
        unsafe extern "system" fn PublisherType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublishertype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::PublisherType(this) {
                Ok(ok__) => {
                    pbstrpublishertype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublishertype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetPublisherType(this, core::mem::transmute(&bstrpublishertype)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    pbstrownersid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::Description(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn GetDefaultProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::GetDefaultProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutDefaultProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::PutDefaultProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::RemoveDefaultProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventPublisher_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::GetDefaultPropertyCollection(this) {
                Ok(ok__) => {
                    collection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PublisherID: PublisherID::<Identity, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, OFFSET>,
            PublisherName: PublisherName::<Identity, OFFSET>,
            SetPublisherName: SetPublisherName::<Identity, OFFSET>,
            PublisherType: PublisherType::<Identity, OFFSET>,
            SetPublisherType: SetPublisherType::<Identity, OFFSET>,
            OwnerSID: OwnerSID::<Identity, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDefaultProperty: GetDefaultProperty::<Identity, OFFSET>,
            PutDefaultProperty: PutDefaultProperty::<Identity, OFFSET>,
            RemoveDefaultProperty: RemoveDefaultProperty::<Identity, OFFSET>,
            GetDefaultPropertyCollection: GetDefaultPropertyCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventPublisher as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEventSubscription_Impl: Sized + super::IDispatch_Impl {
    fn SubscriptionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubscriptionID(&self, bstrsubscriptionid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubscriptionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubscriptionName(&self, bstrsubscriptionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MethodName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMethodName(&self, bstrmethodname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubscriberCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubscriberCLSID(&self, bstrsubscriberclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubscriberInterface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetSubscriberInterface(&self, psubscriberinterface: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn PerUser(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetPerUser(&self, fperuser: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenabled: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MachineName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMachineName(&self, bstrmachinename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPublisherProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn PutPublisherProperty(&self, bstrpropertyname: &windows_core::BSTR, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn RemovePublisherProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPublisherPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection>;
    fn GetSubscriberProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn PutSubscriberProperty(&self, bstrpropertyname: &windows_core::BSTR, propertyvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn RemoveSubscriberProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSubscriberPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection>;
    fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInterfaceID(&self, bstrinterfaceid: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventSubscription {}
impl IEventSubscription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventSubscription_Vtbl
    where
        Identity: IEventSubscription_Impl,
    {
        unsafe extern "system" fn SubscriptionID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriptionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriptionID(this) {
                Ok(ok__) => {
                    pbstrsubscriptionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriptionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriptionID(this, core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn SubscriptionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriptionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriptionName(this) {
                Ok(ok__) => {
                    pbstrsubscriptionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriptionname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriptionName(this, core::mem::transmute(&bstrsubscriptionname)).into()
        }
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::PublisherID(this) {
                Ok(ok__) => {
                    pbstrpublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn EventClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::EventClassID(this) {
                Ok(ok__) => {
                    pbstreventclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetEventClassID(this, core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn MethodName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmethodname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::MethodName(this) {
                Ok(ok__) => {
                    pbstrmethodname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetMethodName(this, core::mem::transmute(&bstrmethodname)).into()
        }
        unsafe extern "system" fn SubscriberCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriberclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriberCLSID(this) {
                Ok(ok__) => {
                    pbstrsubscriberclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriberclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriberCLSID(this, core::mem::transmute(&bstrsubscriberclsid)).into()
        }
        unsafe extern "system" fn SubscriberInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubscriberinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriberInterface(this) {
                Ok(ok__) => {
                    ppsubscriberinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubscriberinterface: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriberInterface(this, windows_core::from_raw_borrowed(&psubscriberinterface)).into()
        }
        unsafe extern "system" fn PerUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::PerUser(this) {
                Ok(ok__) => {
                    pfperuser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetPerUser(this, core::mem::transmute_copy(&fperuser)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    pbstrownersid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::Enabled(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::Description(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MachineName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachinename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::MachineName(this) {
                Ok(ok__) => {
                    pbstrmachinename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmachinename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetMachineName(this, core::mem::transmute(&bstrmachinename)).into()
        }
        unsafe extern "system" fn GetPublisherProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetPublisherProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPublisherProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::PutPublisherProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemovePublisherProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::RemovePublisherProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetPublisherPropertyCollection(this) {
                Ok(ok__) => {
                    collection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetSubscriberProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutSubscriberProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::PutSubscriberProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::RemoveSubscriberProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetSubscriberPropertyCollection(this) {
                Ok(ok__) => {
                    collection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinterfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    pbstrinterfaceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetInterfaceID(this, core::mem::transmute(&bstrinterfaceid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SubscriptionID: SubscriptionID::<Identity, OFFSET>,
            SetSubscriptionID: SetSubscriptionID::<Identity, OFFSET>,
            SubscriptionName: SubscriptionName::<Identity, OFFSET>,
            SetSubscriptionName: SetSubscriptionName::<Identity, OFFSET>,
            PublisherID: PublisherID::<Identity, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, OFFSET>,
            EventClassID: EventClassID::<Identity, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, OFFSET>,
            MethodName: MethodName::<Identity, OFFSET>,
            SetMethodName: SetMethodName::<Identity, OFFSET>,
            SubscriberCLSID: SubscriberCLSID::<Identity, OFFSET>,
            SetSubscriberCLSID: SetSubscriberCLSID::<Identity, OFFSET>,
            SubscriberInterface: SubscriberInterface::<Identity, OFFSET>,
            SetSubscriberInterface: SetSubscriberInterface::<Identity, OFFSET>,
            PerUser: PerUser::<Identity, OFFSET>,
            SetPerUser: SetPerUser::<Identity, OFFSET>,
            OwnerSID: OwnerSID::<Identity, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            MachineName: MachineName::<Identity, OFFSET>,
            SetMachineName: SetMachineName::<Identity, OFFSET>,
            GetPublisherProperty: GetPublisherProperty::<Identity, OFFSET>,
            PutPublisherProperty: PutPublisherProperty::<Identity, OFFSET>,
            RemovePublisherProperty: RemovePublisherProperty::<Identity, OFFSET>,
            GetPublisherPropertyCollection: GetPublisherPropertyCollection::<Identity, OFFSET>,
            GetSubscriberProperty: GetSubscriberProperty::<Identity, OFFSET>,
            PutSubscriberProperty: PutSubscriberProperty::<Identity, OFFSET>,
            RemoveSubscriberProperty: RemoveSubscriberProperty::<Identity, OFFSET>,
            GetSubscriberPropertyCollection: GetSubscriberPropertyCollection::<Identity, OFFSET>,
            InterfaceID: InterfaceID::<Identity, OFFSET>,
            SetInterfaceID: SetInterfaceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventSubscription as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEventSystem_Impl: Sized + super::IDispatch_Impl {
    fn Query(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR, errorindex: *mut i32) -> windows_core::Result<windows_core::IUnknown>;
    fn Store(&self, progid: &windows_core::BSTR, pinterface: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Remove(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn EventObjectChangeEventClassID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn QueryS(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn RemoveS(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventSystem {}
impl IEventSystem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEventSystem_Vtbl
    where
        Identity: IEventSystem_Impl,
    {
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::Query(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria), core::mem::transmute_copy(&errorindex)) {
                Ok(ok__) => {
                    ppinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Store<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, pinterface: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSystem_Impl::Store(this, core::mem::transmute(&progid), windows_core::from_raw_borrowed(&pinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT
        where
            Identity: IEventSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::Remove(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)) {
                Ok(ok__) => {
                    errorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::EventObjectChangeEventClassID(this) {
                Ok(ok__) => {
                    pbstreventclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEventSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::QueryS(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)) {
                Ok(ok__) => {
                    ppinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IEventSystem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSystem_Impl::RemoveS(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Query: Query::<Identity, OFFSET>,
            Store: Store::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Identity, OFFSET>,
            QueryS: QueryS::<Identity, OFFSET>,
            RemoveS: RemoveS::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventSystem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IFiringControl_Impl: Sized + super::IDispatch_Impl {
    fn FireSubscription(&self, subscription: Option<&IEventSubscription>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IFiringControl {}
impl IFiringControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFiringControl_Vtbl
    where
        Identity: IFiringControl_Impl,
    {
        unsafe extern "system" fn FireSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscription: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFiringControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFiringControl_Impl::FireSubscription(this, windows_core::from_raw_borrowed(&subscription)).into()
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), FireSubscription: FireSubscription::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFiringControl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IMultiInterfaceEventControl_Impl: Sized {
    fn SetMultiInterfacePublisherFilter(&self, classfilter: Option<&IMultiInterfacePublisherFilter>) -> windows_core::Result<()>;
    fn GetSubscriptions(&self, eventiid: *const windows_core::GUID, bstrmethodname: &windows_core::BSTR, optionalcriteria: &windows_core::BSTR, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, eventiid: *const windows_core::GUID, bstrmethodname: &windows_core::BSTR, bstrcriteria: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMultiInterfaceEventControl {}
impl IMultiInterfaceEventControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultiInterfaceEventControl_Vtbl
    where
        Identity: IMultiInterfaceEventControl_Impl,
    {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, classfilter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfaceEventControl_Impl::SetMultiInterfacePublisherFilter(this, windows_core::from_raw_borrowed(&classfilter)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventiid: *const windows_core::GUID, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>, optionalcriteria: core::mem::MaybeUninit<windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::GetSubscriptions(this, core::mem::transmute_copy(&eventiid), core::mem::transmute(&bstrmethodname), core::mem::transmute(&optionalcriteria), core::mem::transmute_copy(&optionalerrorindex)) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventiid: *const windows_core::GUID, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>, bstrcriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::SetDefaultQuery(this, core::mem::transmute_copy(&eventiid), core::mem::transmute(&bstrmethodname), core::mem::transmute(&bstrcriteria)) {
                Ok(ok__) => {
                    errorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    pfallowinprocactivation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfaceEventControl_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::FireInParallel(this) {
                Ok(ok__) => {
                    pffireinparallel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IMultiInterfaceEventControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfaceEventControl_Impl::SetFireInParallel(this, core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMultiInterfacePublisherFilter: SetMultiInterfacePublisherFilter::<Identity, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, OFFSET>,
            FireInParallel: FireInParallel::<Identity, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiInterfaceEventControl as windows_core::Interface>::IID
    }
}
pub trait IMultiInterfacePublisherFilter_Impl: Sized {
    fn Initialize(&self, peic: Option<&IMultiInterfaceEventControl>) -> windows_core::Result<()>;
    fn PrepareToFire(&self, iid: *const windows_core::GUID, methodname: &windows_core::BSTR, firingcontrol: Option<&IFiringControl>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMultiInterfacePublisherFilter {}
impl IMultiInterfacePublisherFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultiInterfacePublisherFilter_Vtbl
    where
        Identity: IMultiInterfacePublisherFilter_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peic: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiInterfacePublisherFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfacePublisherFilter_Impl::Initialize(this, windows_core::from_raw_borrowed(&peic)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, methodname: core::mem::MaybeUninit<windows_core::BSTR>, firingcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultiInterfacePublisherFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfacePublisherFilter_Impl::PrepareToFire(this, core::mem::transmute_copy(&iid), core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiInterfacePublisherFilter as windows_core::Interface>::IID
    }
}
pub trait IPublisherFilter_Impl: Sized {
    fn Initialize(&self, methodname: &windows_core::BSTR, dispuserdefined: Option<&super::IDispatch>) -> windows_core::Result<()>;
    fn PrepareToFire(&self, methodname: &windows_core::BSTR, firingcontrol: Option<&IFiringControl>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPublisherFilter {}
impl IPublisherFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPublisherFilter_Vtbl
    where
        Identity: IPublisherFilter_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, dispuserdefined: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPublisherFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPublisherFilter_Impl::Initialize(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&dispuserdefined)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, firingcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPublisherFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPublisherFilter_Impl::PrepareToFire(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPublisherFilter as windows_core::Interface>::IID
    }
}
