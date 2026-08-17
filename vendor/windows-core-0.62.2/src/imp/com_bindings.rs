#[inline]
pub unsafe fn RoGetAgileReference<P2>(
    options: AgileReferenceOptions,
    riid: *const windows_core::GUID,
    punk: P2,
) -> windows_core::Result<IAgileReference>
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("combase.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, ppagilereference : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetAgileReference(options, riid, punk.param().abi(), &mut result__)
            .and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AgileReferenceOptions(pub i32);
pub const CO_E_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800401F0_u32 as _);
pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80004002_u32 as _);
pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
windows_core::imp::define_interface!(
    IAgileObject,
    IAgileObject_Vtbl,
    0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90
);
windows_core::imp::interface_hierarchy!(IAgileObject, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IAgileObject_Impl: windows_core::IUnknownImpl {}
impl IAgileObject_Vtbl {
    pub const fn new<Identity: IAgileObject_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAgileObject {}
windows_core::imp::define_interface!(
    IAgileReference,
    IAgileReference_Vtbl,
    0xc03f6a43_65a4_9818_987e_e0b810d2a6f2
);
windows_core::imp::interface_hierarchy!(IAgileReference, windows_core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).Resolve)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IAgileReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(
        &self,
        riid: *const windows_core::GUID,
        ppvobjectreference: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl IAgileReference_Vtbl {
    pub const fn new<Identity: IAgileReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IAgileReference_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppvobjectreference: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAgileReference_Impl::Resolve(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvobjectreference),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Resolve: Resolve::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAgileReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAgileReference {}
windows_core::imp::define_interface!(
    IWeakReference,
    IWeakReference_Vtbl,
    0x00000037_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IWeakReference, windows_core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).Resolve)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IWeakReference_Impl: windows_core::IUnknownImpl {
    fn Resolve(
        &self,
        riid: *const windows_core::GUID,
        objectreference: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl IWeakReference_Vtbl {
    pub const fn new<Identity: IWeakReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IWeakReference_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            objectreference: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeakReference_Impl::Resolve(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&objectreference),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Resolve: Resolve::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReference as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWeakReference {}
windows_core::imp::define_interface!(
    IWeakReferenceSource,
    IWeakReferenceSource_Vtbl,
    0x00000038_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IWeakReferenceSource, windows_core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> windows_core::Result<IWeakReference> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWeakReference)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IWeakReferenceSource_Impl: windows_core::IUnknownImpl {
    fn GetWeakReference(&self) -> windows_core::Result<IWeakReference>;
}
impl IWeakReferenceSource_Vtbl {
    pub const fn new<Identity: IWeakReferenceSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWeakReference<
            Identity: IWeakReferenceSource_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            weakreference: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWeakReferenceSource_Impl::GetWeakReference(this) {
                    Ok(ok__) => {
                        weakreference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWeakReference: GetWeakReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeakReferenceSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWeakReferenceSource {}
pub const JSCRIPT_E_CANTEXECUTE: windows_core::HRESULT = windows_core::HRESULT(0x89020001_u32 as _);
pub const REGDB_E_CLASSNOTREG: windows_core::HRESULT = windows_core::HRESULT(0x80040154_u32 as _);
pub const RPC_E_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80010108_u32 as _);
pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
