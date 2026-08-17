pub trait IDirect3DDevice_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn Trim(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirect3DDevice {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
}
impl IDirect3DDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice_Impl, const OFFSET: isize>() -> IDirect3DDevice_Vtbl {
        unsafe extern "system" fn Trim<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDirect3DDevice_Impl::Trim(this).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDirect3DDevice, OFFSET>(), Trim: Trim::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DDevice as windows_core::Interface>::IID
    }
}
pub trait IDirect3DSurface_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn Description(&self) -> windows_core::Result<Direct3DSurfaceDescription>;
}
impl windows_core::RuntimeName for IDirect3DSurface {
    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
}
impl IDirect3DSurface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface_Impl, const OFFSET: isize>() -> IDirect3DSurface_Vtbl {
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirect3DSurface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDirect3DSurface_Impl::Description(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDirect3DSurface, OFFSET>(), Description: Description::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirect3DSurface as windows_core::Interface>::IID
    }
}
