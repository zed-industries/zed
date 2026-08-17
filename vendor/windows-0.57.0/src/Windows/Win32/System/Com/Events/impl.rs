pub trait IDontSupportEventSubscription_Impl: Sized {}
impl windows_core::RuntimeName for IDontSupportEventSubscription {}
impl IDontSupportEventSubscription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDontSupportEventSubscription_Impl, const OFFSET: isize>() -> IDontSupportEventSubscription_Vtbl {
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>() -> IEnumEventObject_Vtbl {
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEnumEventObject_Impl::Clone(this) {
                Ok(ok__) => {
                    core::ptr::write(ppinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut core::ffi::c_void, cretelem: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumEventObject_Impl::Next(this, core::mem::transmute_copy(&creqelem), core::mem::transmute_copy(&ppinterface), core::mem::transmute_copy(&cretelem)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumEventObject_Impl::Reset(this)
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskipelem: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEnumEventObject_Impl::Skip(this, core::mem::transmute_copy(&cskipelem)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>() -> IEventClass_Vtbl {
        unsafe extern "system" fn EventClassID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::EventClassID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstreventclassid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetEventClassID(this, core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn EventClassName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::EventClassName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstreventclassname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetEventClassName(this, core::mem::transmute(&bstreventclassname)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrownersid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn FiringInterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfiringinterfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::FiringInterfaceID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrfiringinterfaceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiringInterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfiringinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetFiringInterfaceID(this, core::mem::transmute(&bstrfiringinterfaceid)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn CustomConfigCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcustomconfigclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::CustomConfigCLSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrcustomconfigclsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcustomconfigclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetCustomConfigCLSID(this, core::mem::transmute(&bstrcustomconfigclsid)).into()
        }
        unsafe extern "system" fn TypeLib<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtypelib: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass_Impl::TypeLib(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrtypelib, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeLib<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypelib: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass_Impl::SetTypeLib(this, core::mem::transmute(&bstrtypelib)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventClassID: EventClassID::<Identity, Impl, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, Impl, OFFSET>,
            EventClassName: EventClassName::<Identity, Impl, OFFSET>,
            SetEventClassName: SetEventClassName::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            FiringInterfaceID: FiringInterfaceID::<Identity, Impl, OFFSET>,
            SetFiringInterfaceID: SetFiringInterfaceID::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            CustomConfigCLSID: CustomConfigCLSID::<Identity, Impl, OFFSET>,
            SetCustomConfigCLSID: SetCustomConfigCLSID::<Identity, Impl, OFFSET>,
            TypeLib: TypeLib::<Identity, Impl, OFFSET>,
            SetTypeLib: SetTypeLib::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>() -> IEventClass2_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass2_Impl::PublisherID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpublisherid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass2_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpubfilclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass2_Impl::MultiInterfacePublisherFilterCLSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpubfilclsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpubfilclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass2_Impl::SetMultiInterfacePublisherFilterCLSID(this, core::mem::transmute(&bstrpubfilclsid)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass2_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    core::ptr::write(pfallowinprocactivation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass2_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventClass2_Impl::FireInParallel(this) {
                Ok(ok__) => {
                    core::ptr::write(pffireinparallel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventClass2_Impl::SetFireInParallel(this, core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: IEventClass_Vtbl::new::<Identity, Impl, OFFSET>(),
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            MultiInterfacePublisherFilterCLSID: MultiInterfacePublisherFilterCLSID::<Identity, Impl, OFFSET>,
            SetMultiInterfacePublisherFilterCLSID: SetMultiInterfacePublisherFilterCLSID::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            FireInParallel: FireInParallel::<Identity, Impl, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>() -> IEventControl_Vtbl {
        unsafe extern "system" fn SetPublisherFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, ppublisherfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventControl_Impl::SetPublisherFilter(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&ppublisherfilter)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventControl_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    core::ptr::write(pfallowinprocactivation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventControl_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, optionalcriteria: core::mem::MaybeUninit<windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventControl_Impl::GetSubscriptions(this, core::mem::transmute(&methodname), core::mem::transmute(&optionalcriteria), core::mem::transmute_copy(&optionalerrorindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, criteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventControl_Impl::SetDefaultQuery(this, core::mem::transmute(&methodname), core::mem::transmute(&criteria)) {
                Ok(ok__) => {
                    core::ptr::write(errorindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPublisherFilter: SetPublisherFilter::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>() -> IEventObjectChange_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectChange_Impl::ChangedSubscription(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectChange_Impl::ChangedEventClass(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn ChangedPublisher<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectChange_Impl::ChangedPublisher(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstrpublisherid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
            ChangedPublisher: ChangedPublisher::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: isize>() -> IEventObjectChange2_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectChange2_Impl::ChangedSubscription(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectChange2_Impl::ChangedEventClass(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>() -> IEventObjectCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventObjectCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppunkenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventObjectCollection_Impl::get_Item(this, core::mem::transmute(&objectid)) {
                Ok(ok__) => {
                    core::ptr::write(pitem, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEnum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventObjectCollection_Impl::NewEnum(this) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventObjectCollection_Impl::Count(this) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *const core::mem::MaybeUninit<windows_core::VARIANT>, objectid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectCollection_Impl::Add(this, core::mem::transmute_copy(&item), core::mem::transmute(&objectid)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventObjectCollection_Impl::Remove(this, core::mem::transmute(&objectid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            NewEnum: NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>() -> IEventProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventProperty_Impl::Name(this) {
                Ok(ok__) => {
                    core::ptr::write(propertyname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventProperty_Impl::SetName(this, core::mem::transmute(&propertyname)).into()
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventProperty_Impl::Value(this) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventProperty_Impl::SetValue(this, core::mem::transmute_copy(&propertyvalue)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>() -> IEventPublisher_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::PublisherID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpublisherid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn PublisherName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublishername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::PublisherName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpublishername, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublishername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::SetPublisherName(this, core::mem::transmute(&bstrpublishername)).into()
        }
        unsafe extern "system" fn PublisherType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublishertype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::PublisherType(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpublishertype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublishertype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::SetPublisherType(this, core::mem::transmute(&bstrpublishertype)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrownersid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn GetDefaultProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::GetDefaultProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutDefaultProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::PutDefaultProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventPublisher_Impl::RemoveDefaultProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventPublisher_Impl::GetDefaultPropertyCollection(this) {
                Ok(ok__) => {
                    core::ptr::write(collection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            PublisherName: PublisherName::<Identity, Impl, OFFSET>,
            SetPublisherName: SetPublisherName::<Identity, Impl, OFFSET>,
            PublisherType: PublisherType::<Identity, Impl, OFFSET>,
            SetPublisherType: SetPublisherType::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDefaultProperty: GetDefaultProperty::<Identity, Impl, OFFSET>,
            PutDefaultProperty: PutDefaultProperty::<Identity, Impl, OFFSET>,
            RemoveDefaultProperty: RemoveDefaultProperty::<Identity, Impl, OFFSET>,
            GetDefaultPropertyCollection: GetDefaultPropertyCollection::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>() -> IEventSubscription_Vtbl {
        unsafe extern "system" fn SubscriptionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriptionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::SubscriptionID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubscriptionid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriptionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetSubscriptionID(this, core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn SubscriptionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriptionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::SubscriptionName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubscriptionname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriptionname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetSubscriptionName(this, core::mem::transmute(&bstrsubscriptionname)).into()
        }
        unsafe extern "system" fn PublisherID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::PublisherID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrpublisherid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn EventClassID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::EventClassID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstreventclassid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetEventClassID(this, core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn MethodName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmethodname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::MethodName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrmethodname, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetMethodName(this, core::mem::transmute(&bstrmethodname)).into()
        }
        unsafe extern "system" fn SubscriberCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriberclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::SubscriberCLSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrsubscriberclsid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberCLSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriberclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetSubscriberCLSID(this, core::mem::transmute(&bstrsubscriberclsid)).into()
        }
        unsafe extern "system" fn SubscriberInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubscriberinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::SubscriberInterface(this) {
                Ok(ok__) => {
                    core::ptr::write(ppsubscriberinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubscriberinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetSubscriberInterface(this, windows_core::from_raw_borrowed(&psubscriberinterface)).into()
        }
        unsafe extern "system" fn PerUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::PerUser(this) {
                Ok(ok__) => {
                    core::ptr::write(pfperuser, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerUser<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetPerUser(this, core::mem::transmute_copy(&fperuser)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrownersid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Enabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::Enabled(this) {
                Ok(ok__) => {
                    core::ptr::write(pfenabled, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrdescription, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MachineName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachinename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::MachineName(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrmachinename, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmachinename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetMachineName(this, core::mem::transmute(&bstrmachinename)).into()
        }
        unsafe extern "system" fn GetPublisherProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::GetPublisherProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPublisherProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::PutPublisherProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemovePublisherProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::RemovePublisherProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::GetPublisherPropertyCollection(this) {
                Ok(ok__) => {
                    core::ptr::write(collection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::GetSubscriberProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    core::ptr::write(propertyvalue, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutSubscriberProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::PutSubscriberProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::RemoveSubscriberProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::GetSubscriberPropertyCollection(this) {
                Ok(ok__) => {
                    core::ptr::write(collection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinterfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSubscription_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstrinterfaceid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSubscription_Impl::SetInterfaceID(this, core::mem::transmute(&bstrinterfaceid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubscriptionID: SubscriptionID::<Identity, Impl, OFFSET>,
            SetSubscriptionID: SetSubscriptionID::<Identity, Impl, OFFSET>,
            SubscriptionName: SubscriptionName::<Identity, Impl, OFFSET>,
            SetSubscriptionName: SetSubscriptionName::<Identity, Impl, OFFSET>,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            EventClassID: EventClassID::<Identity, Impl, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, Impl, OFFSET>,
            MethodName: MethodName::<Identity, Impl, OFFSET>,
            SetMethodName: SetMethodName::<Identity, Impl, OFFSET>,
            SubscriberCLSID: SubscriberCLSID::<Identity, Impl, OFFSET>,
            SetSubscriberCLSID: SetSubscriberCLSID::<Identity, Impl, OFFSET>,
            SubscriberInterface: SubscriberInterface::<Identity, Impl, OFFSET>,
            SetSubscriberInterface: SetSubscriberInterface::<Identity, Impl, OFFSET>,
            PerUser: PerUser::<Identity, Impl, OFFSET>,
            SetPerUser: SetPerUser::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MachineName: MachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
            GetPublisherProperty: GetPublisherProperty::<Identity, Impl, OFFSET>,
            PutPublisherProperty: PutPublisherProperty::<Identity, Impl, OFFSET>,
            RemovePublisherProperty: RemovePublisherProperty::<Identity, Impl, OFFSET>,
            GetPublisherPropertyCollection: GetPublisherPropertyCollection::<Identity, Impl, OFFSET>,
            GetSubscriberProperty: GetSubscriberProperty::<Identity, Impl, OFFSET>,
            PutSubscriberProperty: PutSubscriberProperty::<Identity, Impl, OFFSET>,
            RemoveSubscriberProperty: RemoveSubscriberProperty::<Identity, Impl, OFFSET>,
            GetSubscriberPropertyCollection: GetSubscriberPropertyCollection::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            SetInterfaceID: SetInterfaceID::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>() -> IEventSystem_Vtbl {
        unsafe extern "system" fn Query<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSystem_Impl::Query(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria), core::mem::transmute_copy(&errorindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Store<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, pinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSystem_Impl::Store(this, core::mem::transmute(&progid), windows_core::from_raw_borrowed(&pinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSystem_Impl::Remove(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)) {
                Ok(ok__) => {
                    core::ptr::write(errorindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSystem_Impl::EventObjectChangeEventClassID(this) {
                Ok(ok__) => {
                    core::ptr::write(pbstreventclassid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryS<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IEventSystem_Impl::QueryS(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)) {
                Ok(ok__) => {
                    core::ptr::write(ppinterface, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveS<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IEventSystem_Impl::RemoveS(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Query: Query::<Identity, Impl, OFFSET>,
            Store: Store::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Identity, Impl, OFFSET>,
            QueryS: QueryS::<Identity, Impl, OFFSET>,
            RemoveS: RemoveS::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFiringControl_Impl, const OFFSET: isize>() -> IFiringControl_Vtbl {
        unsafe extern "system" fn FireSubscription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IFiringControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IFiringControl_Impl::FireSubscription(this, windows_core::from_raw_borrowed(&subscription)).into()
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), FireSubscription: FireSubscription::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>() -> IMultiInterfaceEventControl_Vtbl {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMultiInterfaceEventControl_Impl::SetMultiInterfacePublisherFilter(this, windows_core::from_raw_borrowed(&classfilter)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventiid: *const windows_core::GUID, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>, optionalcriteria: core::mem::MaybeUninit<windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMultiInterfaceEventControl_Impl::GetSubscriptions(this, core::mem::transmute_copy(&eventiid), core::mem::transmute(&bstrmethodname), core::mem::transmute(&optionalcriteria), core::mem::transmute_copy(&optionalerrorindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppcollection, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventiid: *const windows_core::GUID, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>, bstrcriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMultiInterfaceEventControl_Impl::SetDefaultQuery(this, core::mem::transmute_copy(&eventiid), core::mem::transmute(&bstrmethodname), core::mem::transmute(&bstrcriteria)) {
                Ok(ok__) => {
                    core::ptr::write(errorindex, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMultiInterfaceEventControl_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    core::ptr::write(pfallowinprocactivation, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMultiInterfaceEventControl_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMultiInterfaceEventControl_Impl::FireInParallel(this) {
                Ok(ok__) => {
                    core::ptr::write(pffireinparallel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMultiInterfaceEventControl_Impl::SetFireInParallel(this, core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMultiInterfacePublisherFilter: SetMultiInterfacePublisherFilter::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            FireInParallel: FireInParallel::<Identity, Impl, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>() -> IMultiInterfacePublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMultiInterfacePublisherFilter_Impl::Initialize(this, windows_core::from_raw_borrowed(&peic)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, methodname: core::mem::MaybeUninit<windows_core::BSTR>, firingcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMultiInterfacePublisherFilter_Impl::PrepareToFire(this, core::mem::transmute_copy(&iid), core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: isize>() -> IPublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, dispuserdefined: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPublisherFilter_Impl::Initialize(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&dispuserdefined)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, firingcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPublisherFilter_Impl::PrepareToFire(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPublisherFilter as windows_core::Interface>::IID
    }
}
