pub trait IKsAggregateControl_Impl: Sized {
    fn KsAddAggregate(&self, aggregateclass: *const windows_core::GUID) -> windows_core::Result<()>;
    fn KsRemoveAggregate(&self, aggregateclass: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsAggregateControl {}
impl IKsAggregateControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>() -> IKsAggregateControl_Vtbl {
        unsafe extern "system" fn KsAddAggregate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aggregateclass: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAggregateControl_Impl::KsAddAggregate(this, core::mem::transmute_copy(&aggregateclass)).into()
        }
        unsafe extern "system" fn KsRemoveAggregate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aggregateclass: *const windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAggregateControl_Impl::KsRemoveAggregate(this, core::mem::transmute_copy(&aggregateclass)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsAddAggregate: KsAddAggregate::<Identity, Impl, OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsAggregateControl as windows_core::Interface>::IID
    }
}
pub trait IKsAllocator_Impl: Sized {
    fn KsGetAllocatorHandle(&self) -> super::super::Foundation::HANDLE;
    fn KsGetAllocatorMode(&self) -> KSALLOCATORMODE;
    fn KsGetAllocatorStatus(&self, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> windows_core::Result<()>;
    fn KsSetAllocatorMode(&self, mode: KSALLOCATORMODE);
}
impl windows_core::RuntimeName for IKsAllocator {}
impl IKsAllocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: isize>() -> IKsAllocator_Vtbl {
        unsafe extern "system" fn KsGetAllocatorHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocator_Impl::KsGetAllocatorHandle(this)
        }
        unsafe extern "system" fn KsGetAllocatorMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> KSALLOCATORMODE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocator_Impl::KsGetAllocatorMode(this)
        }
        unsafe extern "system" fn KsGetAllocatorStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocatorstatus: *mut KSSTREAMALLOCATOR_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocator_Impl::KsGetAllocatorStatus(this, core::mem::transmute_copy(&allocatorstatus)).into()
        }
        unsafe extern "system" fn KsSetAllocatorMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: KSALLOCATORMODE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocator_Impl::KsSetAllocatorMode(this, core::mem::transmute_copy(&mode))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsGetAllocatorHandle: KsGetAllocatorHandle::<Identity, Impl, OFFSET>,
            KsGetAllocatorMode: KsGetAllocatorMode::<Identity, Impl, OFFSET>,
            KsGetAllocatorStatus: KsGetAllocatorStatus::<Identity, Impl, OFFSET>,
            KsSetAllocatorMode: KsSetAllocatorMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsAllocator as windows_core::Interface>::IID
    }
}
pub trait IKsAllocatorEx_Impl: Sized + IKsAllocator_Impl {
    fn KsGetProperties(&self) -> *mut ALLOCATOR_PROPERTIES_EX;
    fn KsSetProperties(&self, param0: *const ALLOCATOR_PROPERTIES_EX);
    fn KsSetAllocatorHandle(&self, allocatorhandle: super::super::Foundation::HANDLE);
    fn KsCreateAllocatorAndGetHandle(&self, kspin: Option<&IKsPin>) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for IKsAllocatorEx {}
impl IKsAllocatorEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: isize>() -> IKsAllocatorEx_Vtbl {
        unsafe extern "system" fn KsGetProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut ALLOCATOR_PROPERTIES_EX {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocatorEx_Impl::KsGetProperties(this)
        }
        unsafe extern "system" fn KsSetProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const ALLOCATOR_PROPERTIES_EX) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocatorEx_Impl::KsSetProperties(this, core::mem::transmute_copy(&param0))
        }
        unsafe extern "system" fn KsSetAllocatorHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocatorhandle: super::super::Foundation::HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocatorEx_Impl::KsSetAllocatorHandle(this, core::mem::transmute_copy(&allocatorhandle))
        }
        unsafe extern "system" fn KsCreateAllocatorAndGetHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kspin: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsAllocatorEx_Impl::KsCreateAllocatorAndGetHandle(this, windows_core::from_raw_borrowed(&kspin))
        }
        Self {
            base__: IKsAllocator_Vtbl::new::<Identity, Impl, OFFSET>(),
            KsGetProperties: KsGetProperties::<Identity, Impl, OFFSET>,
            KsSetProperties: KsSetProperties::<Identity, Impl, OFFSET>,
            KsSetAllocatorHandle: KsSetAllocatorHandle::<Identity, Impl, OFFSET>,
            KsCreateAllocatorAndGetHandle: KsCreateAllocatorAndGetHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsAllocatorEx as windows_core::Interface>::IID || iid == &<IKsAllocator as windows_core::Interface>::IID
    }
}
pub trait IKsClockPropertySet_Impl: Sized {
    fn KsGetTime(&self) -> windows_core::Result<i64>;
    fn KsSetTime(&self, time: i64) -> windows_core::Result<()>;
    fn KsGetPhysicalTime(&self) -> windows_core::Result<i64>;
    fn KsSetPhysicalTime(&self, time: i64) -> windows_core::Result<()>;
    fn KsGetCorrelatedTime(&self) -> windows_core::Result<KSCORRELATED_TIME>;
    fn KsSetCorrelatedTime(&self, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::Result<()>;
    fn KsGetCorrelatedPhysicalTime(&self) -> windows_core::Result<KSCORRELATED_TIME>;
    fn KsSetCorrelatedPhysicalTime(&self, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::Result<()>;
    fn KsGetResolution(&self) -> windows_core::Result<KSRESOLUTION>;
    fn KsGetState(&self) -> windows_core::Result<KSSTATE>;
}
impl windows_core::RuntimeName for IKsClockPropertySet {}
impl IKsClockPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>() -> IKsClockPropertySet_Vtbl {
        unsafe extern "system" fn KsGetTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsClockPropertySet_Impl::KsGetTime(this) {
                Ok(ok__) => {
                    core::ptr::write(time, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsClockPropertySet_Impl::KsSetTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn KsGetPhysicalTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsClockPropertySet_Impl::KsGetPhysicalTime(this) {
                Ok(ok__) => {
                    core::ptr::write(time, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetPhysicalTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsClockPropertySet_Impl::KsSetPhysicalTime(this, core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn KsGetCorrelatedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *mut KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsClockPropertySet_Impl::KsGetCorrelatedTime(this) {
                Ok(ok__) => {
                    core::ptr::write(correlatedtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetCorrelatedTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsClockPropertySet_Impl::KsSetCorrelatedTime(this, core::mem::transmute_copy(&correlatedtime)).into()
        }
        unsafe extern "system" fn KsGetCorrelatedPhysicalTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *mut KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsClockPropertySet_Impl::KsGetCorrelatedPhysicalTime(this) {
                Ok(ok__) => {
                    core::ptr::write(correlatedtime, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetCorrelatedPhysicalTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, correlatedtime: *const KSCORRELATED_TIME) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsClockPropertySet_Impl::KsSetCorrelatedPhysicalTime(this, core::mem::transmute_copy(&correlatedtime)).into()
        }
        unsafe extern "system" fn KsGetResolution<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resolution: *mut KSRESOLUTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsClockPropertySet_Impl::KsGetResolution(this) {
                Ok(ok__) => {
                    core::ptr::write(resolution, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsGetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsClockPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut KSSTATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsClockPropertySet_Impl::KsGetState(this) {
                Ok(ok__) => {
                    core::ptr::write(state, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsGetTime: KsGetTime::<Identity, Impl, OFFSET>,
            KsSetTime: KsSetTime::<Identity, Impl, OFFSET>,
            KsGetPhysicalTime: KsGetPhysicalTime::<Identity, Impl, OFFSET>,
            KsSetPhysicalTime: KsSetPhysicalTime::<Identity, Impl, OFFSET>,
            KsGetCorrelatedTime: KsGetCorrelatedTime::<Identity, Impl, OFFSET>,
            KsSetCorrelatedTime: KsSetCorrelatedTime::<Identity, Impl, OFFSET>,
            KsGetCorrelatedPhysicalTime: KsGetCorrelatedPhysicalTime::<Identity, Impl, OFFSET>,
            KsSetCorrelatedPhysicalTime: KsSetCorrelatedPhysicalTime::<Identity, Impl, OFFSET>,
            KsGetResolution: KsGetResolution::<Identity, Impl, OFFSET>,
            KsGetState: KsGetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsClockPropertySet as windows_core::Interface>::IID
    }
}
pub trait IKsControl_Impl: Sized {
    fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsControl {}
impl IKsControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>() -> IKsControl_Vtbl {
        unsafe extern "system" fn KsProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsControl_Impl::KsProperty(this, core::mem::transmute_copy(&property), core::mem::transmute_copy(&propertylength), core::mem::transmute_copy(&propertydata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsMethod<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsControl_Impl::KsMethod(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&methodlength), core::mem::transmute_copy(&methoddata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsControl_Impl::KsEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&eventlength), core::mem::transmute_copy(&eventdata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsProperty: KsProperty::<Identity, Impl, OFFSET>,
            KsMethod: KsMethod::<Identity, Impl, OFFSET>,
            KsEvent: KsEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IKsDataTypeCompletion_Impl: Sized {
    fn KsCompleteMediaType(&self, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::RuntimeName for IKsDataTypeCompletion {}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IKsDataTypeCompletion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeCompletion_Impl, const OFFSET: isize>() -> IKsDataTypeCompletion_Vtbl {
        unsafe extern "system" fn KsCompleteMediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeCompletion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterhandle: super::super::Foundation::HANDLE, pinfactoryid: u32, ammediatype: *mut super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsDataTypeCompletion_Impl::KsCompleteMediaType(this, core::mem::transmute_copy(&filterhandle), core::mem::transmute_copy(&pinfactoryid), core::mem::transmute_copy(&ammediatype)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsCompleteMediaType: KsCompleteMediaType::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsDataTypeCompletion as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
pub trait IKsDataTypeHandler_Impl: Sized {
    fn KsCompleteIoOperation(&self, sample: Option<&super::DirectShow::IMediaSample>, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn KsIsMediaTypeInRanges(&self, dataranges: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn KsPrepareIoOperation(&self, sample: Option<&super::DirectShow::IMediaSample>, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION) -> windows_core::Result<()>;
    fn KsQueryExtendedSize(&self) -> windows_core::Result<u32>;
    fn KsSetMediaType(&self, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
impl windows_core::RuntimeName for IKsDataTypeHandler {}
#[cfg(all(feature = "Win32_Media_DirectShow", feature = "Win32_Media_MediaFoundation"))]
impl IKsDataTypeHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: isize>() -> IKsDataTypeHandler_Vtbl {
        unsafe extern "system" fn KsCompleteIoOperation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION, cancelled: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsDataTypeHandler_Impl::KsCompleteIoOperation(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&streamheader), core::mem::transmute_copy(&iooperation), core::mem::transmute_copy(&cancelled)).into()
        }
        unsafe extern "system" fn KsIsMediaTypeInRanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataranges: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsDataTypeHandler_Impl::KsIsMediaTypeInRanges(this, core::mem::transmute_copy(&dataranges)).into()
        }
        unsafe extern "system" fn KsPrepareIoOperation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, streamheader: *mut core::ffi::c_void, iooperation: KSIOOPERATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsDataTypeHandler_Impl::KsPrepareIoOperation(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&streamheader), core::mem::transmute_copy(&iooperation)).into()
        }
        unsafe extern "system" fn KsQueryExtendedSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extendedsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsDataTypeHandler_Impl::KsQueryExtendedSize(this) {
                Ok(ok__) => {
                    core::ptr::write(extendedsize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsSetMediaType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsDataTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ammediatype: *const super::MediaFoundation::AM_MEDIA_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsDataTypeHandler_Impl::KsSetMediaType(this, core::mem::transmute_copy(&ammediatype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsCompleteIoOperation: KsCompleteIoOperation::<Identity, Impl, OFFSET>,
            KsIsMediaTypeInRanges: KsIsMediaTypeInRanges::<Identity, Impl, OFFSET>,
            KsPrepareIoOperation: KsPrepareIoOperation::<Identity, Impl, OFFSET>,
            KsQueryExtendedSize: KsQueryExtendedSize::<Identity, Impl, OFFSET>,
            KsSetMediaType: KsSetMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsDataTypeHandler as windows_core::Interface>::IID
    }
}
pub trait IKsFormatSupport_Impl: Sized {
    fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDevicePreferredFormat(&self) -> windows_core::Result<*mut KSDATAFORMAT>;
}
impl windows_core::RuntimeName for IKsFormatSupport {}
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>() -> IKsFormatSupport_Vtbl {
        unsafe extern "system" fn IsFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsFormatSupport_Impl::IsFormatSupported(this, core::mem::transmute_copy(&pksformat), core::mem::transmute_copy(&cbformat), core::mem::transmute_copy(&pbsupported)).into()
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsFormatSupport_Impl::GetDevicePreferredFormat(this) {
                Ok(ok__) => {
                    core::ptr::write(ppksformat, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsFormatSupport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsInterfaceHandler_Impl: Sized {
    fn KsSetPin(&self, kspin: Option<&IKsPin>) -> windows_core::Result<()>;
    fn KsProcessMediaSamples(&self, ksdatatypehandler: Option<&IKsDataTypeHandler>, samplelist: *const Option<super::DirectShow::IMediaSample>, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> windows_core::Result<()>;
    fn KsCompleteIo(&self, streamsegment: *mut KSSTREAM_SEGMENT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsInterfaceHandler {}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsInterfaceHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: isize>() -> IKsInterfaceHandler_Vtbl {
        unsafe extern "system" fn KsSetPin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kspin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsInterfaceHandler_Impl::KsSetPin(this, windows_core::from_raw_borrowed(&kspin)).into()
        }
        unsafe extern "system" fn KsProcessMediaSamples<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ksdatatypehandler: *mut core::ffi::c_void, samplelist: *const *mut core::ffi::c_void, samplecount: *mut i32, iooperation: KSIOOPERATION, streamsegment: *mut *mut KSSTREAM_SEGMENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsInterfaceHandler_Impl::KsProcessMediaSamples(this, windows_core::from_raw_borrowed(&ksdatatypehandler), core::mem::transmute_copy(&samplelist), core::mem::transmute_copy(&samplecount), core::mem::transmute_copy(&iooperation), core::mem::transmute_copy(&streamsegment)).into()
        }
        unsafe extern "system" fn KsCompleteIo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsInterfaceHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamsegment: *mut KSSTREAM_SEGMENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsInterfaceHandler_Impl::KsCompleteIo(this, core::mem::transmute_copy(&streamsegment)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsSetPin: KsSetPin::<Identity, Impl, OFFSET>,
            KsProcessMediaSamples: KsProcessMediaSamples::<Identity, Impl, OFFSET>,
            KsCompleteIo: KsCompleteIo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsInterfaceHandler as windows_core::Interface>::IID
    }
}
pub trait IKsJackContainerId_Impl: Sized {
    fn GetJackContainerId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IKsJackContainerId {}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: isize>() -> IKsJackContainerId_Vtbl {
        unsafe extern "system" fn GetJackContainerId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjackcontainerid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsJackContainerId_Impl::GetJackContainerId(this) {
                Ok(ok__) => {
                    core::ptr::write(pjackcontainerid, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackContainerId: GetJackContainerId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackContainerId as windows_core::Interface>::IID
    }
}
pub trait IKsJackDescription_Impl: Sized {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription(&self, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsJackDescription {}
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>() -> IKsJackDescription_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsJackDescription_Impl::GetJackCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcjacks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsJackDescription_Impl::GetJackDescription(this, core::mem::transmute_copy(&njack), core::mem::transmute_copy(&pdescription)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription as windows_core::Interface>::IID
    }
}
pub trait IKsJackDescription2_Impl: Sized {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription2(&self, njack: u32) -> windows_core::Result<KSJACK_DESCRIPTION2>;
}
impl windows_core::RuntimeName for IKsJackDescription2 {}
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>() -> IKsJackDescription2_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsJackDescription2_Impl::GetJackCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcjacks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsJackDescription2_Impl::GetJackDescription2(this, core::mem::transmute_copy(&njack)) {
                Ok(ok__) => {
                    core::ptr::write(pdescription2, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription2: GetJackDescription2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription2 as windows_core::Interface>::IID
    }
}
pub trait IKsJackDescription3_Impl: Sized {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription3(&self, njack: u32) -> windows_core::Result<KSJACK_DESCRIPTION3>;
}
impl windows_core::RuntimeName for IKsJackDescription3 {}
impl IKsJackDescription3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription3_Impl, const OFFSET: isize>() -> IKsJackDescription3_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsJackDescription3_Impl::GetJackCount(this) {
                Ok(ok__) => {
                    core::ptr::write(pcjacks, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription3: *mut KSJACK_DESCRIPTION3) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsJackDescription3_Impl::GetJackDescription3(this, core::mem::transmute_copy(&njack)) {
                Ok(ok__) => {
                    core::ptr::write(pdescription3, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription3: GetJackDescription3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription3 as windows_core::Interface>::IID
    }
}
pub trait IKsJackSinkInformation_Impl: Sized {
    fn GetJackSinkInformation(&self, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsJackSinkInformation {}
impl IKsJackSinkInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>() -> IKsJackSinkInformation_Vtbl {
        unsafe extern "system" fn GetJackSinkInformation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsJackSinkInformation_Impl::GetJackSinkInformation(this, core::mem::transmute_copy(&pjacksinkinformation)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as windows_core::Interface>::IID
    }
}
pub trait IKsNodeControl_Impl: Sized {
    fn SetNodeId(&self, dwnodeid: u32) -> windows_core::Result<()>;
    fn SetKsControl(&self, pkscontrol: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsNodeControl {}
impl IKsNodeControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsNodeControl_Impl, const OFFSET: isize>() -> IKsNodeControl_Vtbl {
        unsafe extern "system" fn SetNodeId<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsNodeControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsNodeControl_Impl::SetNodeId(this, core::mem::transmute_copy(&dwnodeid)).into()
        }
        unsafe extern "system" fn SetKsControl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsNodeControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkscontrol: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsNodeControl_Impl::SetKsControl(this, core::mem::transmute_copy(&pkscontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNodeId: SetNodeId::<Identity, Impl, OFFSET>,
            SetKsControl: SetKsControl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsNodeControl as windows_core::Interface>::IID
    }
}
pub trait IKsNotifyEvent_Impl: Sized {
    fn KsNotifyEvent(&self, event: u32, lparam1: usize, lparam2: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsNotifyEvent {}
impl IKsNotifyEvent_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsNotifyEvent_Impl, const OFFSET: isize>() -> IKsNotifyEvent_Vtbl {
        unsafe extern "system" fn KsNotifyEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsNotifyEvent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: u32, lparam1: usize, lparam2: usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsNotifyEvent_Impl::KsNotifyEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&lparam1), core::mem::transmute_copy(&lparam2)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsNotifyEvent: KsNotifyEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsNotifyEvent as windows_core::Interface>::IID
    }
}
pub trait IKsObject_Impl: Sized {
    fn KsGetObjectHandle(&self) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for IKsObject {}
impl IKsObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsObject_Impl, const OFFSET: isize>() -> IKsObject_Vtbl {
        unsafe extern "system" fn KsGetObjectHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsObject_Impl::KsGetObjectHandle(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsGetObjectHandle: KsGetObjectHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsObject as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPin_Impl: Sized {
    fn KsQueryMediums(&self) -> windows_core::Result<*mut KSMULTIPLE_ITEM>;
    fn KsQueryInterfaces(&self) -> windows_core::Result<*mut KSMULTIPLE_ITEM>;
    fn KsCreateSinkPinHandle(&self, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> windows_core::Result<()>;
    fn KsGetCurrentCommunication(&self, communication: *mut KSPIN_COMMUNICATION, interface: *mut KSIDENTIFIER, medium: *mut KSIDENTIFIER) -> windows_core::Result<()>;
    fn KsPropagateAcquire(&self) -> windows_core::Result<()>;
    fn KsDeliver(&self, sample: Option<&super::DirectShow::IMediaSample>, flags: u32) -> windows_core::Result<()>;
    fn KsMediaSamplesCompleted(&self, streamsegment: *const KSSTREAM_SEGMENT) -> windows_core::Result<()>;
    fn KsPeekAllocator(&self, operation: KSPEEKOPERATION) -> Option<super::DirectShow::IMemAllocator>;
    fn KsReceiveAllocator(&self, memallocator: Option<&super::DirectShow::IMemAllocator>) -> windows_core::Result<()>;
    fn KsRenegotiateAllocator(&self) -> windows_core::Result<()>;
    fn KsIncrementPendingIoCount(&self) -> i32;
    fn KsDecrementPendingIoCount(&self) -> i32;
    fn KsQualityNotify(&self, proportion: u32, timedelta: i64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsPin {}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsPin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>() -> IKsPin_Vtbl {
        unsafe extern "system" fn KsQueryMediums<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediumlist: *mut *mut KSMULTIPLE_ITEM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsPin_Impl::KsQueryMediums(this) {
                Ok(ok__) => {
                    core::ptr::write(mediumlist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsQueryInterfaces<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interfacelist: *mut *mut KSMULTIPLE_ITEM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsPin_Impl::KsQueryInterfaces(this) {
                Ok(ok__) => {
                    core::ptr::write(interfacelist, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsCreateSinkPinHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interface: *const KSIDENTIFIER, medium: *const KSIDENTIFIER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsCreateSinkPinHandle(this, core::mem::transmute_copy(&interface), core::mem::transmute_copy(&medium)).into()
        }
        unsafe extern "system" fn KsGetCurrentCommunication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, communication: *mut KSPIN_COMMUNICATION, interface: *mut KSIDENTIFIER, medium: *mut KSIDENTIFIER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsGetCurrentCommunication(this, core::mem::transmute_copy(&communication), core::mem::transmute_copy(&interface), core::mem::transmute_copy(&medium)).into()
        }
        unsafe extern "system" fn KsPropagateAcquire<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsPropagateAcquire(this).into()
        }
        unsafe extern "system" fn KsDeliver<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsDeliver(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn KsMediaSamplesCompleted<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamsegment: *const KSSTREAM_SEGMENT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsMediaSamplesCompleted(this, core::mem::transmute_copy(&streamsegment)).into()
        }
        unsafe extern "system" fn KsPeekAllocator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: KSPEEKOPERATION) -> Option<super::DirectShow::IMemAllocator> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsPeekAllocator(this, core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn KsReceiveAllocator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, memallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsReceiveAllocator(this, windows_core::from_raw_borrowed(&memallocator)).into()
        }
        unsafe extern "system" fn KsRenegotiateAllocator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsRenegotiateAllocator(this).into()
        }
        unsafe extern "system" fn KsIncrementPendingIoCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsIncrementPendingIoCount(this)
        }
        unsafe extern "system" fn KsDecrementPendingIoCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsDecrementPendingIoCount(this)
        }
        unsafe extern "system" fn KsQualityNotify<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proportion: u32, timedelta: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPin_Impl::KsQualityNotify(this, core::mem::transmute_copy(&proportion), core::mem::transmute_copy(&timedelta)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsQueryMediums: KsQueryMediums::<Identity, Impl, OFFSET>,
            KsQueryInterfaces: KsQueryInterfaces::<Identity, Impl, OFFSET>,
            KsCreateSinkPinHandle: KsCreateSinkPinHandle::<Identity, Impl, OFFSET>,
            KsGetCurrentCommunication: KsGetCurrentCommunication::<Identity, Impl, OFFSET>,
            KsPropagateAcquire: KsPropagateAcquire::<Identity, Impl, OFFSET>,
            KsDeliver: KsDeliver::<Identity, Impl, OFFSET>,
            KsMediaSamplesCompleted: KsMediaSamplesCompleted::<Identity, Impl, OFFSET>,
            KsPeekAllocator: KsPeekAllocator::<Identity, Impl, OFFSET>,
            KsReceiveAllocator: KsReceiveAllocator::<Identity, Impl, OFFSET>,
            KsRenegotiateAllocator: KsRenegotiateAllocator::<Identity, Impl, OFFSET>,
            KsIncrementPendingIoCount: KsIncrementPendingIoCount::<Identity, Impl, OFFSET>,
            KsDecrementPendingIoCount: KsDecrementPendingIoCount::<Identity, Impl, OFFSET>,
            KsQualityNotify: KsQualityNotify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPinEx_Impl: Sized + IKsPin_Impl {
    fn KsNotifyError(&self, sample: Option<&super::DirectShow::IMediaSample>, hr: windows_core::HRESULT);
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsPinEx {}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsPinEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinEx_Impl, const OFFSET: isize>() -> IKsPinEx_Vtbl {
        unsafe extern "system" fn KsNotifyError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sample: *mut core::ffi::c_void, hr: windows_core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinEx_Impl::KsNotifyError(this, windows_core::from_raw_borrowed(&sample), core::mem::transmute_copy(&hr))
        }
        Self { base__: IKsPin_Vtbl::new::<Identity, Impl, OFFSET>(), KsNotifyError: KsNotifyError::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPinEx as windows_core::Interface>::IID || iid == &<IKsPin as windows_core::Interface>::IID
    }
}
pub trait IKsPinFactory_Impl: Sized {
    fn KsPinFactory(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IKsPinFactory {}
impl IKsPinFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinFactory_Impl, const OFFSET: isize>() -> IKsPinFactory_Vtbl {
        unsafe extern "system" fn KsPinFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfactory: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsPinFactory_Impl::KsPinFactory(this) {
                Ok(ok__) => {
                    core::ptr::write(pinfactory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), KsPinFactory: KsPinFactory::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPinFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IKsPinPipe_Impl: Sized {
    fn KsGetPinFramingCache(&self, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::Result<()>;
    fn KsSetPinFramingCache(&self, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::Result<()>;
    fn KsGetConnectedPin(&self) -> Option<super::DirectShow::IPin>;
    fn KsGetPipe(&self, operation: KSPEEKOPERATION) -> Option<IKsAllocatorEx>;
    fn KsSetPipe(&self, ksallocator: Option<&IKsAllocatorEx>) -> windows_core::Result<()>;
    fn KsGetPipeAllocatorFlag(&self) -> u32;
    fn KsSetPipeAllocatorFlag(&self, flag: u32) -> windows_core::Result<()>;
    fn KsGetPinBusCache(&self) -> windows_core::GUID;
    fn KsSetPinBusCache(&self, bus: &windows_core::GUID) -> windows_core::Result<()>;
    fn KsGetPinName(&self) -> windows_core::PWSTR;
    fn KsGetFilterName(&self) -> windows_core::PWSTR;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl windows_core::RuntimeName for IKsPinPipe {}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IKsPinPipe_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>() -> IKsPinPipe_Vtbl {
        unsafe extern "system" fn KsGetPinFramingCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framingex: *mut *mut KSALLOCATOR_FRAMING_EX, framingprop: *mut FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsGetPinFramingCache(this, core::mem::transmute_copy(&framingex), core::mem::transmute_copy(&framingprop), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn KsSetPinFramingCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framingex: *const KSALLOCATOR_FRAMING_EX, framingprop: *const FRAMING_PROP, option: FRAMING_CACHE_OPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsSetPinFramingCache(this, core::mem::transmute_copy(&framingex), core::mem::transmute_copy(&framingprop), core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn KsGetConnectedPin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<super::DirectShow::IPin> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsGetConnectedPin(this)
        }
        unsafe extern "system" fn KsGetPipe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operation: KSPEEKOPERATION) -> Option<IKsAllocatorEx> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsGetPipe(this, core::mem::transmute_copy(&operation))
        }
        unsafe extern "system" fn KsSetPipe<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ksallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsSetPipe(this, windows_core::from_raw_borrowed(&ksallocator)).into()
        }
        unsafe extern "system" fn KsGetPipeAllocatorFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsGetPipeAllocatorFlag(this)
        }
        unsafe extern "system" fn KsSetPipeAllocatorFlag<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flag: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsSetPipeAllocatorFlag(this, core::mem::transmute_copy(&flag)).into()
        }
        unsafe extern "system" fn KsGetPinBusCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = IKsPinPipe_Impl::KsGetPinBusCache(this)
        }
        unsafe extern "system" fn KsSetPinBusCache<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bus: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsSetPinBusCache(this, core::mem::transmute(&bus)).into()
        }
        unsafe extern "system" fn KsGetPinName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsGetPinName(this)
        }
        unsafe extern "system" fn KsGetFilterName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPinPipe_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::PWSTR {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPinPipe_Impl::KsGetFilterName(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsGetPinFramingCache: KsGetPinFramingCache::<Identity, Impl, OFFSET>,
            KsSetPinFramingCache: KsSetPinFramingCache::<Identity, Impl, OFFSET>,
            KsGetConnectedPin: KsGetConnectedPin::<Identity, Impl, OFFSET>,
            KsGetPipe: KsGetPipe::<Identity, Impl, OFFSET>,
            KsSetPipe: KsSetPipe::<Identity, Impl, OFFSET>,
            KsGetPipeAllocatorFlag: KsGetPipeAllocatorFlag::<Identity, Impl, OFFSET>,
            KsSetPipeAllocatorFlag: KsSetPipeAllocatorFlag::<Identity, Impl, OFFSET>,
            KsGetPinBusCache: KsGetPinBusCache::<Identity, Impl, OFFSET>,
            KsSetPinBusCache: KsSetPinBusCache::<Identity, Impl, OFFSET>,
            KsGetPinName: KsGetPinName::<Identity, Impl, OFFSET>,
            KsGetFilterName: KsGetFilterName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPinPipe as windows_core::Interface>::IID
    }
}
pub trait IKsPropertySet_Impl: Sized {
    fn Set(&self, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *const core::ffi::c_void, cbpropdata: u32) -> windows_core::Result<()>;
    fn Get(&self, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> windows_core::Result<()>;
    fn QuerySupported(&self, guidpropset: *const windows_core::GUID, dwpropid: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IKsPropertySet {}
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>() -> IKsPropertySet_Vtbl {
        unsafe extern "system" fn Set<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *const core::ffi::c_void, cbpropdata: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPropertySet_Impl::Set(this, core::mem::transmute_copy(&guidpropset), core::mem::transmute_copy(&dwpropid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&cbinstancedata), core::mem::transmute_copy(&ppropdata), core::mem::transmute_copy(&cbpropdata)).into()
        }
        unsafe extern "system" fn Get<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpropset: *const windows_core::GUID, dwpropid: u32, pinstancedata: *const core::ffi::c_void, cbinstancedata: u32, ppropdata: *mut core::ffi::c_void, cbpropdata: u32, pcbreturned: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsPropertySet_Impl::Get(this, core::mem::transmute_copy(&guidpropset), core::mem::transmute_copy(&dwpropid), core::mem::transmute_copy(&pinstancedata), core::mem::transmute_copy(&cbinstancedata), core::mem::transmute_copy(&ppropdata), core::mem::transmute_copy(&cbpropdata), core::mem::transmute_copy(&pcbreturned)).into()
        }
        unsafe extern "system" fn QuerySupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpropset: *const windows_core::GUID, dwpropid: u32, ptypesupport: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsPropertySet_Impl::QuerySupported(this, core::mem::transmute_copy(&guidpropset), core::mem::transmute_copy(&dwpropid)) {
                Ok(ok__) => {
                    core::ptr::write(ptypesupport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            QuerySupported: QuerySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsPropertySet as windows_core::Interface>::IID
    }
}
pub trait IKsQualityForwarder_Impl: Sized + IKsObject_Impl {
    fn KsFlushClient(&self, pin: Option<&IKsPin>);
}
impl windows_core::RuntimeName for IKsQualityForwarder {}
impl IKsQualityForwarder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsQualityForwarder_Impl, const OFFSET: isize>() -> IKsQualityForwarder_Vtbl {
        unsafe extern "system" fn KsFlushClient<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsQualityForwarder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsQualityForwarder_Impl::KsFlushClient(this, windows_core::from_raw_borrowed(&pin))
        }
        Self { base__: IKsObject_Vtbl::new::<Identity, Impl, OFFSET>(), KsFlushClient: KsFlushClient::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsQualityForwarder as windows_core::Interface>::IID || iid == &<IKsObject as windows_core::Interface>::IID
    }
}
pub trait IKsTopology_Impl: Sized {
    fn CreateNodeInstance(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: Option<&windows_core::IUnknown>, interfaceid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsTopology {}
impl IKsTopology_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: isize>() -> IKsTopology_Vtbl {
        unsafe extern "system" fn CreateNodeInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut core::ffi::c_void, interfaceid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsTopology_Impl::CreateNodeInstance(this, core::mem::transmute_copy(&nodeid), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&desiredaccess), windows_core::from_raw_borrowed(&unkouter), core::mem::transmute_copy(&interfaceid), core::mem::transmute_copy(&interface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsTopology as windows_core::Interface>::IID
    }
}
pub trait IKsTopologyInfo_Impl: Sized {
    fn NumCategories(&self) -> windows_core::Result<u32>;
    fn get_Category(&self, dwindex: u32) -> windows_core::Result<windows_core::GUID>;
    fn NumConnections(&self) -> windows_core::Result<u32>;
    fn get_ConnectionInfo(&self, dwindex: u32) -> windows_core::Result<KSTOPOLOGY_CONNECTION>;
    fn get_NodeName(&self, dwnodeid: u32, pwchnodename: windows_core::PWSTR, dwbufsize: u32, pdwnamelen: *mut u32) -> windows_core::Result<()>;
    fn NumNodes(&self) -> windows_core::Result<u32>;
    fn get_NodeType(&self, dwnodeid: u32) -> windows_core::Result<windows_core::GUID>;
    fn CreateNodeInstance(&self, dwnodeid: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IKsTopologyInfo {}
impl IKsTopologyInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>() -> IKsTopologyInfo_Vtbl {
        unsafe extern "system" fn NumCategories<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumcategories: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsTopologyInfo_Impl::NumCategories(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwnumcategories, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Category<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pcategory: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsTopologyInfo_Impl::get_Category(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    core::ptr::write(pcategory, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumConnections<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumconnections: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsTopologyInfo_Impl::NumConnections(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwnumconnections, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ConnectionInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pconnectioninfo: *mut KSTOPOLOGY_CONNECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsTopologyInfo_Impl::get_ConnectionInfo(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    core::ptr::write(pconnectioninfo, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NodeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32, pwchnodename: windows_core::PWSTR, dwbufsize: u32, pdwnamelen: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsTopologyInfo_Impl::get_NodeName(this, core::mem::transmute_copy(&dwnodeid), core::mem::transmute_copy(&pwchnodename), core::mem::transmute_copy(&dwbufsize), core::mem::transmute_copy(&pdwnamelen)).into()
        }
        unsafe extern "system" fn NumNodes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwnumnodes: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsTopologyInfo_Impl::NumNodes(this) {
                Ok(ok__) => {
                    core::ptr::write(pdwnumnodes, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_NodeType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32, pnodetype: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IKsTopologyInfo_Impl::get_NodeType(this, core::mem::transmute_copy(&dwnodeid)) {
                Ok(ok__) => {
                    core::ptr::write(pnodetype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNodeInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IKsTopologyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnodeid: u32, iid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IKsTopologyInfo_Impl::CreateNodeInstance(this, core::mem::transmute_copy(&dwnodeid), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NumCategories: NumCategories::<Identity, Impl, OFFSET>,
            get_Category: get_Category::<Identity, Impl, OFFSET>,
            NumConnections: NumConnections::<Identity, Impl, OFFSET>,
            get_ConnectionInfo: get_ConnectionInfo::<Identity, Impl, OFFSET>,
            get_NodeName: get_NodeName::<Identity, Impl, OFFSET>,
            NumNodes: NumNodes::<Identity, Impl, OFFSET>,
            get_NodeType: get_NodeType::<Identity, Impl, OFFSET>,
            CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsTopologyInfo as windows_core::Interface>::IID
    }
}
