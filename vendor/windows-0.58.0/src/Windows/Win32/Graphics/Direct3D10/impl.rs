pub trait ID3D10Asynchronous_Impl: Sized + ID3D10DeviceChild_Impl {
    fn Begin(&self);
    fn End(&self);
    fn GetData(&self, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::Result<()>;
    fn GetDataSize(&self) -> u32;
}
impl windows_core::RuntimeName for ID3D10Asynchronous {}
impl ID3D10Asynchronous_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Asynchronous_Vtbl
    where
        Identity: ID3D10Asynchronous_Impl,
    {
        unsafe extern "system" fn Begin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Asynchronous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Asynchronous_Impl::Begin(this)
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Asynchronous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Asynchronous_Impl::End(this)
        }
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Asynchronous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Asynchronous_Impl::GetData(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&getdataflags)).into()
        }
        unsafe extern "system" fn GetDataSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10Asynchronous_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Asynchronous_Impl::GetDataSize(this)
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            Begin: Begin::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetDataSize: GetDataSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Asynchronous as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10BlendState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC);
}
impl windows_core::RuntimeName for ID3D10BlendState {}
impl ID3D10BlendState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10BlendState_Vtbl
    where
        Identity: ID3D10BlendState_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC)
        where
            Identity: ID3D10BlendState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10BlendState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10BlendState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10BlendState1_Impl: Sized + ID3D10BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_BLEND_DESC1);
}
impl windows_core::RuntimeName for ID3D10BlendState1 {}
impl ID3D10BlendState1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10BlendState1_Vtbl
    where
        Identity: ID3D10BlendState1_Impl,
    {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BLEND_DESC1)
        where
            Identity: ID3D10BlendState1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10BlendState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10BlendState_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10BlendState1 as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10BlendState as windows_core::Interface>::IID
    }
}
pub trait ID3D10Buffer_Impl: Sized + ID3D10Resource_Impl {
    fn Map(&self, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self);
    fn GetDesc(&self, pdesc: *mut D3D10_BUFFER_DESC);
}
impl windows_core::RuntimeName for ID3D10Buffer {}
impl ID3D10Buffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Buffer_Vtbl
    where
        Identity: ID3D10Buffer_Impl,
    {
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Buffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Buffer_Impl::Map(this, core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Buffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Buffer_Impl::Unmap(this)
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_BUFFER_DESC)
        where
            Identity: ID3D10Buffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Buffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Buffer as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
pub trait ID3D10Counter_Impl: Sized + ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_COUNTER_DESC);
}
impl windows_core::RuntimeName for ID3D10Counter {}
impl ID3D10Counter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Counter_Vtbl
    where
        Identity: ID3D10Counter_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_COUNTER_DESC)
        where
            Identity: ID3D10Counter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Counter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Counter as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D10Debug_Impl: Sized {
    fn SetFeatureMask(&self, mask: u32) -> windows_core::Result<()>;
    fn GetFeatureMask(&self) -> u32;
    fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> windows_core::Result<()>;
    fn GetPresentPerRenderOpDelay(&self) -> u32;
    fn SetSwapChain(&self, pswapchain: Option<&super::Dxgi::IDXGISwapChain>) -> windows_core::Result<()>;
    fn GetSwapChain(&self) -> windows_core::Result<super::Dxgi::IDXGISwapChain>;
    fn Validate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for ID3D10Debug {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D10Debug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Debug_Vtbl
    where
        Identity: ID3D10Debug_Impl,
    {
        unsafe extern "system" fn SetFeatureMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Debug_Impl::SetFeatureMask(this, core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Debug_Impl::GetFeatureMask(this)
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, milliseconds: u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Debug_Impl::SetPresentPerRenderOpDelay(this, core::mem::transmute_copy(&milliseconds)).into()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Debug_Impl::GetPresentPerRenderOpDelay(this)
        }
        unsafe extern "system" fn SetSwapChain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pswapchain: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Debug_Impl::SetSwapChain(this, windows_core::from_raw_borrowed(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Debug_Impl::GetSwapChain(this) {
                Ok(ok__) => {
                    ppswapchain.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Debug_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Debug_Impl::Validate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Debug as windows_core::Interface>::IID
    }
}
pub trait ID3D10DepthStencilState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_DESC);
}
impl windows_core::RuntimeName for ID3D10DepthStencilState {}
impl ID3D10DepthStencilState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10DepthStencilState_Vtbl
    where
        Identity: ID3D10DepthStencilState_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_DESC)
        where
            Identity: ID3D10DepthStencilState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10DepthStencilState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DepthStencilState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10DepthStencilView_Impl: Sized + ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10DepthStencilView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10DepthStencilView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10DepthStencilView_Vtbl
    where
        Identity: ID3D10DepthStencilView_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_DEPTH_STENCIL_VIEW_DESC)
        where
            Identity: ID3D10DepthStencilView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10DepthStencilView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DepthStencilView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device_Impl: Sized {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: Option<&ID3D10PixelShader>);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn VSSetShader(&self, pvertexshader: Option<&ID3D10VertexShader>);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn IASetInputLayout(&self, pinputlayout: Option<&ID3D10InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const Option<ID3D10Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: Option<&ID3D10Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D10Buffer>);
    fn GSSetShader(&self, pshader: Option<&ID3D10GeometryShader>);
    fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn SetPredication(&self, ppredicate: Option<&ID3D10Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D10ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D10SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const Option<ID3D10RenderTargetView>, pdepthstencilview: Option<&ID3D10DepthStencilView>);
    fn OMSetBlendState(&self, pblendstate: Option<&ID3D10BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: Option<&ID3D10DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const Option<ID3D10Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn RSSetState(&self, prasterizerstate: Option<&ID3D10RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D10_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&self, pdstresource: Option<&ID3D10Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: Option<&ID3D10Resource>, srcsubresource: u32, psrcbox: *const D3D10_BOX);
    fn CopyResource(&self, pdstresource: Option<&ID3D10Resource>, psrcresource: Option<&ID3D10Resource>);
    fn UpdateSubresource(&self, pdstresource: Option<&ID3D10Resource>, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ClearRenderTargetView(&self, prendertargetview: Option<&ID3D10RenderTargetView>, colorrgba: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: Option<&ID3D10DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: Option<&ID3D10ShaderResourceView>);
    fn ResolveSubresource(&self, pdstresource: Option<&ID3D10Resource>, dstsubresource: u32, psrcresource: Option<&ID3D10Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: *mut Option<ID3D10PixelShader>);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn VSGetShader(&self, ppvertexshader: *mut Option<ID3D10VertexShader>);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: *mut Option<ID3D10InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut Option<ID3D10Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: *mut Option<ID3D10Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D10Buffer>);
    fn GSGetShader(&self, ppgeometryshader: *mut Option<ID3D10GeometryShader>);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn GetPredication(&self, pppredicate: *mut Option<ID3D10Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D10ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D10SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut Option<ID3D10RenderTargetView>, ppdepthstencilview: *mut Option<ID3D10DepthStencilView>);
    fn OMGetBlendState(&self, ppblendstate: *mut Option<ID3D10BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut Option<ID3D10DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut Option<ID3D10Buffer>, poffsets: *mut u32);
    fn RSGetState(&self, pprasterizerstate: *mut Option<ID3D10RasterizerState>);
    fn RSGetViewports(&self, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT);
    fn RSGetScissorRects(&self, numrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ClearState(&self);
    fn Flush(&self);
    fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut Option<ID3D10Buffer>) -> windows_core::Result<()>;
    fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture1D>;
    fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture2D>;
    fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA) -> windows_core::Result<ID3D10Texture3D>;
    fn CreateShaderResourceView(&self, presource: Option<&ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut Option<ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn CreateRenderTargetView(&self, presource: Option<&ID3D10Resource>, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut Option<ID3D10RenderTargetView>) -> windows_core::Result<()>;
    fn CreateDepthStencilView(&self, presource: Option<&ID3D10Resource>, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut Option<ID3D10DepthStencilView>) -> windows_core::Result<()>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut Option<ID3D10InputLayout>) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut Option<ID3D10VertexShader>) -> windows_core::Result<()>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut Option<ID3D10GeometryShader>) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut Option<ID3D10GeometryShader>) -> windows_core::Result<()>;
    fn CreatePixelShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut Option<ID3D10PixelShader>) -> windows_core::Result<()>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut Option<ID3D10BlendState>) -> windows_core::Result<()>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut Option<ID3D10DepthStencilState>) -> windows_core::Result<()>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut Option<ID3D10RasterizerState>) -> windows_core::Result<()>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut Option<ID3D10SamplerState>) -> windows_core::Result<()>;
    fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut Option<ID3D10Query>) -> windows_core::Result<()>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut Option<ID3D10Predicate>) -> windows_core::Result<()>;
    fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut Option<ID3D10Counter>) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D10_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::Result<()>;
    fn GetCreationFlags(&self) -> u32;
    fn OpenSharedResource(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetTextFilterSize(&self, width: u32, height: u32);
    fn GetTextFilterSize(&self, pwidth: *mut u32, pheight: *mut u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Device_Vtbl
    where
        Identity: ID3D10Device_Impl,
    {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSSetShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelshader: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSSetShader(this, windows_core::from_raw_borrowed(&ppixelshader))
        }
        unsafe extern "system" fn PSSetSamplers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSSetShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexshader: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSSetShader(this, windows_core::from_raw_borrowed(&pvertexshader))
        }
        unsafe extern "system" fn DrawIndexed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::DrawIndexed(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation))
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcount: u32, startvertexlocation: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::Draw(this, core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&startvertexlocation))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IASetInputLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputlayout: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IASetInputLayout(this, windows_core::from_raw_borrowed(&pinputlayout))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut core::ffi::c_void, pstrides: *const u32, poffsets: *const u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IASetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IASetIndexBuffer(this, windows_core::from_raw_borrowed(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::DrawIndexedInstanced(this, core::mem::transmute_copy(&indexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation), core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawInstanced<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::DrawInstanced(this, core::mem::transmute_copy(&vertexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startvertexlocation), core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSSetShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSSetShader(this, windows_core::from_raw_borrowed(&pshader))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IASetPrimitiveTopology(this, core::mem::transmute_copy(&topology))
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSSetSamplers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn SetPredication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicate: *mut core::ffi::c_void, predicatevalue: super::super::Foundation::BOOL)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SetPredication(this, windows_core::from_raw_borrowed(&ppredicate), core::mem::transmute_copy(&predicatevalue))
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSSetSamplers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OMSetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), windows_core::from_raw_borrowed(&pdepthstencilview))
        }
        unsafe extern "system" fn OMSetBlendState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstate: *mut core::ffi::c_void, blendfactor: *const f32, samplemask: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OMSetBlendState(this, windows_core::from_raw_borrowed(&pblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&samplemask))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilstate: *mut core::ffi::c_void, stencilref: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OMSetDepthStencilState(this, windows_core::from_raw_borrowed(&pdepthstencilstate), core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SOSetTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut core::ffi::c_void, poffsets: *const u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SOSetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn DrawAuto<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::DrawAuto(this)
        }
        unsafe extern "system" fn RSSetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerstate: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::RSSetState(this, windows_core::from_raw_borrowed(&prasterizerstate))
        }
        unsafe extern "system" fn RSSetViewports<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: u32, pviewports: *const D3D10_VIEWPORT)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::RSSetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::RSSetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D10_BOX)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CopySubresourceRegion(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, psrcresource: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CopyResource(this, windows_core::from_raw_borrowed(&pdstresource), windows_core::from_raw_borrowed(&psrcresource))
        }
        unsafe extern "system" fn UpdateSubresource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D10_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::UpdateSubresource(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertargetview: *mut core::ffi::c_void, colorrgba: *const f32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::ClearRenderTargetView(this, windows_core::from_raw_borrowed(&prendertargetview), core::mem::transmute_copy(&colorrgba))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::ClearDepthStencilView(this, windows_core::from_raw_borrowed(&pdepthstencilview), core::mem::transmute_copy(&clearflags), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&stencil))
        }
        unsafe extern "system" fn GenerateMips<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderresourceview: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GenerateMips(this, windows_core::from_raw_borrowed(&pshaderresourceview))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::ResolveSubresource(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSGetShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSGetShader(this, core::mem::transmute_copy(&pppixelshader))
        }
        unsafe extern "system" fn PSGetSamplers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSGetShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSGetShader(this, core::mem::transmute_copy(&ppvertexshader))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::PSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinputlayout: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IAGetInputLayout(this, core::mem::transmute_copy(&ppinputlayout))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IAGetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut *mut core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IAGetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSGetShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSGetShader(this, core::mem::transmute_copy(&ppgeometryshader))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::IAGetPrimitiveTopology(this, core::mem::transmute_copy(&ptopology))
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSGetSamplers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::VSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn GetPredication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppredicate: *mut *mut core::ffi::c_void, ppredicatevalue: *mut super::super::Foundation::BOOL)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GetPredication(this, core::mem::transmute_copy(&pppredicate), core::mem::transmute_copy(&ppredicatevalue))
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSGetSamplers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OMGetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview))
        }
        unsafe extern "system" fn OMGetBlendState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblendstate: *mut *mut core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OMGetBlendState(this, core::mem::transmute_copy(&ppblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&psamplemask))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdepthstencilstate: *mut *mut core::ffi::c_void, pstencilref: *mut u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OMGetDepthStencilState(this, core::mem::transmute_copy(&ppdepthstencilstate), core::mem::transmute_copy(&pstencilref))
        }
        unsafe extern "system" fn SOGetTargets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut core::ffi::c_void, poffsets: *mut u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SOGetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn RSGetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprasterizerstate: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::RSGetState(this, core::mem::transmute_copy(&pprasterizerstate))
        }
        unsafe extern "system" fn RSGetViewports<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: *mut u32, pviewports: *mut D3D10_VIEWPORT)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::RSGetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: *mut u32, prects: *mut super::super::Foundation::RECT)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::RSGetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GetDeviceRemovedReason(this).into()
        }
        unsafe extern "system" fn SetExceptionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, raiseflags: u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags)).into()
        }
        unsafe extern "system" fn GetExceptionMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GetExceptionMode(this)
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn ClearState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::ClearState(this)
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::Flush(this)
        }
        unsafe extern "system" fn CreateBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateBuffer(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn CreateTexture1D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture1d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Device_Impl::CreateTexture1D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                Ok(ok__) => {
                    pptexture1d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture2D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Device_Impl::CreateTexture2D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                Ok(ok__) => {
                    pptexture2d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTexture3D<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: *const D3D10_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Device_Impl::CreateTexture3D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata)) {
                Ok(ok__) => {
                    pptexture3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateShaderResourceView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateRenderTargetView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprtview)).into()
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateDepthStencilView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppdepthstencilview)).into()
        }
        unsafe extern "system" fn CreateInputLayout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputelementdescs: *const D3D10_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateInputLayout(this, core::mem::transmute_copy(&pinputelementdescs), core::mem::transmute_copy(&numelements), core::mem::transmute_copy(&pshaderbytecodewithinputsignature), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppinputlayout)).into()
        }
        unsafe extern "system" fn CreateVertexShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppvertexshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppvertexshader)).into()
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateGeometryShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppgeometryshader)).into()
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D10_SO_DECLARATION_ENTRY, numentries: u32, outputstreamstride: u32, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateGeometryShaderWithStreamOutput(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&psodeclaration), core::mem::transmute_copy(&numentries), core::mem::transmute_copy(&outputstreamstride), core::mem::transmute_copy(&ppgeometryshader)).into()
        }
        unsafe extern "system" fn CreatePixelShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pppixelshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&pppixelshader)).into()
        }
        unsafe extern "system" fn CreateBlendState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateBlendState(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateDepthStencilState(this, core::mem::transmute_copy(&pdepthstencildesc), core::mem::transmute_copy(&ppdepthstencilstate)).into()
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateRasterizerState(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
        }
        unsafe extern "system" fn CreateSamplerState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateSamplerState(this, core::mem::transmute_copy(&psamplerdesc), core::mem::transmute_copy(&ppsamplerstate)).into()
        }
        unsafe extern "system" fn CreateQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc: *const D3D10_QUERY_DESC, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateQuery(this, core::mem::transmute_copy(&pquerydesc), core::mem::transmute_copy(&ppquery)).into()
        }
        unsafe extern "system" fn CreatePredicate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreatePredicate(this, core::mem::transmute_copy(&ppredicatedesc), core::mem::transmute_copy(&pppredicate)).into()
        }
        unsafe extern "system" fn CreateCounter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CreateCounter(this, core::mem::transmute_copy(&pcounterdesc), core::mem::transmute_copy(&ppcounter)).into()
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Device_Impl::CheckFormatSupport(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    pformatsupport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Device_Impl::CheckMultisampleQualityLevels(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount)) {
                Ok(ok__) => {
                    pnumqualitylevels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterinfo: *mut D3D10_COUNTER_INFO)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo))
        }
        unsafe extern "system" fn CheckCounter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::CheckCounter(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pactivecounters), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pnamelength), core::mem::transmute_copy(&szunits), core::mem::transmute_copy(&punitslength), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pdescriptionlength)).into()
        }
        unsafe extern "system" fn GetCreationFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GetCreationFlags(this)
        }
        unsafe extern "system" fn OpenSharedResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::OpenSharedResource(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn SetTextFilterSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::SetTextFilterSize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height))
        }
        unsafe extern "system" fn GetTextFilterSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32)
        where
            Identity: ID3D10Device_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device_Impl::GetTextFilterSize(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, OFFSET>,
            PSSetShader: PSSetShader::<Identity, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, OFFSET>,
            VSSetShader: VSSetShader::<Identity, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, OFFSET>,
            GSSetShader: GSSetShader::<Identity, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, OFFSET>,
            SetPredication: SetPredication::<Identity, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, OFFSET>,
            DrawAuto: DrawAuto::<Identity, OFFSET>,
            RSSetState: RSSetState::<Identity, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, OFFSET>,
            CopyResource: CopyResource::<Identity, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, OFFSET>,
            GenerateMips: GenerateMips::<Identity, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, OFFSET>,
            PSGetShader: PSGetShader::<Identity, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, OFFSET>,
            VSGetShader: VSGetShader::<Identity, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, OFFSET>,
            GSGetShader: GSGetShader::<Identity, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, OFFSET>,
            GetPredication: GetPredication::<Identity, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, OFFSET>,
            RSGetState: RSGetState::<Identity, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            ClearState: ClearState::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            CreateBuffer: CreateBuffer::<Identity, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, OFFSET>,
            CreateCounter: CreateCounter::<Identity, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, OFFSET>,
            CheckCounter: CheckCounter::<Identity, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, OFFSET>,
            SetTextFilterSize: SetTextFilterSize::<Identity, OFFSET>,
            GetTextFilterSize: GetTextFilterSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10Device1_Impl: Sized + ID3D10Device_Impl {
    fn CreateShaderResourceView1(&self, presource: Option<&ID3D10Resource>, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut Option<ID3D10ShaderResourceView1>) -> windows_core::Result<()>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut Option<ID3D10BlendState1>) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> D3D10_FEATURE_LEVEL1;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10Device1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10Device1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Device1_Vtbl
    where
        Identity: ID3D10Device1_Impl,
    {
        unsafe extern "system" fn CreateShaderResourceView1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device1_Impl::CreateShaderResourceView1(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
        }
        unsafe extern "system" fn CreateBlendState1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D10_BLEND_DESC1, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device1_Impl::CreateBlendState1(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D10_FEATURE_LEVEL1
        where
            Identity: ID3D10Device1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Device1_Impl::GetFeatureLevel(this)
        }
        Self {
            base__: ID3D10Device_Vtbl::new::<Identity, OFFSET>(),
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Device1 as windows_core::Interface>::IID || iid == &<ID3D10Device as windows_core::Interface>::IID
    }
}
pub trait ID3D10DeviceChild_Impl: Sized {
    fn GetDevice(&self, ppdevice: *mut Option<ID3D10Device>);
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D10DeviceChild {}
impl ID3D10DeviceChild_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10DeviceChild_Vtbl
    where
        Identity: ID3D10DeviceChild_Impl,
    {
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10DeviceChild_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice))
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10DeviceChild_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10DeviceChild_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10DeviceChild_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10DeviceChild_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10DeviceChild_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10DeviceChild_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10Effect_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn IsPool(&self) -> super::super::Foundation::BOOL;
    fn GetDevice(&self) -> windows_core::Result<ID3D10Device>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10EffectConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetVariableBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetTechniqueByIndex(&self, index: u32) -> Option<ID3D10EffectTechnique>;
    fn GetTechniqueByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectTechnique>;
    fn Optimize(&self) -> windows_core::Result<()>;
    fn IsOptimized(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D10Effect {}
impl ID3D10Effect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Effect_Vtbl
    where
        Identity: ID3D10Effect_Impl,
    {
        unsafe extern "system" fn IsValid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::IsValid(this)
        }
        unsafe extern "system" fn IsPool<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::IsPool(this)
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Effect_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectConstantBuffer>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectConstantBuffer>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetVariableByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetVariableBySemantic<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetVariableBySemantic(this, core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetTechniqueByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectTechnique>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetTechniqueByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetTechniqueByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectTechnique>
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::GetTechniqueByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn Optimize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::Optimize(this).into()
        }
        unsafe extern "system" fn IsOptimized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10Effect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Effect_Impl::IsOptimized(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsValid: IsValid::<Identity, OFFSET>,
            IsPool: IsPool::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetVariableByIndex: GetVariableByIndex::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetVariableBySemantic: GetVariableBySemantic::<Identity, OFFSET>,
            GetTechniqueByIndex: GetTechniqueByIndex::<Identity, OFFSET>,
            GetTechniqueByName: GetTechniqueByName::<Identity, OFFSET>,
            Optimize: Optimize::<Identity, OFFSET>,
            IsOptimized: IsOptimized::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Effect as windows_core::Interface>::IID
    }
}
pub trait ID3D10EffectBlendVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetBlendState(&self, index: u32) -> windows_core::Result<ID3D10BlendState>;
    fn GetBackingStore(&self, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectBlendVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectBlendVariable_Impl>() -> ID3D10EffectBlendVariable_Vtbl {
        unsafe extern "system" fn GetBlendState<Impl: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectBlendVariable_Impl::GetBlendState(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppblendstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectBlendVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pblenddesc: *mut D3D10_BLEND_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectBlendVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pblenddesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetBlendState: GetBlendState::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10EffectBlendVariable_ImplVtbl<T: ID3D10EffectBlendVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectBlendVariable_Impl> ID3D10EffectBlendVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectBlendVariable_Vtbl = ID3D10EffectBlendVariable_Vtbl::new::<T>();
}
impl ID3D10EffectBlendVariable {
    pub fn new<'a, T: ID3D10EffectBlendVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectBlendVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectConstantBuffer_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetConstantBuffer(&self, pconstantbuffer: Option<&ID3D10Buffer>) -> windows_core::Result<()>;
    fn GetConstantBuffer(&self) -> windows_core::Result<ID3D10Buffer>;
    fn SetTextureBuffer(&self, ptexturebuffer: Option<&ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn GetTextureBuffer(&self) -> windows_core::Result<ID3D10ShaderResourceView>;
}
impl ID3D10EffectConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D10EffectConstantBuffer_Impl>() -> ID3D10EffectConstantBuffer_Vtbl {
        unsafe extern "system" fn SetConstantBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pconstantbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectConstantBuffer_Impl::SetConstantBuffer(this, windows_core::from_raw_borrowed(&pconstantbuffer)).into()
        }
        unsafe extern "system" fn GetConstantBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, ppconstantbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectConstantBuffer_Impl::GetConstantBuffer(this) {
                Ok(ok__) => {
                    ppconstantbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextureBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, ptexturebuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectConstantBuffer_Impl::SetTextureBuffer(this, windows_core::from_raw_borrowed(&ptexturebuffer)).into()
        }
        unsafe extern "system" fn GetTextureBuffer<Impl: ID3D10EffectConstantBuffer_Impl>(this: *mut core::ffi::c_void, pptexturebuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectConstantBuffer_Impl::GetTextureBuffer(this) {
                Ok(ok__) => {
                    pptexturebuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetConstantBuffer: SetConstantBuffer::<Impl>,
            GetConstantBuffer: GetConstantBuffer::<Impl>,
            SetTextureBuffer: SetTextureBuffer::<Impl>,
            GetTextureBuffer: GetTextureBuffer::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectConstantBuffer_ImplVtbl<T: ID3D10EffectConstantBuffer_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectConstantBuffer_Impl> ID3D10EffectConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10EffectConstantBuffer_Vtbl = ID3D10EffectConstantBuffer_Vtbl::new::<T>();
}
impl ID3D10EffectConstantBuffer {
    pub fn new<'a, T: ID3D10EffectConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectDepthStencilVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetDepthStencilState(&self, index: u32) -> windows_core::Result<ID3D10DepthStencilState>;
    fn GetBackingStore(&self, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectDepthStencilVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectDepthStencilVariable_Impl>() -> ID3D10EffectDepthStencilVariable_Vtbl {
        unsafe extern "system" fn GetDepthStencilState<Impl: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectDepthStencilVariable_Impl::GetDepthStencilState(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppdepthstencilstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectDepthStencilVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pdepthstencildesc: *mut D3D10_DEPTH_STENCIL_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectDepthStencilVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pdepthstencildesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetDepthStencilState: GetDepthStencilState::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10EffectDepthStencilVariable_ImplVtbl<T: ID3D10EffectDepthStencilVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectDepthStencilVariable_Impl> ID3D10EffectDepthStencilVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilVariable_Vtbl = ID3D10EffectDepthStencilVariable_Vtbl::new::<T>();
}
impl ID3D10EffectDepthStencilVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectDepthStencilViewVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetDepthStencil(&self, presource: Option<&ID3D10DepthStencilView>) -> windows_core::Result<()>;
    fn GetDepthStencil(&self) -> windows_core::Result<ID3D10DepthStencilView>;
    fn SetDepthStencilArray(&self, ppresources: *const Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetDepthStencilArray(&self, ppresources: *mut Option<ID3D10DepthStencilView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectDepthStencilViewVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectDepthStencilViewVariable_Impl>() -> ID3D10EffectDepthStencilViewVariable_Vtbl {
        unsafe extern "system" fn SetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectDepthStencilViewVariable_Impl::SetDepthStencil(this, windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn GetDepthStencil<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectDepthStencilViewVariable_Impl::GetDepthStencil(this) {
                Ok(ok__) => {
                    ppresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectDepthStencilViewVariable_Impl::SetDepthStencilArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetDepthStencilArray<Impl: ID3D10EffectDepthStencilViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectDepthStencilViewVariable_Impl::GetDepthStencilArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetDepthStencil: SetDepthStencil::<Impl>,
            GetDepthStencil: GetDepthStencil::<Impl>,
            SetDepthStencilArray: SetDepthStencilArray::<Impl>,
            GetDepthStencilArray: GetDepthStencilArray::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectDepthStencilViewVariable_ImplVtbl<T: ID3D10EffectDepthStencilViewVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectDepthStencilViewVariable_Impl> ID3D10EffectDepthStencilViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectDepthStencilViewVariable_Vtbl = ID3D10EffectDepthStencilViewVariable_Vtbl::new::<T>();
}
impl ID3D10EffectDepthStencilViewVariable {
    pub fn new<'a, T: ID3D10EffectDepthStencilViewVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectDepthStencilViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectMatrixVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn GetMatrix(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn SetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetMatrixArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn GetMatrixTranspose(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn SetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetMatrixTransposeArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectMatrixVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectMatrixVariable_Impl>() -> ID3D10EffectMatrixVariable_Vtbl {
        unsafe extern "system" fn SetMatrix<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::SetMatrix(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrix<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::GetMatrix(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::SetMatrixArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::GetMatrixArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetMatrixTranspose<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::SetMatrixTranspose(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetMatrixTranspose<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::GetMatrixTranspose(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::SetMatrixTransposeArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetMatrixTransposeArray<Impl: ID3D10EffectMatrixVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectMatrixVariable_Impl::GetMatrixTransposeArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetMatrix: SetMatrix::<Impl>,
            GetMatrix: GetMatrix::<Impl>,
            SetMatrixArray: SetMatrixArray::<Impl>,
            GetMatrixArray: GetMatrixArray::<Impl>,
            SetMatrixTranspose: SetMatrixTranspose::<Impl>,
            GetMatrixTranspose: GetMatrixTranspose::<Impl>,
            SetMatrixTransposeArray: SetMatrixTransposeArray::<Impl>,
            GetMatrixTransposeArray: GetMatrixTransposeArray::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectMatrixVariable_ImplVtbl<T: ID3D10EffectMatrixVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectMatrixVariable_Impl> ID3D10EffectMatrixVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectMatrixVariable_Vtbl = ID3D10EffectMatrixVariable_Vtbl::new::<T>();
}
impl ID3D10EffectMatrixVariable {
    pub fn new<'a, T: ID3D10EffectMatrixVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectMatrixVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectPass_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_PASS_DESC) -> windows_core::Result<()>;
    fn GetVertexShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()>;
    fn GetGeometryShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()>;
    fn GetPixelShaderDesc(&self, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn Apply(&self, flags: u32) -> windows_core::Result<()>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()>;
}
impl ID3D10EffectPass_Vtbl {
    pub const fn new<Impl: ID3D10EffectPass_Impl>() -> ID3D10EffectPass_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::IsValid(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVertexShaderDesc<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::GetVertexShaderDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetGeometryShaderDesc<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::GetGeometryShaderDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetPixelShaderDesc<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_PASS_SHADER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::GetPixelShaderDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn Apply<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, flags: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::Apply(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectPass_Impl>(this: *mut core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectPass_Impl::ComputeStateBlockMask(this, core::mem::transmute_copy(&pstateblockmask)).into()
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetVertexShaderDesc: GetVertexShaderDesc::<Impl>,
            GetGeometryShaderDesc: GetGeometryShaderDesc::<Impl>,
            GetPixelShaderDesc: GetPixelShaderDesc::<Impl>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl>,
            GetAnnotationByName: GetAnnotationByName::<Impl>,
            Apply: Apply::<Impl>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectPass_ImplVtbl<T: ID3D10EffectPass_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectPass_Impl> ID3D10EffectPass_ImplVtbl<T> {
    const VTABLE: ID3D10EffectPass_Vtbl = ID3D10EffectPass_Vtbl::new::<T>();
}
impl ID3D10EffectPass {
    pub fn new<'a, T: ID3D10EffectPass_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectPass_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectPool_Impl: Sized {
    fn AsEffect(&self) -> Option<ID3D10Effect>;
}
impl windows_core::RuntimeName for ID3D10EffectPool {}
impl ID3D10EffectPool_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10EffectPool_Vtbl
    where
        Identity: ID3D10EffectPool_Impl,
    {
        unsafe extern "system" fn AsEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<ID3D10Effect>
        where
            Identity: ID3D10EffectPool_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10EffectPool_Impl::AsEffect(this)
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsEffect: AsEffect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10EffectPool as windows_core::Interface>::IID
    }
}
pub trait ID3D10EffectRasterizerVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetRasterizerState(&self, index: u32) -> windows_core::Result<ID3D10RasterizerState>;
    fn GetBackingStore(&self, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectRasterizerVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectRasterizerVariable_Impl>() -> ID3D10EffectRasterizerVariable_Vtbl {
        unsafe extern "system" fn GetRasterizerState<Impl: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectRasterizerVariable_Impl::GetRasterizerState(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pprasterizerstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectRasterizerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, prasterizerdesc: *mut D3D10_RASTERIZER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectRasterizerVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&prasterizerdesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetRasterizerState: GetRasterizerState::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10EffectRasterizerVariable_ImplVtbl<T: ID3D10EffectRasterizerVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectRasterizerVariable_Impl> ID3D10EffectRasterizerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRasterizerVariable_Vtbl = ID3D10EffectRasterizerVariable_Vtbl::new::<T>();
}
impl ID3D10EffectRasterizerVariable {
    pub fn new<'a, T: ID3D10EffectRasterizerVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectRasterizerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectRenderTargetViewVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetRenderTarget(&self, presource: Option<&ID3D10RenderTargetView>) -> windows_core::Result<()>;
    fn GetRenderTarget(&self) -> windows_core::Result<ID3D10RenderTargetView>;
    fn SetRenderTargetArray(&self, ppresources: *const Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetRenderTargetArray(&self, ppresources: *mut Option<ID3D10RenderTargetView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectRenderTargetViewVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectRenderTargetViewVariable_Impl>() -> ID3D10EffectRenderTargetViewVariable_Vtbl {
        unsafe extern "system" fn SetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectRenderTargetViewVariable_Impl::SetRenderTarget(this, windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn GetRenderTarget<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectRenderTargetViewVariable_Impl::GetRenderTarget(this) {
                Ok(ok__) => {
                    ppresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectRenderTargetViewVariable_Impl::SetRenderTargetArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetRenderTargetArray<Impl: ID3D10EffectRenderTargetViewVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectRenderTargetViewVariable_Impl::GetRenderTargetArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetRenderTarget: SetRenderTarget::<Impl>,
            GetRenderTarget: GetRenderTarget::<Impl>,
            SetRenderTargetArray: SetRenderTargetArray::<Impl>,
            GetRenderTargetArray: GetRenderTargetArray::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectRenderTargetViewVariable_ImplVtbl<T: ID3D10EffectRenderTargetViewVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectRenderTargetViewVariable_Impl> ID3D10EffectRenderTargetViewVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectRenderTargetViewVariable_Vtbl = ID3D10EffectRenderTargetViewVariable_Vtbl::new::<T>();
}
impl ID3D10EffectRenderTargetViewVariable {
    pub fn new<'a, T: ID3D10EffectRenderTargetViewVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectRenderTargetViewVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectSamplerVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetSampler(&self, index: u32) -> windows_core::Result<ID3D10SamplerState>;
    fn GetBackingStore(&self, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> windows_core::Result<()>;
}
impl ID3D10EffectSamplerVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectSamplerVariable_Impl>() -> ID3D10EffectSamplerVariable_Vtbl {
        unsafe extern "system" fn GetSampler<Impl: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, ppsampler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectSamplerVariable_Impl::GetSampler(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppsampler.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackingStore<Impl: ID3D10EffectSamplerVariable_Impl>(this: *mut core::ffi::c_void, index: u32, psamplerdesc: *mut D3D10_SAMPLER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectSamplerVariable_Impl::GetBackingStore(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&psamplerdesc)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetSampler: GetSampler::<Impl>, GetBackingStore: GetBackingStore::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10EffectSamplerVariable_ImplVtbl<T: ID3D10EffectSamplerVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectSamplerVariable_Impl> ID3D10EffectSamplerVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectSamplerVariable_Vtbl = ID3D10EffectSamplerVariable_Vtbl::new::<T>();
}
impl ID3D10EffectSamplerVariable {
    pub fn new<'a, T: ID3D10EffectSamplerVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectSamplerVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectScalarVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetFloat(&self, value: f32) -> windows_core::Result<()>;
    fn GetFloat(&self) -> windows_core::Result<f32>;
    fn SetFloatArray(&self, pdata: *const f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetFloatArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetInt(&self, value: i32) -> windows_core::Result<()>;
    fn GetInt(&self) -> windows_core::Result<i32>;
    fn SetIntArray(&self, pdata: *const i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetIntArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetBool(&self, value: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBool(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBoolArray(&self, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetBoolArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectScalarVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectScalarVariable_Impl>() -> ID3D10EffectScalarVariable_Vtbl {
        unsafe extern "system" fn SetFloat<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::SetFloat(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetFloat<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectScalarVariable_Impl::GetFloat(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFloatArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::SetFloatArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::GetFloatArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetInt<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::SetInt(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetInt<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectScalarVariable_Impl::GetInt(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIntArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const i32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::SetIntArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::GetIntArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetBool<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, value: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::SetBool(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetBool<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pvalue: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectScalarVariable_Impl::GetBool(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBoolArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::SetBoolArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolArray<Impl: ID3D10EffectScalarVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectScalarVariable_Impl::GetBoolArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetFloat: SetFloat::<Impl>,
            GetFloat: GetFloat::<Impl>,
            SetFloatArray: SetFloatArray::<Impl>,
            GetFloatArray: GetFloatArray::<Impl>,
            SetInt: SetInt::<Impl>,
            GetInt: GetInt::<Impl>,
            SetIntArray: SetIntArray::<Impl>,
            GetIntArray: GetIntArray::<Impl>,
            SetBool: SetBool::<Impl>,
            GetBool: GetBool::<Impl>,
            SetBoolArray: SetBoolArray::<Impl>,
            GetBoolArray: GetBoolArray::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectScalarVariable_ImplVtbl<T: ID3D10EffectScalarVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectScalarVariable_Impl> ID3D10EffectScalarVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectScalarVariable_Vtbl = ID3D10EffectScalarVariable_Vtbl::new::<T>();
}
impl ID3D10EffectScalarVariable {
    pub fn new<'a, T: ID3D10EffectScalarVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectScalarVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectShaderResourceVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetResource(&self, presource: Option<&ID3D10ShaderResourceView>) -> windows_core::Result<()>;
    fn GetResource(&self) -> windows_core::Result<ID3D10ShaderResourceView>;
    fn SetResourceArray(&self, ppresources: *const Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetResourceArray(&self, ppresources: *mut Option<ID3D10ShaderResourceView>, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectShaderResourceVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectShaderResourceVariable_Impl>() -> ID3D10EffectShaderResourceVariable_Vtbl {
        unsafe extern "system" fn SetResource<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectShaderResourceVariable_Impl::SetResource(this, windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn GetResource<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectShaderResourceVariable_Impl::GetResource(this) {
                Ok(ok__) => {
                    ppresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceArray<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *const *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectShaderResourceVariable_Impl::SetResourceArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetResourceArray<Impl: ID3D10EffectShaderResourceVariable_Impl>(this: *mut core::ffi::c_void, ppresources: *mut *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectShaderResourceVariable_Impl::GetResourceArray(this, core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetResource: SetResource::<Impl>,
            GetResource: GetResource::<Impl>,
            SetResourceArray: SetResourceArray::<Impl>,
            GetResourceArray: GetResourceArray::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectShaderResourceVariable_ImplVtbl<T: ID3D10EffectShaderResourceVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectShaderResourceVariable_Impl> ID3D10EffectShaderResourceVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderResourceVariable_Vtbl = ID3D10EffectShaderResourceVariable_Vtbl::new::<T>();
}
impl ID3D10EffectShaderResourceVariable {
    pub fn new<'a, T: ID3D10EffectShaderResourceVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectShaderResourceVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10EffectShaderVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetShaderDesc(&self, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::Result<()>;
    fn GetVertexShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10VertexShader>;
    fn GetGeometryShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10GeometryShader>;
    fn GetPixelShader(&self, shaderindex: u32) -> windows_core::Result<ID3D10PixelShader>;
    fn GetInputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputSignatureElementDesc(&self, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectShaderVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectShaderVariable_Impl>() -> ID3D10EffectShaderVariable_Vtbl {
        unsafe extern "system" fn GetShaderDesc<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, pdesc: *mut D3D10_EFFECT_SHADER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectShaderVariable_Impl::GetShaderDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVertexShader<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppvs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectShaderVariable_Impl::GetVertexShader(this, core::mem::transmute_copy(&shaderindex)) {
                Ok(ok__) => {
                    ppvs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeometryShader<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppgs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectShaderVariable_Impl::GetGeometryShader(this, core::mem::transmute_copy(&shaderindex)) {
                Ok(ok__) => {
                    ppgs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPixelShader<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, ppps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectShaderVariable_Impl::GetPixelShader(this, core::mem::transmute_copy(&shaderindex)) {
                Ok(ok__) => {
                    ppps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSignatureElementDesc<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectShaderVariable_Impl::GetInputSignatureElementDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&element), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetOutputSignatureElementDesc<Impl: ID3D10EffectShaderVariable_Impl>(this: *mut core::ffi::c_void, shaderindex: u32, element: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectShaderVariable_Impl::GetOutputSignatureElementDesc(this, core::mem::transmute_copy(&shaderindex), core::mem::transmute_copy(&element), core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            GetShaderDesc: GetShaderDesc::<Impl>,
            GetVertexShader: GetVertexShader::<Impl>,
            GetGeometryShader: GetGeometryShader::<Impl>,
            GetPixelShader: GetPixelShader::<Impl>,
            GetInputSignatureElementDesc: GetInputSignatureElementDesc::<Impl>,
            GetOutputSignatureElementDesc: GetOutputSignatureElementDesc::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10EffectShaderVariable_ImplVtbl<T: ID3D10EffectShaderVariable_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10EffectShaderVariable_Impl> ID3D10EffectShaderVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectShaderVariable_Vtbl = ID3D10EffectShaderVariable_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectShaderVariable {
    pub fn new<'a, T: ID3D10EffectShaderVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectShaderVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectStringVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn GetString(&self) -> windows_core::Result<windows_core::PCSTR>;
    fn GetStringArray(&self, ppstrings: *mut windows_core::PCSTR, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectStringVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectStringVariable_Impl>() -> ID3D10EffectStringVariable_Vtbl {
        unsafe extern "system" fn GetString<Impl: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, ppstring: *mut windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match ID3D10EffectStringVariable_Impl::GetString(this) {
                Ok(ok__) => {
                    ppstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringArray<Impl: ID3D10EffectStringVariable_Impl>(this: *mut core::ffi::c_void, ppstrings: *mut windows_core::PCSTR, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectStringVariable_Impl::GetStringArray(this, core::mem::transmute_copy(&ppstrings), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self { base__: ID3D10EffectVariable_Vtbl::new::<Impl>(), GetString: GetString::<Impl>, GetStringArray: GetStringArray::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10EffectStringVariable_ImplVtbl<T: ID3D10EffectStringVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectStringVariable_Impl> ID3D10EffectStringVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectStringVariable_Vtbl = ID3D10EffectStringVariable_Vtbl::new::<T>();
}
impl ID3D10EffectStringVariable {
    pub fn new<'a, T: ID3D10EffectStringVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectStringVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectTechnique_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetPassByIndex(&self, index: u32) -> Option<ID3D10EffectPass>;
    fn GetPassByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectPass>;
    fn ComputeStateBlockMask(&self, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::Result<()>;
}
impl ID3D10EffectTechnique_Vtbl {
    pub const fn new<Impl: ID3D10EffectTechnique_Impl>() -> ID3D10EffectTechnique_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::IsValid(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TECHNIQUE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetPassByIndex<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectPass> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::GetPassByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetPassByName<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectPass> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::GetPassByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn ComputeStateBlockMask<Impl: ID3D10EffectTechnique_Impl>(this: *mut core::ffi::c_void, pstateblockmask: *mut D3D10_STATE_BLOCK_MASK) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectTechnique_Impl::ComputeStateBlockMask(this, core::mem::transmute_copy(&pstateblockmask)).into()
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl>,
            GetAnnotationByName: GetAnnotationByName::<Impl>,
            GetPassByIndex: GetPassByIndex::<Impl>,
            GetPassByName: GetPassByName::<Impl>,
            ComputeStateBlockMask: ComputeStateBlockMask::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectTechnique_ImplVtbl<T: ID3D10EffectTechnique_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectTechnique_Impl> ID3D10EffectTechnique_ImplVtbl<T> {
    const VTABLE: ID3D10EffectTechnique_Vtbl = ID3D10EffectTechnique_Vtbl::new::<T>();
}
impl ID3D10EffectTechnique {
    pub fn new<'a, T: ID3D10EffectTechnique_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectTechnique_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10EffectType_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10EffectType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectType>;
    fn GetMemberTypeBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectType>;
    fn GetMemberName(&self, index: u32) -> windows_core::PCSTR;
    fn GetMemberSemantic(&self, index: u32) -> windows_core::PCSTR;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectType_Vtbl {
    pub const fn new<Impl: ID3D10EffectType_Impl>() -> ID3D10EffectType_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::IsValid(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_TYPE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeBySemantic<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::GetMemberTypeBySemantic(this, core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetMemberName<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::GetMemberName(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberSemantic<Impl: ID3D10EffectType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectType_Impl::GetMemberSemantic(this, core::mem::transmute_copy(&index))
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeBySemantic: GetMemberTypeBySemantic::<Impl>,
            GetMemberName: GetMemberName::<Impl>,
            GetMemberSemantic: GetMemberSemantic::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10EffectType_ImplVtbl<T: ID3D10EffectType_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10EffectType_Impl> ID3D10EffectType_ImplVtbl<T> {
    const VTABLE: ID3D10EffectType_Vtbl = ID3D10EffectType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10EffectType {
    pub fn new<'a, T: ID3D10EffectType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectVariable_Impl: Sized {
    fn IsValid(&self) -> super::super::Foundation::BOOL;
    fn GetType(&self) -> Option<ID3D10EffectType>;
    fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetAnnotationByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetAnnotationByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberByIndex(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetMemberByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetMemberBySemantic(&self, semantic: &windows_core::PCSTR) -> Option<ID3D10EffectVariable>;
    fn GetElement(&self, index: u32) -> Option<ID3D10EffectVariable>;
    fn GetParentConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsScalar(&self) -> Option<ID3D10EffectScalarVariable>;
    fn AsVector(&self) -> Option<ID3D10EffectVectorVariable>;
    fn AsMatrix(&self) -> Option<ID3D10EffectMatrixVariable>;
    fn AsString(&self) -> Option<ID3D10EffectStringVariable>;
    fn AsShaderResource(&self) -> Option<ID3D10EffectShaderResourceVariable>;
    fn AsRenderTargetView(&self) -> Option<ID3D10EffectRenderTargetViewVariable>;
    fn AsDepthStencilView(&self) -> Option<ID3D10EffectDepthStencilViewVariable>;
    fn AsConstantBuffer(&self) -> Option<ID3D10EffectConstantBuffer>;
    fn AsShader(&self) -> Option<ID3D10EffectShaderVariable>;
    fn AsBlend(&self) -> Option<ID3D10EffectBlendVariable>;
    fn AsDepthStencil(&self) -> Option<ID3D10EffectDepthStencilVariable>;
    fn AsRasterizer(&self) -> Option<ID3D10EffectRasterizerVariable>;
    fn AsSampler(&self) -> Option<ID3D10EffectSamplerVariable>;
    fn SetRawValue(&self, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
    fn GetRawValue(&self, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectVariable_Impl>() -> ID3D10EffectVariable_Vtbl {
        unsafe extern "system" fn IsValid<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::IsValid(this)
        }
        unsafe extern "system" fn GetType<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetType(this)
        }
        unsafe extern "system" fn GetDesc<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetAnnotationByIndex<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetAnnotationByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetAnnotationByName<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetAnnotationByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberByIndex<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetMemberByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberByName<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetMemberByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberBySemantic<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, semantic: windows_core::PCSTR) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetMemberBySemantic(this, core::mem::transmute(&semantic))
        }
        unsafe extern "system" fn GetElement<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10EffectVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetElement(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetParentConstantBuffer<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetParentConstantBuffer(this)
        }
        unsafe extern "system" fn AsScalar<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectScalarVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsScalar(this)
        }
        unsafe extern "system" fn AsVector<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectVectorVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsVector(this)
        }
        unsafe extern "system" fn AsMatrix<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectMatrixVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsMatrix(this)
        }
        unsafe extern "system" fn AsString<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectStringVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsString(this)
        }
        unsafe extern "system" fn AsShaderResource<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderResourceVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsShaderResource(this)
        }
        unsafe extern "system" fn AsRenderTargetView<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRenderTargetViewVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsRenderTargetView(this)
        }
        unsafe extern "system" fn AsDepthStencilView<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilViewVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsDepthStencilView(this)
        }
        unsafe extern "system" fn AsConstantBuffer<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsConstantBuffer(this)
        }
        unsafe extern "system" fn AsShader<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectShaderVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsShader(this)
        }
        unsafe extern "system" fn AsBlend<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectBlendVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsBlend(this)
        }
        unsafe extern "system" fn AsDepthStencil<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectDepthStencilVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsDepthStencil(this)
        }
        unsafe extern "system" fn AsRasterizer<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectRasterizerVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsRasterizer(this)
        }
        unsafe extern "system" fn AsSampler<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10EffectSamplerVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::AsSampler(this)
        }
        unsafe extern "system" fn SetRawValue<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::SetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
        }
        unsafe extern "system" fn GetRawValue<Impl: ID3D10EffectVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, offset: u32, bytecount: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVariable_Impl::GetRawValue(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&bytecount)).into()
        }
        Self {
            IsValid: IsValid::<Impl>,
            GetType: GetType::<Impl>,
            GetDesc: GetDesc::<Impl>,
            GetAnnotationByIndex: GetAnnotationByIndex::<Impl>,
            GetAnnotationByName: GetAnnotationByName::<Impl>,
            GetMemberByIndex: GetMemberByIndex::<Impl>,
            GetMemberByName: GetMemberByName::<Impl>,
            GetMemberBySemantic: GetMemberBySemantic::<Impl>,
            GetElement: GetElement::<Impl>,
            GetParentConstantBuffer: GetParentConstantBuffer::<Impl>,
            AsScalar: AsScalar::<Impl>,
            AsVector: AsVector::<Impl>,
            AsMatrix: AsMatrix::<Impl>,
            AsString: AsString::<Impl>,
            AsShaderResource: AsShaderResource::<Impl>,
            AsRenderTargetView: AsRenderTargetView::<Impl>,
            AsDepthStencilView: AsDepthStencilView::<Impl>,
            AsConstantBuffer: AsConstantBuffer::<Impl>,
            AsShader: AsShader::<Impl>,
            AsBlend: AsBlend::<Impl>,
            AsDepthStencil: AsDepthStencil::<Impl>,
            AsRasterizer: AsRasterizer::<Impl>,
            AsSampler: AsSampler::<Impl>,
            SetRawValue: SetRawValue::<Impl>,
            GetRawValue: GetRawValue::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectVariable_ImplVtbl<T: ID3D10EffectVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectVariable_Impl> ID3D10EffectVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVariable_Vtbl = ID3D10EffectVariable_Vtbl::new::<T>();
}
impl ID3D10EffectVariable {
    pub fn new<'a, T: ID3D10EffectVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10EffectVectorVariable_Impl: Sized + ID3D10EffectVariable_Impl {
    fn SetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()>;
    fn SetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn GetBoolVector(&self, pdata: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetIntVector(&self, pdata: *mut i32) -> windows_core::Result<()>;
    fn GetFloatVector(&self, pdata: *mut f32) -> windows_core::Result<()>;
    fn SetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn SetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetBoolVectorArray(&self, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetIntVectorArray(&self, pdata: *mut i32, offset: u32, count: u32) -> windows_core::Result<()>;
    fn GetFloatVectorArray(&self, pdata: *mut f32, offset: u32, count: u32) -> windows_core::Result<()>;
}
impl ID3D10EffectVectorVariable_Vtbl {
    pub const fn new<Impl: ID3D10EffectVectorVariable_Impl>() -> ID3D10EffectVectorVariable_Vtbl {
        unsafe extern "system" fn SetBoolVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::SetBoolVector(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetIntVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::SetIntVector(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetFloatVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::SetFloatVector(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetBoolVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::GetBoolVector(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetIntVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::GetIntVector(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetFloatVector<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::GetFloatVector(this, core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetBoolVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::SetBoolVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetIntVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::SetIntVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetFloatVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::SetFloatVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetBoolVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut super::super::Foundation::BOOL, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::GetBoolVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetIntVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut i32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::GetIntVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetFloatVectorArray<Impl: ID3D10EffectVectorVariable_Impl>(this: *mut core::ffi::c_void, pdata: *mut f32, offset: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10EffectVectorVariable_Impl::GetFloatVectorArray(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&offset), core::mem::transmute_copy(&count)).into()
        }
        Self {
            base__: ID3D10EffectVariable_Vtbl::new::<Impl>(),
            SetBoolVector: SetBoolVector::<Impl>,
            SetIntVector: SetIntVector::<Impl>,
            SetFloatVector: SetFloatVector::<Impl>,
            GetBoolVector: GetBoolVector::<Impl>,
            GetIntVector: GetIntVector::<Impl>,
            GetFloatVector: GetFloatVector::<Impl>,
            SetBoolVectorArray: SetBoolVectorArray::<Impl>,
            SetIntVectorArray: SetIntVectorArray::<Impl>,
            SetFloatVectorArray: SetFloatVectorArray::<Impl>,
            GetBoolVectorArray: GetBoolVectorArray::<Impl>,
            GetIntVectorArray: GetIntVectorArray::<Impl>,
            GetFloatVectorArray: GetFloatVectorArray::<Impl>,
        }
    }
}
#[doc(hidden)]
struct ID3D10EffectVectorVariable_ImplVtbl<T: ID3D10EffectVectorVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10EffectVectorVariable_Impl> ID3D10EffectVectorVariable_ImplVtbl<T> {
    const VTABLE: ID3D10EffectVectorVariable_Vtbl = ID3D10EffectVectorVariable_Vtbl::new::<T>();
}
impl ID3D10EffectVectorVariable {
    pub fn new<'a, T: ID3D10EffectVectorVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10EffectVectorVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10GeometryShader_Impl: Sized + ID3D10DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D10GeometryShader {}
impl ID3D10GeometryShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10GeometryShader_Vtbl
    where
        Identity: ID3D10GeometryShader_Impl,
    {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10GeometryShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self);
    fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self) -> u64;
    fn GetNumStoredMessages(&self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64;
    fn GetMessageCountLimit(&self) -> u64;
    fn AddStorageFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self);
    fn PushEmptyStorageFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self);
    fn GetStorageFilterStackSize(&self) -> u32;
    fn AddRetrievalFilterEntries(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self);
    fn PushEmptyRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self);
    fn GetRetrievalFilterStackSize(&self) -> u32;
    fn AddMessage(&self, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: D3D10_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D10InfoQueue {}
impl ID3D10InfoQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10InfoQueue_Vtbl
    where
        Identity: ID3D10InfoQueue_Impl,
    {
        unsafe extern "system" fn SetMessageCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagecountlimit: u64) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::ClearStoredMessages(this)
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, messageindex: u64, pmessage: *mut D3D10_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetMessage(this, core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this)
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this)
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetNumStoredMessages(this)
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this)
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this)
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetMessageCountLimit(this)
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetStorageFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::ClearStorageFilter(this)
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PushEmptyStorageFilter(this).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PushCopyOfStorageFilter(this).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PushStorageFilter(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PopStorageFilter(this)
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetStorageFilterStackSize(this)
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D10_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::ClearRetrievalFilter(this)
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PushEmptyRetrievalFilter(this).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PushCopyOfRetrievalFilter(this).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D10_INFO_QUEUE_FILTER) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::PopRetrievalFilter(this)
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetRetrievalFilterStackSize(this)
        }
        unsafe extern "system" fn AddMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, severity: D3D10_MESSAGE_SEVERITY, id: D3D10_MESSAGE_ID, pdescription: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::AddMessage(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D10_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::SetBreakOnID(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D10_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D10_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D10_MESSAGE_ID) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetBreakOnID(this, core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: super::super::Foundation::BOOL)
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10InfoQueue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10InfoQueue_Impl::GetMuteDebugOutput(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10InfoQueue as windows_core::Interface>::IID
    }
}
pub trait ID3D10InputLayout_Impl: Sized + ID3D10DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D10InputLayout {}
impl ID3D10InputLayout_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10InputLayout_Vtbl
    where
        Identity: ID3D10InputLayout_Impl,
    {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10InputLayout as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10Multithread_Impl: Sized {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D10Multithread {}
impl ID3D10Multithread_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Multithread_Vtbl
    where
        Identity: ID3D10Multithread_Impl,
    {
        unsafe extern "system" fn Enter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Multithread_Impl::Enter(this)
        }
        unsafe extern "system" fn Leave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: ID3D10Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Multithread_Impl::Leave(this)
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Multithread_Impl::SetMultithreadProtected(this, core::mem::transmute_copy(&bmtprotect))
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10Multithread_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Multithread_Impl::GetMultithreadProtected(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, OFFSET>,
            Leave: Leave::<Identity, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Multithread as windows_core::Interface>::IID
    }
}
pub trait ID3D10PixelShader_Impl: Sized + ID3D10DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D10PixelShader {}
impl ID3D10PixelShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10PixelShader_Vtbl
    where
        Identity: ID3D10PixelShader_Impl,
    {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10PixelShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10Predicate_Impl: Sized + ID3D10Query_Impl {}
impl windows_core::RuntimeName for ID3D10Predicate {}
impl ID3D10Predicate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Predicate_Vtbl
    where
        Identity: ID3D10Predicate_Impl,
    {
        Self { base__: ID3D10Query_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Predicate as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID || iid == &<ID3D10Query as windows_core::Interface>::IID
    }
}
pub trait ID3D10Query_Impl: Sized + ID3D10Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_QUERY_DESC);
}
impl windows_core::RuntimeName for ID3D10Query {}
impl ID3D10Query_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Query_Vtbl
    where
        Identity: ID3D10Query_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_QUERY_DESC)
        where
            Identity: ID3D10Query_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Query_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10Asynchronous_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Query as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Asynchronous as windows_core::Interface>::IID
    }
}
pub trait ID3D10RasterizerState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RASTERIZER_DESC);
}
impl windows_core::RuntimeName for ID3D10RasterizerState {}
impl ID3D10RasterizerState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10RasterizerState_Vtbl
    where
        Identity: ID3D10RasterizerState_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_RASTERIZER_DESC)
        where
            Identity: ID3D10RasterizerState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10RasterizerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10RasterizerState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10RenderTargetView_Impl: Sized + ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10RenderTargetView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10RenderTargetView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10RenderTargetView_Vtbl
    where
        Identity: ID3D10RenderTargetView_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_RENDER_TARGET_VIEW_DESC)
        where
            Identity: ID3D10RenderTargetView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10RenderTargetView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10RenderTargetView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
pub trait ID3D10Resource_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetType(&self, rtype: *mut D3D10_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl windows_core::RuntimeName for ID3D10Resource {}
impl ID3D10Resource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Resource_Vtbl
    where
        Identity: ID3D10Resource_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rtype: *mut D3D10_RESOURCE_DIMENSION)
        where
            Identity: ID3D10Resource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Resource_Impl::GetType(this, core::mem::transmute_copy(&rtype))
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32)
        where
            Identity: ID3D10Resource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Resource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority))
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: ID3D10Resource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Resource_Impl::GetEvictionPriority(this)
        }
        Self {
            base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Resource as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10SamplerState_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SAMPLER_DESC);
}
impl windows_core::RuntimeName for ID3D10SamplerState {}
impl ID3D10SamplerState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10SamplerState_Vtbl
    where
        Identity: ID3D10SamplerState_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SAMPLER_DESC)
        where
            Identity: ID3D10SamplerState_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10SamplerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10SamplerState as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D10ShaderReflection {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10ShaderReflection_Vtbl
    where
        Identity: ID3D10ShaderReflection_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>
        where
            Identity: ID3D10ShaderReflection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>
        where
            Identity: ID3D10ShaderReflection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflection1_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetMovInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetMovcInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetConversionInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetBitwiseInstructionCount(&self) -> windows_core::Result<u32>;
    fn GetGSInputPrimitive(&self) -> windows_core::Result<super::Direct3D::D3D_PRIMITIVE>;
    fn IsLevel9Shader(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsSampleFrequencyShader(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D10ShaderReflection1 {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflection1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10ShaderReflection1_Vtbl
    where
        Identity: ID3D10ShaderReflection1_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionConstantBuffer>
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionConstantBuffer>
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D10_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D10_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderReflection1_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::GetMovInstructionCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::GetMovcInstructionCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::GetConversionInstructionCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::GetBitwiseInstructionCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprim: *mut super::Direct3D::D3D_PRIMITIVE) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::GetGSInputPrimitive(this) {
                Ok(ok__) => {
                    pprim.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLevel9Shader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblevel9shader: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::IsLevel9Shader(this) {
                Ok(ok__) => {
                    pblevel9shader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsamplefrequency: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ID3D10ShaderReflection1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10ShaderReflection1_Impl::IsSampleFrequencyShader(this) {
                Ok(ok__) => {
                    pbsamplefrequency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, OFFSET>,
            IsLevel9Shader: IsLevel9Shader::<Identity, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderReflection1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>() -> ID3D10ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D10ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        Self { GetDesc: GetDesc::<Impl>, GetVariableByIndex: GetVariableByIndex::<Impl>, GetVariableByName: GetVariableByName::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D10ShaderReflectionConstantBuffer_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10ShaderReflectionConstantBuffer_Impl> ID3D10ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionConstantBuffer_Vtbl = ID3D10ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D10ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D10ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType_Vtbl {
    pub const fn new<Impl: ID3D10ShaderReflectionType_Impl>() -> ID3D10ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D10ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeName: GetMemberTypeName::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D10ShaderReflectionType_ImplVtbl<T: ID3D10ShaderReflectionType_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D10ShaderReflectionType_Impl> ID3D10ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionType_Vtbl = ID3D10ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D10ShaderReflectionType {
    pub fn new<'a, T: ID3D10ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D10ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D10ShaderReflectionType>;
}
impl ID3D10ShaderReflectionVariable_Vtbl {
    pub const fn new<Impl: ID3D10ShaderReflectionVariable_Impl>() -> ID3D10ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D10ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D10ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D10ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D10ShaderReflectionVariable_Impl::GetType(this)
        }
        Self { GetDesc: GetDesc::<Impl>, GetType: GetType::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D10ShaderReflectionVariable_ImplVtbl<T: ID3D10ShaderReflectionVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D10ShaderReflectionVariable_Impl> ID3D10ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D10ShaderReflectionVariable_Vtbl = ID3D10ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D10ShaderReflectionVariable {
    pub fn new<'a, T: ID3D10ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D10ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView_Impl: Sized + ID3D10View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10ShaderResourceView {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10ShaderResourceView_Vtbl
    where
        Identity: ID3D10ShaderResourceView_Impl,
    {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC)
        where
            Identity: ID3D10ShaderResourceView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderResourceView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10View_Vtbl::new::<Identity, OFFSET>(), GetDesc: GetDesc::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D10ShaderResourceView1_Impl: Sized + ID3D10ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D10ShaderResourceView1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D10ShaderResourceView1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10ShaderResourceView1_Vtbl
    where
        Identity: ID3D10ShaderResourceView1_Impl,
    {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC1)
        where
            Identity: ID3D10ShaderResourceView1_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10ShaderResourceView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D10ShaderResourceView_Vtbl::new::<Identity, OFFSET>(), GetDesc1: GetDesc1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10ShaderResourceView1 as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10View as windows_core::Interface>::IID || iid == &<ID3D10ShaderResourceView as windows_core::Interface>::IID
    }
}
pub trait ID3D10StateBlock_Impl: Sized {
    fn Capture(&self) -> windows_core::Result<()>;
    fn Apply(&self) -> windows_core::Result<()>;
    fn ReleaseAllDeviceObjects(&self) -> windows_core::Result<()>;
    fn GetDevice(&self) -> windows_core::Result<ID3D10Device>;
}
impl windows_core::RuntimeName for ID3D10StateBlock {}
impl ID3D10StateBlock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10StateBlock_Vtbl
    where
        Identity: ID3D10StateBlock_Impl,
    {
        unsafe extern "system" fn Capture<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10StateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10StateBlock_Impl::Capture(this).into()
        }
        unsafe extern "system" fn Apply<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10StateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10StateBlock_Impl::Apply(this).into()
        }
        unsafe extern "system" fn ReleaseAllDeviceObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10StateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10StateBlock_Impl::ReleaseAllDeviceObjects(this).into()
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10StateBlock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10StateBlock_Impl::GetDevice(this) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Capture: Capture::<Identity, OFFSET>,
            Apply: Apply::<Identity, OFFSET>,
            ReleaseAllDeviceObjects: ReleaseAllDeviceObjects::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10StateBlock as windows_core::Interface>::IID
    }
}
pub trait ID3D10SwitchToRef_Impl: Sized {
    fn SetUseRef(&self, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D10SwitchToRef {}
impl ID3D10SwitchToRef_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10SwitchToRef_Vtbl
    where
        Identity: ID3D10SwitchToRef_Impl,
    {
        unsafe extern "system" fn SetUseRef<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10SwitchToRef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10SwitchToRef_Impl::SetUseRef(this, core::mem::transmute_copy(&useref))
        }
        unsafe extern "system" fn GetUseRef<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL
        where
            Identity: ID3D10SwitchToRef_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10SwitchToRef_Impl::GetUseRef(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUseRef: SetUseRef::<Identity, OFFSET>,
            GetUseRef: GetUseRef::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10SwitchToRef as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture1D_Impl: Sized + ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10Texture1D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture1D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Texture1D_Vtbl
    where
        Identity: ID3D10Texture1D_Impl,
    {
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, ppdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ID3D10Texture1D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture1D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&ppdata)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32)
        where
            Identity: ID3D10Texture1D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture1D_Impl::Unmap(this, core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE1D_DESC)
        where
            Identity: ID3D10Texture1D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture1D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture1D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture2D_Impl: Sized + ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE2D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10Texture2D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture2D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Texture2D_Vtbl
    where
        Identity: ID3D10Texture2D_Impl,
    {
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex2d: *mut D3D10_MAPPED_TEXTURE2D) -> windows_core::HRESULT
        where
            Identity: ID3D10Texture2D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Texture2D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags)) {
                Ok(ok__) => {
                    pmappedtex2d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32)
        where
            Identity: ID3D10Texture2D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture2D_Impl::Unmap(this, core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE2D_DESC)
        where
            Identity: ID3D10Texture2D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture2D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture2D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D10Texture3D_Impl: Sized + ID3D10Resource_Impl {
    fn Map(&self, subresource: u32, maptype: D3D10_MAP, mapflags: u32) -> windows_core::Result<D3D10_MAPPED_TEXTURE3D>;
    fn Unmap(&self, subresource: u32);
    fn GetDesc(&self, pdesc: *mut D3D10_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D10Texture3D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D10Texture3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10Texture3D_Vtbl
    where
        Identity: ID3D10Texture3D_Impl,
    {
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32, maptype: D3D10_MAP, mapflags: u32, pmappedtex3d: *mut D3D10_MAPPED_TEXTURE3D) -> windows_core::HRESULT
        where
            Identity: ID3D10Texture3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ID3D10Texture3D_Impl::Map(this, core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags)) {
                Ok(ok__) => {
                    pmappedtex3d.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subresource: u32)
        where
            Identity: ID3D10Texture3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture3D_Impl::Unmap(this, core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D10_TEXTURE3D_DESC)
        where
            Identity: ID3D10Texture3D_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10Texture3D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self {
            base__: ID3D10Resource_Vtbl::new::<Identity, OFFSET>(),
            Map: Map::<Identity, OFFSET>,
            Unmap: Unmap::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10Texture3D as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID || iid == &<ID3D10Resource as windows_core::Interface>::IID
    }
}
pub trait ID3D10VertexShader_Impl: Sized + ID3D10DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D10VertexShader {}
impl ID3D10VertexShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10VertexShader_Vtbl
    where
        Identity: ID3D10VertexShader_Impl,
    {
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10VertexShader as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D10View_Impl: Sized + ID3D10DeviceChild_Impl {
    fn GetResource(&self, ppresource: *mut Option<ID3D10Resource>);
}
impl windows_core::RuntimeName for ID3D10View {}
impl ID3D10View_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ID3D10View_Vtbl
    where
        Identity: ID3D10View_Impl,
    {
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void)
        where
            Identity: ID3D10View_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ID3D10View_Impl::GetResource(this, core::mem::transmute_copy(&ppresource))
        }
        Self { base__: ID3D10DeviceChild_Vtbl::new::<Identity, OFFSET>(), GetResource: GetResource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D10View as windows_core::Interface>::IID || iid == &<ID3D10DeviceChild as windows_core::Interface>::IID
    }
}
