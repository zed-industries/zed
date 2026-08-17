windows_core::imp::define_interface!(ICoreFrameworkInputViewInterop, ICoreFrameworkInputViewInterop_Vtbl, 0x0e3da342_b11c_484b_9c1c_be0d61c2f6c5);
windows_core::imp::interface_hierarchy!(ICoreFrameworkInputViewInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreFrameworkInputViewInterop {
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::super::super::Foundation::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreFrameworkInputViewInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, coreframeworkinputview: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ICoreFrameworkInputViewInterop_Vtbl {
    pub const fn new<Identity: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, coreframeworkinputview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreFrameworkInputViewInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&coreframeworkinputview)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreFrameworkInputViewInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreFrameworkInputViewInterop {}
