#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[inline]
pub unsafe fn CoCreateGuid() -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoCreateGuid(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn RoGetAgileReference<P0>(
    options: AgileReferenceOptions,
    riid: *const windows_core::GUID,
    punk: P0,
) -> windows_core::Result<IAgileReference>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn RoGetAgileReference(options : AgileReferenceOptions, riid : *const windows_core::GUID, punk : * mut core::ffi::c_void, ppagilereference : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RoGetAgileReference(options, riid, punk.param().abi(), &mut result__)
        .and_then(|| windows_core::Type::from_abi(result__))
}
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AgileReferenceOptions(pub i32);
impl windows_core::TypeKind for AgileReferenceOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AgileReferenceOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AgileReferenceOptions")
            .field(&self.0)
            .finish()
    }
}
pub const CO_E_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800401F0_u32 as _);
pub const E_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x8000000B_u32 as _);
pub const E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80004002_u32 as _);
pub const E_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8007000E_u32 as _);
pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
windows_core::imp::define_interface!(
    IAgileObject,
    IAgileObject_Vtbl,
    0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90
);
impl core::ops::Deref for IAgileObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAgileObject, windows_core::IUnknown);
impl IAgileObject {}
#[repr(C)]
pub struct IAgileObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(
    IAgileReference,
    IAgileReference_Vtbl,
    0xc03f6a43_65a4_9818_987e_e0b810d2a6f2
);
impl core::ops::Deref for IAgileReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAgileReference, windows_core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Resolve)(
            windows_core::Interface::as_raw(self),
            &T::IID,
            &mut result__,
        )
        .and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAgileReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWeakReference,
    IWeakReference_Vtbl,
    0x00000037_0000_0000_c000_000000000046
);
impl core::ops::Deref for IWeakReference {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWeakReference, windows_core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Resolve)(
            windows_core::Interface::as_raw(self),
            &T::IID,
            &mut result__,
        )
        .and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWeakReference_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWeakReferenceSource,
    IWeakReferenceSource_Vtbl,
    0x00000038_0000_0000_c000_000000000046
);
impl core::ops::Deref for IWeakReferenceSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWeakReferenceSource, windows_core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> windows_core::Result<IWeakReference> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWeakReference)(
            windows_core::Interface::as_raw(self),
            &mut result__,
        )
        .and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub const JSCRIPT_E_CANTEXECUTE: windows_core::HRESULT = windows_core::HRESULT(0x89020001_u32 as _);
pub const RPC_E_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80010108_u32 as _);
pub const TYPE_E_TYPEMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80028CA0_u32 as _);
