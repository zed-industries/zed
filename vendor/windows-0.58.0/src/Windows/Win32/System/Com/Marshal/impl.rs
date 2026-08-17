pub trait IMarshal_Impl: Sized {
    fn GetUnmarshalClass(&self, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::Result<windows_core::GUID>;
    fn GetMarshalSizeMax(&self, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::Result<u32>;
    fn MarshalInterface(&self, pstm: Option<&super::IStream>, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::Result<()>;
    fn UnmarshalInterface(&self, pstm: Option<&super::IStream>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseMarshalData(&self, pstm: Option<&super::IStream>) -> windows_core::Result<()>;
    fn DisconnectObject(&self, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMarshal {}
impl IMarshal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMarshal_Vtbl
    where
        Identity: IMarshal_Impl,
    {
        unsafe extern "system" fn GetUnmarshalClass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32, pcid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMarshal_Impl::GetUnmarshalClass(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)) {
                Ok(ok__) => {
                    pcid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarshalSizeMax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMarshal_Impl::GetMarshalSizeMax(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)) {
                Ok(ok__) => {
                    psize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarshalInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, riid: *const windows_core::GUID, pv: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, mshlflags: u32) -> windows_core::HRESULT
        where
            Identity: IMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMarshal_Impl::MarshalInterface(this, windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&dwdestcontext), core::mem::transmute_copy(&pvdestcontext), core::mem::transmute_copy(&mshlflags)).into()
        }
        unsafe extern "system" fn UnmarshalInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMarshal_Impl::UnmarshalInterface(this, windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ReleaseMarshalData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMarshal_Impl::ReleaseMarshalData(this, windows_core::from_raw_borrowed(&pstm)).into()
        }
        unsafe extern "system" fn DisconnectObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT
        where
            Identity: IMarshal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMarshal_Impl::DisconnectObject(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUnmarshalClass: GetUnmarshalClass::<Identity, OFFSET>,
            GetMarshalSizeMax: GetMarshalSizeMax::<Identity, OFFSET>,
            MarshalInterface: MarshalInterface::<Identity, OFFSET>,
            UnmarshalInterface: UnmarshalInterface::<Identity, OFFSET>,
            ReleaseMarshalData: ReleaseMarshalData::<Identity, OFFSET>,
            DisconnectObject: DisconnectObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshal as windows_core::Interface>::IID
    }
}
pub trait IMarshal2_Impl: Sized + IMarshal_Impl {}
impl windows_core::RuntimeName for IMarshal2 {}
impl IMarshal2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMarshal2_Vtbl
    where
        Identity: IMarshal2_Impl,
    {
        Self { base__: IMarshal_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshal2 as windows_core::Interface>::IID || iid == &<IMarshal as windows_core::Interface>::IID
    }
}
pub trait IMarshalingStream_Impl: Sized + super::IStream_Impl {
    fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> windows_core::Result<usize>;
}
impl windows_core::RuntimeName for IMarshalingStream {}
impl IMarshalingStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMarshalingStream_Vtbl
    where
        Identity: IMarshalingStream_Impl,
    {
        unsafe extern "system" fn GetMarshalingContextAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> windows_core::HRESULT
        where
            Identity: IMarshalingStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMarshalingStream_Impl::GetMarshalingContextAttribute(this, core::mem::transmute_copy(&attribute)) {
                Ok(ok__) => {
                    pattributevalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::IStream_Vtbl::new::<Identity, OFFSET>(), GetMarshalingContextAttribute: GetMarshalingContextAttribute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMarshalingStream as windows_core::Interface>::IID || iid == &<super::ISequentialStream as windows_core::Interface>::IID || iid == &<super::IStream as windows_core::Interface>::IID
    }
}
