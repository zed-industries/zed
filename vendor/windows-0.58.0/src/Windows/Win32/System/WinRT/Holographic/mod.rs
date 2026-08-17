windows_core::imp::define_interface!(IHolographicCameraInterop, IHolographicCameraInterop_Vtbl, 0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
impl core::ops::Deref for IHolographicCameraInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHolographicCameraInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicCameraInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12BackBufferResource<P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirect3D12BackBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<P0, P1>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P1) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirect3D12HardwareProtectedBackBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, pprotectedresourcesession.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi(), duration).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<P0>(&self, presourcetounacquire: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetounacquire.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IHolographicCameraInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12BackBufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12BackBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedBackBufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedBackBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
windows_core::imp::define_interface!(IHolographicCameraRenderingParametersInterop, IHolographicCameraRenderingParametersInterop_Vtbl, 0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
impl core::ops::Deref for IHolographicCameraRenderingParametersInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHolographicCameraRenderingParametersInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicCameraRenderingParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).CommitDirect3D12Resource)(windows_core::Interface::as_raw(self), pcolorresourcetocommit.param().abi(), pcolorresourcefence.param().abi(), colorresourcefencesignalvalue).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<P0, P1, P2, P3>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: P2, pdepthresourcefence: P3, depthresourcefencesignalvalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
        P2: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P3: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).CommitDirect3D12ResourceWithDepthData)(windows_core::Interface::as_raw(self), pcolorresourcetocommit.param().abi(), pcolorresourcefence.param().abi(), colorresourcefencesignalvalue, pdepthresourcetocommit.param().abi(), pdepthresourcefence.param().abi(), depthresourcefencesignalvalue).ok()
    }
}
#[repr(C)]
pub struct IHolographicCameraRenderingParametersInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12ResourceWithDepthData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12ResourceWithDepthData: usize,
}
windows_core::imp::define_interface!(IHolographicQuadLayerInterop, IHolographicQuadLayerInterop_Vtbl, 0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
impl core::ops::Deref for IHolographicQuadLayerInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHolographicQuadLayerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicQuadLayerInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12ContentBufferResource<P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirect3D12ContentBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<P0, P1>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P1) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDirect3D12HardwareProtectedContentBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, pprotectedresourcesession.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi(), duration).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<P0>(&self, presourcetounacquire: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        (windows_core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetounacquire.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IHolographicQuadLayerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12ContentBufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12ContentBufferResource: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateDirect3D12HardwareProtectedContentBufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateDirect3D12HardwareProtectedContentBufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResource: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub AcquireDirect3D12BufferResourceWithTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    AcquireDirect3D12BufferResourceWithTimeout: usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub UnacquireDirect3D12BufferResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    UnacquireDirect3D12BufferResource: usize,
}
windows_core::imp::define_interface!(IHolographicQuadLayerUpdateParametersInterop, IHolographicQuadLayerUpdateParametersInterop_Vtbl, 0xe5f549cd_c909_444f_8809_7cc18a9c8920);
impl core::ops::Deref for IHolographicQuadLayerUpdateParametersInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHolographicQuadLayerUpdateParametersInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        (windows_core::Interface::vtable(self).CommitDirect3D12Resource)(windows_core::Interface::as_raw(self), pcolorresourcetocommit.param().abi(), pcolorresourcefence.param().abi(), colorresourcefencesignalvalue).ok()
    }
}
#[repr(C)]
pub struct IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
