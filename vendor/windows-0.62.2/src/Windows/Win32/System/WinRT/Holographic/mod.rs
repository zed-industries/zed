windows_core::imp::define_interface!(IHolographicCameraInterop, IHolographicCameraInterop_Vtbl, 0x7cc1f9c5_6d02_41fa_9500_e1809eb48eec);
windows_core::imp::interface_hierarchy!(IHolographicCameraInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicCameraInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12BackBufferResource<P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirect3D12BackBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<P0, P2>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P2) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
        P2: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirect3D12HardwareProtectedBackBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, pprotectedresourcesession.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi(), duration).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<P0>(&self, presourcetounacquire: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetounacquire.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicCameraInterop_Impl: windows_core::IUnknownImpl {
    fn CreateDirect3D12BackBufferResource(&self, pdevice: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedBackBufferResource(&self, pdevice: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&self, presourcetoacquire: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> windows_core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&self, presourcetoacquire: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> windows_core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&self, presourcetounacquire: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicCameraInterop_Vtbl {
    pub const fn new<Identity: IHolographicCameraInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDirect3D12BackBufferResource<Identity: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHolographicCameraInterop_Impl::CreateDirect3D12BackBufferResource(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&ptexture2ddesc)) {
                    Ok(ok__) => {
                        ppcreatedtexture2dresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedBackBufferResource<Identity: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHolographicCameraInterop_Impl::CreateDirect3D12HardwareProtectedBackBufferResource(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&ptexture2ddesc), core::mem::transmute_copy(&pprotectedresourcesession)) {
                    Ok(ok__) => {
                        ppcreatedtexture2dresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcetoacquire: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicCameraInterop_Impl::AcquireDirect3D12BufferResource(this, core::mem::transmute_copy(&presourcetoacquire), core::mem::transmute_copy(&pcommandqueue)).into()
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcetoacquire: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void, duration: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicCameraInterop_Impl::AcquireDirect3D12BufferResourceWithTimeout(this, core::mem::transmute_copy(&presourcetoacquire), core::mem::transmute_copy(&pcommandqueue), core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: IHolographicCameraInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcetounacquire: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicCameraInterop_Impl::UnacquireDirect3D12BufferResource(this, core::mem::transmute_copy(&presourcetounacquire)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicCameraInterop, OFFSET>(),
            CreateDirect3D12BackBufferResource: CreateDirect3D12BackBufferResource::<Identity, OFFSET>,
            CreateDirect3D12HardwareProtectedBackBufferResource: CreateDirect3D12HardwareProtectedBackBufferResource::<Identity, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicCameraInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for IHolographicCameraInterop {}
windows_core::imp::define_interface!(IHolographicCameraRenderingParametersInterop, IHolographicCameraRenderingParametersInterop_Vtbl, 0xf75b68d6_d1fd_4707_aafd_fa6f4c0e3bf4);
windows_core::imp::interface_hierarchy!(IHolographicCameraRenderingParametersInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicCameraRenderingParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        unsafe { (windows_core::Interface::vtable(self).CommitDirect3D12Resource)(windows_core::Interface::as_raw(self), pcolorresourcetocommit.param().abi(), pcolorresourcefence.param().abi(), colorresourcefencesignalvalue).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<P0, P1, P3, P4>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: P3, pdepthresourcefence: P4, depthresourcefencesignalvalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
        P3: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P4: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        unsafe { (windows_core::Interface::vtable(self).CommitDirect3D12ResourceWithDepthData)(windows_core::Interface::as_raw(self), pcolorresourcetocommit.param().abi(), pcolorresourcefence.param().abi(), colorresourcefencesignalvalue, pdepthresourcetocommit.param().abi(), pdepthresourcefence.param().abi(), depthresourcefencesignalvalue).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicCameraRenderingParametersInterop_Impl: windows_core::IUnknownImpl {
    fn CommitDirect3D12Resource(&self, pcolorresourcetocommit: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> windows_core::Result<()>;
    fn CommitDirect3D12ResourceWithDepthData(&self, pcolorresourcetocommit: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pdepthresourcefence: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Fence>, depthresourcefencesignalvalue: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicCameraRenderingParametersInterop_Vtbl {
    pub const fn new<Identity: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolorresourcetocommit: *mut core::ffi::c_void, pcolorresourcefence: *mut core::ffi::c_void, colorresourcefencesignalvalue: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicCameraRenderingParametersInterop_Impl::CommitDirect3D12Resource(this, core::mem::transmute_copy(&pcolorresourcetocommit), core::mem::transmute_copy(&pcolorresourcefence), core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
            }
        }
        unsafe extern "system" fn CommitDirect3D12ResourceWithDepthData<Identity: IHolographicCameraRenderingParametersInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolorresourcetocommit: *mut core::ffi::c_void, pcolorresourcefence: *mut core::ffi::c_void, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: *mut core::ffi::c_void, pdepthresourcefence: *mut core::ffi::c_void, depthresourcefencesignalvalue: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicCameraRenderingParametersInterop_Impl::CommitDirect3D12ResourceWithDepthData(this, core::mem::transmute_copy(&pcolorresourcetocommit), core::mem::transmute_copy(&pcolorresourcefence), core::mem::transmute_copy(&colorresourcefencesignalvalue), core::mem::transmute_copy(&pdepthresourcetocommit), core::mem::transmute_copy(&pdepthresourcefence), core::mem::transmute_copy(&depthresourcefencesignalvalue)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicCameraRenderingParametersInterop, OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, OFFSET>,
            CommitDirect3D12ResourceWithDepthData: CommitDirect3D12ResourceWithDepthData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParametersInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IHolographicCameraRenderingParametersInterop {}
windows_core::imp::define_interface!(IHolographicQuadLayerInterop, IHolographicQuadLayerInterop_Vtbl, 0xcfa688f0_639e_4a47_83d7_6b7f5ebf7fed);
windows_core::imp::interface_hierarchy!(IHolographicQuadLayerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicQuadLayerInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12ContentBufferResource<P0>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirect3D12ContentBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<P0, P2>(&self, pdevice: P0, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: P2) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Device>,
        P2: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirect3D12HardwareProtectedContentBufferResource)(windows_core::Interface::as_raw(self), pdevice.param().abi(), ptexture2ddesc, pprotectedresourcesession.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResource<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<P0, P1>(&self, presourcetoacquire: P0, pcommandqueue: P1, duration: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>,
    {
        unsafe { (windows_core::Interface::vtable(self).AcquireDirect3D12BufferResourceWithTimeout)(windows_core::Interface::as_raw(self), presourcetoacquire.param().abi(), pcommandqueue.param().abi(), duration).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn UnacquireDirect3D12BufferResource<P0>(&self, presourcetounacquire: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnacquireDirect3D12BufferResource)(windows_core::Interface::as_raw(self), presourcetounacquire.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IHolographicQuadLayerInterop_Impl: windows_core::IUnknownImpl {
    fn CreateDirect3D12ContentBufferResource(&self, pdevice: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn CreateDirect3D12HardwareProtectedContentBufferResource(&self, pdevice: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Device>, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>) -> windows_core::Result<super::super::super::Graphics::Direct3D12::ID3D12Resource>;
    fn AcquireDirect3D12BufferResource(&self, presourcetoacquire: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>) -> windows_core::Result<()>;
    fn AcquireDirect3D12BufferResourceWithTimeout(&self, presourcetoacquire: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcommandqueue: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12CommandQueue>, duration: u64) -> windows_core::Result<()>;
    fn UnacquireDirect3D12BufferResource(&self, presourcetounacquire: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl IHolographicQuadLayerInterop_Vtbl {
    pub const fn new<Identity: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDirect3D12ContentBufferResource<Identity: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHolographicQuadLayerInterop_Impl::CreateDirect3D12ContentBufferResource(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&ptexture2ddesc)) {
                    Ok(ok__) => {
                        pptexture2dresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDirect3D12HardwareProtectedContentBufferResource<Identity: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, ptexture2ddesc: *const super::super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: *mut core::ffi::c_void, ppcreatedtexture2dresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHolographicQuadLayerInterop_Impl::CreateDirect3D12HardwareProtectedContentBufferResource(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&ptexture2ddesc), core::mem::transmute_copy(&pprotectedresourcesession)) {
                    Ok(ok__) => {
                        ppcreatedtexture2dresource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResource<Identity: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcetoacquire: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicQuadLayerInterop_Impl::AcquireDirect3D12BufferResource(this, core::mem::transmute_copy(&presourcetoacquire), core::mem::transmute_copy(&pcommandqueue)).into()
            }
        }
        unsafe extern "system" fn AcquireDirect3D12BufferResourceWithTimeout<Identity: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcetoacquire: *mut core::ffi::c_void, pcommandqueue: *mut core::ffi::c_void, duration: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicQuadLayerInterop_Impl::AcquireDirect3D12BufferResourceWithTimeout(this, core::mem::transmute_copy(&presourcetoacquire), core::mem::transmute_copy(&pcommandqueue), core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn UnacquireDirect3D12BufferResource<Identity: IHolographicQuadLayerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcetounacquire: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicQuadLayerInterop_Impl::UnacquireDirect3D12BufferResource(this, core::mem::transmute_copy(&presourcetounacquire)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicQuadLayerInterop, OFFSET>(),
            CreateDirect3D12ContentBufferResource: CreateDirect3D12ContentBufferResource::<Identity, OFFSET>,
            CreateDirect3D12HardwareProtectedContentBufferResource: CreateDirect3D12HardwareProtectedContentBufferResource::<Identity, OFFSET>,
            AcquireDirect3D12BufferResource: AcquireDirect3D12BufferResource::<Identity, OFFSET>,
            AcquireDirect3D12BufferResourceWithTimeout: AcquireDirect3D12BufferResourceWithTimeout::<Identity, OFFSET>,
            UnacquireDirect3D12BufferResource: UnacquireDirect3D12BufferResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicQuadLayerInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for IHolographicQuadLayerInterop {}
windows_core::imp::define_interface!(IHolographicQuadLayerUpdateParametersInterop, IHolographicQuadLayerUpdateParametersInterop_Vtbl, 0xe5f549cd_c909_444f_8809_7cc18a9c8920);
windows_core::imp::interface_hierarchy!(IHolographicQuadLayerUpdateParametersInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CommitDirect3D12Resource<P0, P1>(&self, pcolorresourcetocommit: P0, pcolorresourcefence: P1, colorresourcefencesignalvalue: u64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Resource>,
        P1: windows_core::Param<super::super::super::Graphics::Direct3D12::ID3D12Fence>,
    {
        unsafe { (windows_core::Interface::vtable(self).CommitDirect3D12Resource)(windows_core::Interface::as_raw(self), pcolorresourcetocommit.param().abi(), pcolorresourcefence.param().abi(), colorresourcefencesignalvalue).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CommitDirect3D12Resource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CommitDirect3D12Resource: usize,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub trait IHolographicQuadLayerUpdateParametersInterop_Impl: windows_core::IUnknownImpl {
    fn CommitDirect3D12Resource(&self, pcolorresourcetocommit: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Resource>, pcolorresourcefence: windows_core::Ref<super::super::super::Graphics::Direct3D12::ID3D12Fence>, colorresourcefencesignalvalue: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl IHolographicQuadLayerUpdateParametersInterop_Vtbl {
    pub const fn new<Identity: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CommitDirect3D12Resource<Identity: IHolographicQuadLayerUpdateParametersInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcolorresourcetocommit: *mut core::ffi::c_void, pcolorresourcefence: *mut core::ffi::c_void, colorresourcefencesignalvalue: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHolographicQuadLayerUpdateParametersInterop_Impl::CommitDirect3D12Resource(this, core::mem::transmute_copy(&pcolorresourcetocommit), core::mem::transmute_copy(&pcolorresourcefence), core::mem::transmute_copy(&colorresourcefencesignalvalue)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHolographicQuadLayerUpdateParametersInterop, OFFSET>(),
            CommitDirect3D12Resource: CommitDirect3D12Resource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHolographicQuadLayerUpdateParametersInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl windows_core::RuntimeName for IHolographicQuadLayerUpdateParametersInterop {}
