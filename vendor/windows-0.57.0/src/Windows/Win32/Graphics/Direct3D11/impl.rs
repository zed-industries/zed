pub trait ID3D11Asynchronous_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDataSize(&self) -> u32;
}
impl windows_core::RuntimeName for ID3D11Asynchronous {}
impl ID3D11Asynchronous_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Asynchronous_Impl, const OFFSET: isize>() -> ID3D11Asynchronous_Vtbl {
        unsafe extern "system" fn GetDataSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Asynchronous_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Asynchronous_Impl::GetDataSize(this)
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDataSize: GetDataSize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Asynchronous as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11AuthenticatedChannel_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCertificateSize(&self) -> windows_core::Result<u32>;
    fn GetCertificate(&self, certificatesize: u32, pcertificate: *mut u8) -> windows_core::Result<()>;
    fn GetChannelHandle(&self, pchannelhandle: *mut super::super::Foundation::HANDLE);
}
impl windows_core::RuntimeName for ID3D11AuthenticatedChannel {}
impl ID3D11AuthenticatedChannel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>() -> ID3D11AuthenticatedChannel_Vtbl {
        unsafe extern "system" fn GetCertificateSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11AuthenticatedChannel_Impl::GetCertificateSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pcertificatesize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11AuthenticatedChannel_Impl::GetCertificate(this, core::mem::transmute_copy(&certificatesize), core::mem::transmute_copy(&pcertificate)).into()
        }
        unsafe extern "system" fn GetChannelHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11AuthenticatedChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11AuthenticatedChannel_Impl::GetChannelHandle(this, core::mem::transmute_copy(&pchannelhandle))
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCertificateSize: GetCertificateSize::<Identity, Impl, OFFSET>,
            GetCertificate: GetCertificate::<Identity, Impl, OFFSET>,
            GetChannelHandle: GetChannelHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11AuthenticatedChannel as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11BlendState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_BLEND_DESC);
}
impl windows_core::RuntimeName for ID3D11BlendState {}
impl ID3D11BlendState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11BlendState_Impl, const OFFSET: isize>() -> ID3D11BlendState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11BlendState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11BlendState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11BlendState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11BlendState1_Impl: Sized + ID3D11BlendState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_BLEND_DESC1);
}
impl windows_core::RuntimeName for ID3D11BlendState1 {}
impl ID3D11BlendState1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11BlendState1_Impl, const OFFSET: isize>() -> ID3D11BlendState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11BlendState1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_BLEND_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11BlendState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11BlendState_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11BlendState1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11BlendState as windows_core::Interface>::IID
    }
}
pub trait ID3D11Buffer_Impl: Sized + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_BUFFER_DESC);
}
impl windows_core::RuntimeName for ID3D11Buffer {}
impl ID3D11Buffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Buffer_Impl, const OFFSET: isize>() -> ID3D11Buffer_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Buffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_BUFFER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Buffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Buffer as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
pub trait ID3D11ClassInstance_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetClassLinkage(&self, pplinkage: *mut Option<ID3D11ClassLinkage>);
    fn GetDesc(&self, pdesc: *mut D3D11_CLASS_INSTANCE_DESC);
    fn GetInstanceName(&self, pinstancename: windows_core::PSTR, pbufferlength: *mut usize);
    fn GetTypeName(&self, ptypename: windows_core::PSTR, pbufferlength: *mut usize);
}
impl windows_core::RuntimeName for ID3D11ClassInstance {}
impl ID3D11ClassInstance_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>() -> ID3D11ClassInstance_Vtbl {
        unsafe extern "system" fn GetClassLinkage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplinkage: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ClassInstance_Impl::GetClassLinkage(this, core::mem::transmute_copy(&pplinkage))
        }
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_CLASS_INSTANCE_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ClassInstance_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn GetInstanceName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstancename: windows_core::PSTR, pbufferlength: *mut usize) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ClassInstance_Impl::GetInstanceName(this, core::mem::transmute_copy(&pinstancename), core::mem::transmute_copy(&pbufferlength))
        }
        unsafe extern "system" fn GetTypeName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypename: windows_core::PSTR, pbufferlength: *mut usize) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ClassInstance_Impl::GetTypeName(this, core::mem::transmute_copy(&ptypename), core::mem::transmute_copy(&pbufferlength))
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetClassLinkage: GetClassLinkage::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetInstanceName: GetInstanceName::<Identity, Impl, OFFSET>,
            GetTypeName: GetTypeName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ClassInstance as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11ClassLinkage_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetClassInstance(&self, pclassinstancename: &windows_core::PCSTR, instanceindex: u32) -> windows_core::Result<ID3D11ClassInstance>;
    fn CreateClassInstance(&self, pclasstypename: &windows_core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32) -> windows_core::Result<ID3D11ClassInstance>;
}
impl windows_core::RuntimeName for ID3D11ClassLinkage {}
impl ID3D11ClassLinkage_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>() -> ID3D11ClassLinkage_Vtbl {
        unsafe extern "system" fn GetClassInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassinstancename: windows_core::PCSTR, instanceindex: u32, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11ClassLinkage_Impl::GetClassInstance(this, core::mem::transmute(&pclassinstancename), core::mem::transmute_copy(&instanceindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateClassInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ClassLinkage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclasstypename: windows_core::PCSTR, constantbufferoffset: u32, constantvectoroffset: u32, textureoffset: u32, sampleroffset: u32, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11ClassLinkage_Impl::CreateClassInstance(this, core::mem::transmute(&pclasstypename), core::mem::transmute_copy(&constantbufferoffset), core::mem::transmute_copy(&constantvectoroffset), core::mem::transmute_copy(&textureoffset), core::mem::transmute_copy(&sampleroffset)) {
                Ok(ok__) => {
                    core::ptr::write(ppinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetClassInstance: GetClassInstance::<Identity, Impl, OFFSET>,
            CreateClassInstance: CreateClassInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ClassLinkage as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11CommandList_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetContextFlags(&self) -> u32;
}
impl windows_core::RuntimeName for ID3D11CommandList {}
impl ID3D11CommandList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CommandList_Impl, const OFFSET: isize>() -> ID3D11CommandList_Vtbl {
        unsafe extern "system" fn GetContextFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CommandList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11CommandList_Impl::GetContextFlags(this)
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetContextFlags: GetContextFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11CommandList as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11ComputeShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11ComputeShader {}
impl ID3D11ComputeShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ComputeShader_Impl, const OFFSET: isize>() -> ID3D11ComputeShader_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ComputeShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11Counter_Impl: Sized + ID3D11Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_COUNTER_DESC);
}
impl windows_core::RuntimeName for ID3D11Counter {}
impl ID3D11Counter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Counter_Impl, const OFFSET: isize>() -> ID3D11Counter_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Counter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_COUNTER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Counter_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Asynchronous_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Counter as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID
    }
}
pub trait ID3D11CryptoSession_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCryptoType(&self, pcryptotype: *mut windows_core::GUID);
    fn GetDecoderProfile(&self, pdecoderprofile: *mut windows_core::GUID);
    fn GetCertificateSize(&self) -> windows_core::Result<u32>;
    fn GetCertificate(&self, certificatesize: u32, pcertificate: *mut u8) -> windows_core::Result<()>;
    fn GetCryptoSessionHandle(&self, pcryptosessionhandle: *mut super::super::Foundation::HANDLE);
}
impl windows_core::RuntimeName for ID3D11CryptoSession {}
impl ID3D11CryptoSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>() -> ID3D11CryptoSession_Vtbl {
        unsafe extern "system" fn GetCryptoType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *mut windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11CryptoSession_Impl::GetCryptoType(this, core::mem::transmute_copy(&pcryptotype))
        }
        unsafe extern "system" fn GetDecoderProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoderprofile: *mut windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11CryptoSession_Impl::GetDecoderProfile(this, core::mem::transmute_copy(&pdecoderprofile))
        }
        unsafe extern "system" fn GetCertificateSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertificatesize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11CryptoSession_Impl::GetCertificateSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pcertificatesize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificatesize: u32, pcertificate: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11CryptoSession_Impl::GetCertificate(this, core::mem::transmute_copy(&certificatesize), core::mem::transmute_copy(&pcertificate)).into()
        }
        unsafe extern "system" fn GetCryptoSessionHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11CryptoSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosessionhandle: *mut super::super::Foundation::HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11CryptoSession_Impl::GetCryptoSessionHandle(this, core::mem::transmute_copy(&pcryptosessionhandle))
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCryptoType: GetCryptoType::<Identity, Impl, OFFSET>,
            GetDecoderProfile: GetDecoderProfile::<Identity, Impl, OFFSET>,
            GetCertificateSize: GetCertificateSize::<Identity, Impl, OFFSET>,
            GetCertificate: GetCertificate::<Identity, Impl, OFFSET>,
            GetCryptoSessionHandle: GetCryptoSessionHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11CryptoSession as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ID3D11Debug_Impl: Sized {
    fn SetFeatureMask(&self, mask: u32) -> windows_core::Result<()>;
    fn GetFeatureMask(&self) -> u32;
    fn SetPresentPerRenderOpDelay(&self, milliseconds: u32) -> windows_core::Result<()>;
    fn GetPresentPerRenderOpDelay(&self) -> u32;
    fn SetSwapChain(&self, pswapchain: Option<&super::Dxgi::IDXGISwapChain>) -> windows_core::Result<()>;
    fn GetSwapChain(&self) -> windows_core::Result<super::Dxgi::IDXGISwapChain>;
    fn ValidateContext(&self, pcontext: Option<&ID3D11DeviceContext>) -> windows_core::Result<()>;
    fn ReportLiveDeviceObjects(&self, flags: D3D11_RLDO_FLAGS) -> windows_core::Result<()>;
    fn ValidateContextForDispatch(&self, pcontext: Option<&ID3D11DeviceContext>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for ID3D11Debug {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ID3D11Debug_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>() -> ID3D11Debug_Vtbl {
        unsafe extern "system" fn SetFeatureMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::SetFeatureMask(this, core::mem::transmute_copy(&mask)).into()
        }
        unsafe extern "system" fn GetFeatureMask<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::GetFeatureMask(this)
        }
        unsafe extern "system" fn SetPresentPerRenderOpDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, milliseconds: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::SetPresentPerRenderOpDelay(this, core::mem::transmute_copy(&milliseconds)).into()
        }
        unsafe extern "system" fn GetPresentPerRenderOpDelay<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::GetPresentPerRenderOpDelay(this)
        }
        unsafe extern "system" fn SetSwapChain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pswapchain: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::SetSwapChain(this, windows_core::from_raw_borrowed(&pswapchain)).into()
        }
        unsafe extern "system" fn GetSwapChain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppswapchain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Debug_Impl::GetSwapChain(this) {
                Ok(ok__) => {
                    core::ptr::write(ppswapchain, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::ValidateContext(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn ReportLiveDeviceObjects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: D3D11_RLDO_FLAGS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::ReportLiveDeviceObjects(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ValidateContextForDispatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Debug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Debug_Impl::ValidateContextForDispatch(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFeatureMask: SetFeatureMask::<Identity, Impl, OFFSET>,
            GetFeatureMask: GetFeatureMask::<Identity, Impl, OFFSET>,
            SetPresentPerRenderOpDelay: SetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            GetPresentPerRenderOpDelay: GetPresentPerRenderOpDelay::<Identity, Impl, OFFSET>,
            SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET>,
            GetSwapChain: GetSwapChain::<Identity, Impl, OFFSET>,
            ValidateContext: ValidateContext::<Identity, Impl, OFFSET>,
            ReportLiveDeviceObjects: ReportLiveDeviceObjects::<Identity, Impl, OFFSET>,
            ValidateContextForDispatch: ValidateContextForDispatch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Debug as windows_core::Interface>::IID
    }
}
pub trait ID3D11DepthStencilState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_DESC);
}
impl windows_core::RuntimeName for ID3D11DepthStencilState {}
impl ID3D11DepthStencilState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DepthStencilState_Impl, const OFFSET: isize>() -> ID3D11DepthStencilState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DepthStencilState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DepthStencilState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DepthStencilState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11DepthStencilView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11DepthStencilView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11DepthStencilView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DepthStencilView_Impl, const OFFSET: isize>() -> ID3D11DepthStencilView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DepthStencilView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_DEPTH_STENCIL_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DepthStencilView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DepthStencilView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device_Impl: Sized {
    fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut Option<ID3D11Buffer>) -> windows_core::Result<()>;
    fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut Option<ID3D11Texture1D>) -> windows_core::Result<()>;
    fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut Option<ID3D11Texture2D>) -> windows_core::Result<()>;
    fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut Option<ID3D11Texture3D>) -> windows_core::Result<()>;
    fn CreateShaderResourceView(&self, presource: Option<&ID3D11Resource>, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut Option<ID3D11ShaderResourceView>) -> windows_core::Result<()>;
    fn CreateUnorderedAccessView(&self, presource: Option<&ID3D11Resource>, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut Option<ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
    fn CreateRenderTargetView(&self, presource: Option<&ID3D11Resource>, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut Option<ID3D11RenderTargetView>) -> windows_core::Result<()>;
    fn CreateDepthStencilView(&self, presource: Option<&ID3D11Resource>, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut Option<ID3D11DepthStencilView>) -> windows_core::Result<()>;
    fn CreateInputLayout(&self, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut Option<ID3D11InputLayout>) -> windows_core::Result<()>;
    fn CreateVertexShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: Option<&ID3D11ClassLinkage>, ppvertexshader: *mut Option<ID3D11VertexShader>) -> windows_core::Result<()>;
    fn CreateGeometryShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: Option<&ID3D11ClassLinkage>, ppgeometryshader: *mut Option<ID3D11GeometryShader>) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: Option<&ID3D11ClassLinkage>, ppgeometryshader: *mut Option<ID3D11GeometryShader>) -> windows_core::Result<()>;
    fn CreatePixelShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: Option<&ID3D11ClassLinkage>, pppixelshader: *mut Option<ID3D11PixelShader>) -> windows_core::Result<()>;
    fn CreateHullShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: Option<&ID3D11ClassLinkage>, pphullshader: *mut Option<ID3D11HullShader>) -> windows_core::Result<()>;
    fn CreateDomainShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: Option<&ID3D11ClassLinkage>, ppdomainshader: *mut Option<ID3D11DomainShader>) -> windows_core::Result<()>;
    fn CreateComputeShader(&self, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: Option<&ID3D11ClassLinkage>, ppcomputeshader: *mut Option<ID3D11ComputeShader>) -> windows_core::Result<()>;
    fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut Option<ID3D11BlendState>) -> windows_core::Result<()>;
    fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut Option<ID3D11DepthStencilState>) -> windows_core::Result<()>;
    fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut Option<ID3D11RasterizerState>) -> windows_core::Result<()>;
    fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut Option<ID3D11SamplerState>) -> windows_core::Result<()>;
    fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut Option<ID3D11Query>) -> windows_core::Result<()>;
    fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut Option<ID3D11Predicate>) -> windows_core::Result<()>;
    fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut Option<ID3D11Counter>) -> windows_core::Result<()>;
    fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: *mut Option<ID3D11DeviceContext>) -> windows_core::Result<()>;
    fn OpenSharedResource(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::Result<()>;
    fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL;
    fn GetCreationFlags(&self) -> u32;
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn GetImmediateContext(&self, ppimmediatecontext: *mut Option<ID3D11DeviceContext>);
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>() -> ID3D11Device_Vtbl {
        unsafe extern "system" fn CreateBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateBuffer(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn CreateTexture1D<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture1d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateTexture1D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture1d)).into()
        }
        unsafe extern "system" fn CreateTexture2D<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateTexture2D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture2d)).into()
        }
        unsafe extern "system" fn CreateTexture3D<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateTexture3D(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture3d)).into()
        }
        unsafe extern "system" fn CreateShaderResourceView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC, ppsrview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateShaderResourceView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppsrview)).into()
        }
        unsafe extern "system" fn CreateUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppuaview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateUnorderedAccessView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppuaview)).into()
        }
        unsafe extern "system" fn CreateRenderTargetView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC, pprtview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateRenderTargetView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&pprtview)).into()
        }
        unsafe extern "system" fn CreateDepthStencilView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC, ppdepthstencilview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateDepthStencilView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppdepthstencilview)).into()
        }
        unsafe extern "system" fn CreateInputLayout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC, numelements: u32, pshaderbytecodewithinputsignature: *const core::ffi::c_void, bytecodelength: usize, ppinputlayout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateInputLayout(this, core::mem::transmute_copy(&pinputelementdescs), core::mem::transmute_copy(&numelements), core::mem::transmute_copy(&pshaderbytecodewithinputsignature), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&ppinputlayout)).into()
        }
        unsafe extern "system" fn CreateVertexShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateVertexShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&ppvertexshader)).into()
        }
        unsafe extern "system" fn CreateGeometryShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateGeometryShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&ppgeometryshader)).into()
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, psodeclaration: *const D3D11_SO_DECLARATION_ENTRY, numentries: u32, pbufferstrides: *const u32, numstrides: u32, rasterizedstream: u32, pclasslinkage: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateGeometryShaderWithStreamOutput(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), core::mem::transmute_copy(&psodeclaration), core::mem::transmute_copy(&numentries), core::mem::transmute_copy(&pbufferstrides), core::mem::transmute_copy(&numstrides), core::mem::transmute_copy(&rasterizedstream), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&ppgeometryshader)).into()
        }
        unsafe extern "system" fn CreatePixelShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreatePixelShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&pppixelshader)).into()
        }
        unsafe extern "system" fn CreateHullShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, pphullshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateHullShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&pphullshader)).into()
        }
        unsafe extern "system" fn CreateDomainShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppdomainshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateDomainShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&ppdomainshader)).into()
        }
        unsafe extern "system" fn CreateComputeShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderbytecode: *const core::ffi::c_void, bytecodelength: usize, pclasslinkage: *mut core::ffi::c_void, ppcomputeshader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateComputeShader(this, core::mem::transmute_copy(&pshaderbytecode), core::mem::transmute_copy(&bytecodelength), windows_core::from_raw_borrowed(&pclasslinkage), core::mem::transmute_copy(&ppcomputeshader)).into()
        }
        unsafe extern "system" fn CreateClassLinkage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplinkage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Device_Impl::CreateClassLinkage(this) {
                Ok(ok__) => {
                    core::ptr::write(pplinkage, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBlendState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateBlendState(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
        }
        unsafe extern "system" fn CreateDepthStencilState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateDepthStencilState(this, core::mem::transmute_copy(&pdepthstencildesc), core::mem::transmute_copy(&ppdepthstencilstate)).into()
        }
        unsafe extern "system" fn CreateRasterizerState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateRasterizerState(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
        }
        unsafe extern "system" fn CreateSamplerState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateSamplerState(this, core::mem::transmute_copy(&psamplerdesc), core::mem::transmute_copy(&ppsamplerstate)).into()
        }
        unsafe extern "system" fn CreateQuery<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc: *const D3D11_QUERY_DESC, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateQuery(this, core::mem::transmute_copy(&pquerydesc), core::mem::transmute_copy(&ppquery)).into()
        }
        unsafe extern "system" fn CreatePredicate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreatePredicate(this, core::mem::transmute_copy(&ppredicatedesc), core::mem::transmute_copy(&pppredicate)).into()
        }
        unsafe extern "system" fn CreateCounter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateCounter(this, core::mem::transmute_copy(&pcounterdesc), core::mem::transmute_copy(&ppcounter)).into()
        }
        unsafe extern "system" fn CreateDeferredContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CreateDeferredContext(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
        }
        unsafe extern "system" fn OpenSharedResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::OpenSharedResource(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn CheckFormatSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pformatsupport: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Device_Impl::CheckFormatSupport(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    core::ptr::write(pformatsupport, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Device_Impl::CheckMultisampleQualityLevels(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount)) {
                Ok(ok__) => {
                    core::ptr::write(pnumqualitylevels, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCounterInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcounterinfo: *mut D3D11_COUNTER_INFO) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo))
        }
        unsafe extern "system" fn CheckCounter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: windows_core::PSTR, pnamelength: *mut u32, szunits: windows_core::PSTR, punitslength: *mut u32, szdescription: windows_core::PSTR, pdescriptionlength: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CheckCounter(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&pactivecounters), core::mem::transmute_copy(&szname), core::mem::transmute_copy(&pnamelength), core::mem::transmute_copy(&szunits), core::mem::transmute_copy(&punitslength), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pdescriptionlength)).into()
        }
        unsafe extern "system" fn CheckFeatureSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: D3D11_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&pdata)).into()
        }
        unsafe extern "system" fn GetFeatureLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::GetFeatureLevel(this)
        }
        unsafe extern "system" fn GetCreationFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::GetCreationFlags(this)
        }
        unsafe extern "system" fn GetDeviceRemovedReason<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::GetDeviceRemovedReason(this).into()
        }
        unsafe extern "system" fn GetImmediateContext<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::GetImmediateContext(this, core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn SetExceptionMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, raiseflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags)).into()
        }
        unsafe extern "system" fn GetExceptionMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device_Impl::GetExceptionMode(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBuffer: CreateBuffer::<Identity, Impl, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, Impl, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, Impl, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, Impl, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, Impl, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, Impl, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, Impl, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, Impl, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, Impl, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, Impl, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<Identity, Impl, OFFSET>,
            CreatePixelShader: CreatePixelShader::<Identity, Impl, OFFSET>,
            CreateHullShader: CreateHullShader::<Identity, Impl, OFFSET>,
            CreateDomainShader: CreateDomainShader::<Identity, Impl, OFFSET>,
            CreateComputeShader: CreateComputeShader::<Identity, Impl, OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Identity, Impl, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, Impl, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, Impl, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, Impl, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, Impl, OFFSET>,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, Impl, OFFSET>,
            CreateCounter: CreateCounter::<Identity, Impl, OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Identity, Impl, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, Impl, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, Impl, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, Impl, OFFSET>,
            CheckCounter: CheckCounter::<Identity, Impl, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, Impl, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, Impl, OFFSET>,
            GetImmediateContext: GetImmediateContext::<Identity, Impl, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, Impl, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device1_Impl: Sized + ID3D11Device_Impl {
    fn GetImmediateContext1(&self, ppimmediatecontext: *mut Option<ID3D11DeviceContext1>);
    fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: *mut Option<ID3D11DeviceContext1>) -> windows_core::Result<()>;
    fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut Option<ID3D11BlendState1>) -> windows_core::Result<()>;
    fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut Option<ID3D11RasterizerState1>) -> windows_core::Result<()>;
    fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const windows_core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut Option<ID3DDeviceContextState>) -> windows_core::Result<()>;
    fn OpenSharedResource1(&self, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenSharedResourceByName(&self, lpname: &windows_core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>() -> ID3D11Device1_Vtbl {
        unsafe extern "system" fn GetImmediateContext1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::GetImmediateContext1(this, core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::CreateDeferredContext1(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
        }
        unsafe extern "system" fn CreateBlendState1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::CreateBlendState1(this, core::mem::transmute_copy(&pblendstatedesc), core::mem::transmute_copy(&ppblendstate)).into()
        }
        unsafe extern "system" fn CreateRasterizerState1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::CreateRasterizerState1(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
        }
        unsafe extern "system" fn CreateDeviceContextState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: u32, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, emulatedinterface: *const windows_core::GUID, pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppcontextstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::CreateDeviceContextState(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pfeaturelevels), core::mem::transmute_copy(&featurelevels), core::mem::transmute_copy(&sdkversion), core::mem::transmute_copy(&emulatedinterface), core::mem::transmute_copy(&pchosenfeaturelevel), core::mem::transmute_copy(&ppcontextstate)).into()
        }
        unsafe extern "system" fn OpenSharedResource1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresource: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::OpenSharedResource1(this, core::mem::transmute_copy(&hresource), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
        }
        unsafe extern "system" fn OpenSharedResourceByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpname: windows_core::PCWSTR, dwdesiredaccess: u32, returnedinterface: *const windows_core::GUID, ppresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device1_Impl::OpenSharedResourceByName(this, core::mem::transmute(&lpname), core::mem::transmute_copy(&dwdesiredaccess), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppresource)).into()
        }
        Self {
            base__: ID3D11Device_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImmediateContext1: GetImmediateContext1::<Identity, Impl, OFFSET>,
            CreateDeferredContext1: CreateDeferredContext1::<Identity, Impl, OFFSET>,
            CreateBlendState1: CreateBlendState1::<Identity, Impl, OFFSET>,
            CreateRasterizerState1: CreateRasterizerState1::<Identity, Impl, OFFSET>,
            CreateDeviceContextState: CreateDeviceContextState::<Identity, Impl, OFFSET>,
            OpenSharedResource1: OpenSharedResource1::<Identity, Impl, OFFSET>,
            OpenSharedResourceByName: OpenSharedResourceByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device1 as windows_core::Interface>::IID || iid == &<ID3D11Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device2_Impl: Sized + ID3D11Device1_Impl {
    fn GetImmediateContext2(&self, ppimmediatecontext: *mut Option<ID3D11DeviceContext2>);
    fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: *mut Option<ID3D11DeviceContext2>) -> windows_core::Result<()>;
    fn GetResourceTiling(&self, ptiledresource: Option<&ID3D11Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING);
    fn CheckMultisampleQualityLevels1(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: isize>() -> ID3D11Device2_Vtbl {
        unsafe extern "system" fn GetImmediateContext2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device2_Impl::GetImmediateContext2(this, core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device2_Impl::CreateDeferredContext2(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
        }
        unsafe extern "system" fn GetResourceTiling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device2_Impl::GetResourceTiling(this, windows_core::from_raw_borrowed(&ptiledresource), core::mem::transmute_copy(&pnumtilesforentireresource), core::mem::transmute_copy(&ppackedmipdesc), core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), core::mem::transmute_copy(&pnumsubresourcetilings), core::mem::transmute_copy(&firstsubresourcetilingtoget), core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips))
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Device2_Impl::CheckMultisampleQualityLevels1(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    core::ptr::write(pnumqualitylevels, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11Device1_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetImmediateContext2: GetImmediateContext2::<Identity, Impl, OFFSET>,
            CreateDeferredContext2: CreateDeferredContext2::<Identity, Impl, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, Impl, OFFSET>,
            CheckMultisampleQualityLevels1: CheckMultisampleQualityLevels1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device2 as windows_core::Interface>::IID || iid == &<ID3D11Device as windows_core::Interface>::IID || iid == &<ID3D11Device1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device3_Impl: Sized + ID3D11Device2_Impl {
    fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut Option<ID3D11Texture2D1>) -> windows_core::Result<()>;
    fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut Option<ID3D11Texture3D1>) -> windows_core::Result<()>;
    fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut Option<ID3D11RasterizerState2>) -> windows_core::Result<()>;
    fn CreateShaderResourceView1(&self, presource: Option<&ID3D11Resource>, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut Option<ID3D11ShaderResourceView1>) -> windows_core::Result<()>;
    fn CreateUnorderedAccessView1(&self, presource: Option<&ID3D11Resource>, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut Option<ID3D11UnorderedAccessView1>) -> windows_core::Result<()>;
    fn CreateRenderTargetView1(&self, presource: Option<&ID3D11Resource>, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut Option<ID3D11RenderTargetView1>) -> windows_core::Result<()>;
    fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut Option<ID3D11Query1>) -> windows_core::Result<()>;
    fn GetImmediateContext3(&self, ppimmediatecontext: *mut Option<ID3D11DeviceContext3>);
    fn CreateDeferredContext3(&self, contextflags: u32, ppdeferredcontext: *mut Option<ID3D11DeviceContext3>) -> windows_core::Result<()>;
    fn WriteToSubresource(&self, pdstresource: Option<&ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn ReadFromSubresource(&self, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: Option<&ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device3 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>() -> ID3D11Device3_Vtbl {
        unsafe extern "system" fn CreateTexture2D1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture2d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateTexture2D1(this, core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture2d)).into()
        }
        unsafe extern "system" fn CreateTexture3D1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: *const D3D11_SUBRESOURCE_DATA, pptexture3d: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateTexture3D1(this, core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&pinitialdata), core::mem::transmute_copy(&pptexture3d)).into()
        }
        unsafe extern "system" fn CreateRasterizerState2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateRasterizerState2(this, core::mem::transmute_copy(&prasterizerdesc), core::mem::transmute_copy(&pprasterizerstate)).into()
        }
        unsafe extern "system" fn CreateShaderResourceView1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc1: *const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppsrview1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateShaderResourceView1(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&ppsrview1)).into()
        }
        unsafe extern "system" fn CreateUnorderedAccessView1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc1: *const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppuaview1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateUnorderedAccessView1(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&ppuaview1)).into()
        }
        unsafe extern "system" fn CreateRenderTargetView1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc1: *const D3D11_RENDER_TARGET_VIEW_DESC1, pprtview1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateRenderTargetView1(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc1), core::mem::transmute_copy(&pprtview1)).into()
        }
        unsafe extern "system" fn CreateQuery1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateQuery1(this, core::mem::transmute_copy(&pquerydesc1), core::mem::transmute_copy(&ppquery1)).into()
        }
        unsafe extern "system" fn GetImmediateContext3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::GetImmediateContext3(this, core::mem::transmute_copy(&ppimmediatecontext))
        }
        unsafe extern "system" fn CreateDeferredContext3<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::CreateDeferredContext3(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
        }
        unsafe extern "system" fn WriteToSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::WriteToSubresource(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn ReadFromSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstdata: *mut core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device3_Impl::ReadFromSubresource(this, core::mem::transmute_copy(&pdstdata), core::mem::transmute_copy(&dstrowpitch), core::mem::transmute_copy(&dstdepthpitch), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox))
        }
        Self {
            base__: ID3D11Device2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateTexture2D1: CreateTexture2D1::<Identity, Impl, OFFSET>,
            CreateTexture3D1: CreateTexture3D1::<Identity, Impl, OFFSET>,
            CreateRasterizerState2: CreateRasterizerState2::<Identity, Impl, OFFSET>,
            CreateShaderResourceView1: CreateShaderResourceView1::<Identity, Impl, OFFSET>,
            CreateUnorderedAccessView1: CreateUnorderedAccessView1::<Identity, Impl, OFFSET>,
            CreateRenderTargetView1: CreateRenderTargetView1::<Identity, Impl, OFFSET>,
            CreateQuery1: CreateQuery1::<Identity, Impl, OFFSET>,
            GetImmediateContext3: GetImmediateContext3::<Identity, Impl, OFFSET>,
            CreateDeferredContext3: CreateDeferredContext3::<Identity, Impl, OFFSET>,
            WriteToSubresource: WriteToSubresource::<Identity, Impl, OFFSET>,
            ReadFromSubresource: ReadFromSubresource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device3 as windows_core::Interface>::IID || iid == &<ID3D11Device as windows_core::Interface>::IID || iid == &<ID3D11Device1 as windows_core::Interface>::IID || iid == &<ID3D11Device2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device4_Impl: Sized + ID3D11Device3_Impl {
    fn RegisterDeviceRemovedEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterDeviceRemoved(&self, dwcookie: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device4 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device4_Impl, const OFFSET: isize>() -> ID3D11Device4_Vtbl {
        unsafe extern "system" fn RegisterDeviceRemovedEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Device4_Impl::RegisterDeviceRemovedEvent(this, core::mem::transmute_copy(&hevent)) {
                Ok(ok__) => {
                    core::ptr::write(pdwcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceRemoved<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device4_Impl::UnregisterDeviceRemoved(this, core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base__: ID3D11Device3_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterDeviceRemovedEvent: RegisterDeviceRemovedEvent::<Identity, Impl, OFFSET>,
            UnregisterDeviceRemoved: UnregisterDeviceRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device4 as windows_core::Interface>::IID || iid == &<ID3D11Device as windows_core::Interface>::IID || iid == &<ID3D11Device1 as windows_core::Interface>::IID || iid == &<ID3D11Device2 as windows_core::Interface>::IID || iid == &<ID3D11Device3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11Device5_Impl: Sized + ID3D11Device4_Impl {
    fn OpenSharedFence(&self, hfence: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateFence(&self, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device5 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11Device5_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device5_Impl, const OFFSET: isize>() -> ID3D11Device5_Vtbl {
        unsafe extern "system" fn OpenSharedFence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfence: super::super::Foundation::HANDLE, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device5_Impl::OpenSharedFence(this, core::mem::transmute_copy(&hfence), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppfence)).into()
        }
        unsafe extern "system" fn CreateFence<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initialvalue: u64, flags: D3D11_FENCE_FLAG, returnedinterface: *const windows_core::GUID, ppfence: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Device5_Impl::CreateFence(this, core::mem::transmute_copy(&initialvalue), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&returnedinterface), core::mem::transmute_copy(&ppfence)).into()
        }
        Self {
            base__: ID3D11Device4_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenSharedFence: OpenSharedFence::<Identity, Impl, OFFSET>,
            CreateFence: CreateFence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device5 as windows_core::Interface>::IID || iid == &<ID3D11Device as windows_core::Interface>::IID || iid == &<ID3D11Device1 as windows_core::Interface>::IID || iid == &<ID3D11Device2 as windows_core::Interface>::IID || iid == &<ID3D11Device3 as windows_core::Interface>::IID || iid == &<ID3D11Device4 as windows_core::Interface>::IID
    }
}
pub trait ID3D11DeviceChild_Impl: Sized {
    fn GetDevice(&self, ppdevice: *mut Option<ID3D11Device>);
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D11DeviceChild {}
impl ID3D11DeviceChild_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>() -> ID3D11DeviceChild_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice))
        }
        unsafe extern "system" fn GetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceChild_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceChild_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceChild_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext_Impl: Sized + ID3D11DeviceChild_Impl {
    fn VSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn PSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn PSSetShader(&self, ppixelshader: Option<&ID3D11PixelShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn PSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn VSSetShader(&self, pvertexshader: Option<&ID3D11VertexShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32);
    fn Draw(&self, vertexcount: u32, startvertexlocation: u32);
    fn Map(&self, presource: Option<&ID3D11Resource>, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> windows_core::Result<()>;
    fn Unmap(&self, presource: Option<&ID3D11Resource>, subresource: u32);
    fn PSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn IASetInputLayout(&self, pinputlayout: Option<&ID3D11InputLayout>);
    fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *const Option<ID3D11Buffer>, pstrides: *const u32, poffsets: *const u32);
    fn IASetIndexBuffer(&self, pindexbuffer: Option<&ID3D11Buffer>, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32);
    fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32);
    fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32);
    fn GSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn GSSetShader(&self, pshader: Option<&ID3D11GeometryShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn VSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn Begin(&self, pasync: Option<&ID3D11Asynchronous>);
    fn End(&self, pasync: Option<&ID3D11Asynchronous>);
    fn GetData(&self, pasync: Option<&ID3D11Asynchronous>, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::Result<()>;
    fn SetPredication(&self, ppredicate: Option<&ID3D11Predicate>, predicatevalue: super::super::Foundation::BOOL);
    fn GSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn GSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn OMSetRenderTargets(&self, numviews: u32, pprendertargetviews: *const Option<ID3D11RenderTargetView>, pdepthstencilview: Option<&ID3D11DepthStencilView>);
    fn OMSetRenderTargetsAndUnorderedAccessViews(&self, numrtvs: u32, pprendertargetviews: *const Option<ID3D11RenderTargetView>, pdepthstencilview: Option<&ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn OMSetBlendState(&self, pblendstate: Option<&ID3D11BlendState>, blendfactor: *const f32, samplemask: u32);
    fn OMSetDepthStencilState(&self, pdepthstencilstate: Option<&ID3D11DepthStencilState>, stencilref: u32);
    fn SOSetTargets(&self, numbuffers: u32, ppsotargets: *const Option<ID3D11Buffer>, poffsets: *const u32);
    fn DrawAuto(&self);
    fn DrawIndexedInstancedIndirect(&self, pbufferforargs: Option<&ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn DrawInstancedIndirect(&self, pbufferforargs: Option<&ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32);
    fn DispatchIndirect(&self, pbufferforargs: Option<&ID3D11Buffer>, alignedbyteoffsetforargs: u32);
    fn RSSetState(&self, prasterizerstate: Option<&ID3D11RasterizerState>);
    fn RSSetViewports(&self, numviewports: u32, pviewports: *const D3D11_VIEWPORT);
    fn RSSetScissorRects(&self, numrects: u32, prects: *const super::super::Foundation::RECT);
    fn CopySubresourceRegion(&self, pdstresource: Option<&ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: Option<&ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX);
    fn CopyResource(&self, pdstresource: Option<&ID3D11Resource>, psrcresource: Option<&ID3D11Resource>);
    fn UpdateSubresource(&self, pdstresource: Option<&ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32);
    fn CopyStructureCount(&self, pdstbuffer: Option<&ID3D11Buffer>, dstalignedbyteoffset: u32, psrcview: Option<&ID3D11UnorderedAccessView>);
    fn ClearRenderTargetView(&self, prendertargetview: Option<&ID3D11RenderTargetView>, colorrgba: *const f32);
    fn ClearUnorderedAccessViewUint(&self, punorderedaccessview: Option<&ID3D11UnorderedAccessView>, values: *const u32);
    fn ClearUnorderedAccessViewFloat(&self, punorderedaccessview: Option<&ID3D11UnorderedAccessView>, values: *const f32);
    fn ClearDepthStencilView(&self, pdepthstencilview: Option<&ID3D11DepthStencilView>, clearflags: u32, depth: f32, stencil: u8);
    fn GenerateMips(&self, pshaderresourceview: Option<&ID3D11ShaderResourceView>);
    fn SetResourceMinLOD(&self, presource: Option<&ID3D11Resource>, minlod: f32);
    fn GetResourceMinLOD(&self, presource: Option<&ID3D11Resource>) -> f32;
    fn ResolveSubresource(&self, pdstresource: Option<&ID3D11Resource>, dstsubresource: u32, psrcresource: Option<&ID3D11Resource>, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT);
    fn ExecuteCommandList(&self, pcommandlist: Option<&ID3D11CommandList>, restorecontextstate: super::super::Foundation::BOOL);
    fn HSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn HSSetShader(&self, phullshader: Option<&ID3D11HullShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn HSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn HSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn DSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn DSSetShader(&self, pdomainshader: Option<&ID3D11DomainShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn DSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn DSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn CSSetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *const Option<ID3D11ShaderResourceView>);
    fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const Option<ID3D11UnorderedAccessView>, puavinitialcounts: *const u32);
    fn CSSetShader(&self, pcomputeshader: Option<&ID3D11ComputeShader>, ppclassinstances: *const Option<ID3D11ClassInstance>, numclassinstances: u32);
    fn CSSetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *const Option<ID3D11SamplerState>);
    fn CSSetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>);
    fn VSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn PSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn PSGetShader(&self, pppixelshader: *mut Option<ID3D11PixelShader>, ppclassinstances: *mut Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn VSGetShader(&self, ppvertexshader: *mut Option<ID3D11VertexShader>, ppclassinstances: *mut Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn PSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn IAGetInputLayout(&self, ppinputlayout: *mut Option<ID3D11InputLayout>);
    fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut Option<ID3D11Buffer>, pstrides: *mut u32, poffsets: *mut u32);
    fn IAGetIndexBuffer(&self, pindexbuffer: *mut Option<ID3D11Buffer>, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32);
    fn GSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn GSGetShader(&self, ppgeometryshader: *mut Option<ID3D11GeometryShader>, ppclassinstances: *mut Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn IAGetPrimitiveTopology(&self, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY);
    fn VSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn VSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn GetPredication(&self, pppredicate: *mut Option<ID3D11Predicate>, ppredicatevalue: *mut super::super::Foundation::BOOL);
    fn GSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn GSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn OMGetRenderTargets(&self, numviews: u32, pprendertargetviews: *mut Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut Option<ID3D11DepthStencilView>);
    fn OMGetRenderTargetsAndUnorderedAccessViews(&self, numrtvs: u32, pprendertargetviews: *mut Option<ID3D11RenderTargetView>, ppdepthstencilview: *mut Option<ID3D11DepthStencilView>, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut Option<ID3D11UnorderedAccessView>);
    fn OMGetBlendState(&self, ppblendstate: *mut Option<ID3D11BlendState>, blendfactor: *mut f32, psamplemask: *mut u32);
    fn OMGetDepthStencilState(&self, ppdepthstencilstate: *mut Option<ID3D11DepthStencilState>, pstencilref: *mut u32);
    fn SOGetTargets(&self, numbuffers: u32, ppsotargets: *mut Option<ID3D11Buffer>);
    fn RSGetState(&self, pprasterizerstate: *mut Option<ID3D11RasterizerState>);
    fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT);
    fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT);
    fn HSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn HSGetShader(&self, pphullshader: *mut Option<ID3D11HullShader>, ppclassinstances: *mut Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn HSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn HSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn DSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn DSGetShader(&self, ppdomainshader: *mut Option<ID3D11DomainShader>, ppclassinstances: *mut Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn DSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn DSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn CSGetShaderResources(&self, startslot: u32, numviews: u32, ppshaderresourceviews: *mut Option<ID3D11ShaderResourceView>);
    fn CSGetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut Option<ID3D11UnorderedAccessView>);
    fn CSGetShader(&self, ppcomputeshader: *mut Option<ID3D11ComputeShader>, ppclassinstances: *mut Option<ID3D11ClassInstance>, pnumclassinstances: *mut u32);
    fn CSGetSamplers(&self, startslot: u32, numsamplers: u32, ppsamplers: *mut Option<ID3D11SamplerState>);
    fn CSGetConstantBuffers(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>);
    fn ClearState(&self);
    fn Flush(&self);
    fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE;
    fn GetContextFlags(&self) -> u32;
    fn FinishCommandList(&self, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut Option<ID3D11CommandList>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11DeviceContext {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>() -> ID3D11DeviceContext_Vtbl {
        unsafe extern "system" fn VSSetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSSetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSSetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSSetShader(this, windows_core::from_raw_borrowed(&ppixelshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn PSSetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSSetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvertexshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSSetShader(this, windows_core::from_raw_borrowed(&pvertexshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn DrawIndexed<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DrawIndexed(this, core::mem::transmute_copy(&indexcount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation))
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcount: u32, startvertexlocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::Draw(this, core::mem::transmute_copy(&vertexcount), core::mem::transmute_copy(&startvertexlocation))
        }
        unsafe extern "system" fn Map<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: *mut D3D11_MAPPED_SUBRESOURCE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::Map(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource), core::mem::transmute_copy(&maptype), core::mem::transmute_copy(&mapflags), core::mem::transmute_copy(&pmappedresource)).into()
        }
        unsafe extern "system" fn Unmap<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, subresource: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::Unmap(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&subresource))
        }
        unsafe extern "system" fn PSSetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IASetInputLayout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputlayout: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IASetInputLayout(this, windows_core::from_raw_borrowed(&pinputlayout))
        }
        unsafe extern "system" fn IASetVertexBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *const *mut core::ffi::c_void, pstrides: *const u32, poffsets: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IASetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IASetIndexBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IASetIndexBuffer(this, windows_core::from_raw_borrowed(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn DrawIndexedInstanced<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DrawIndexedInstanced(this, core::mem::transmute_copy(&indexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startindexlocation), core::mem::transmute_copy(&basevertexlocation), core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn DrawInstanced<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DrawInstanced(this, core::mem::transmute_copy(&vertexcountperinstance), core::mem::transmute_copy(&instancecount), core::mem::transmute_copy(&startvertexlocation), core::mem::transmute_copy(&startinstancelocation))
        }
        unsafe extern "system" fn GSSetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSSetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSSetShader(this, windows_core::from_raw_borrowed(&pshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn IASetPrimitiveTopology<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IASetPrimitiveTopology(this, core::mem::transmute_copy(&topology))
        }
        unsafe extern "system" fn VSSetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSSetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn Begin<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::Begin(this, windows_core::from_raw_borrowed(&pasync))
        }
        unsafe extern "system" fn End<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::End(this, windows_core::from_raw_borrowed(&pasync))
        }
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut core::ffi::c_void, pdata: *mut core::ffi::c_void, datasize: u32, getdataflags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GetData(this, windows_core::from_raw_borrowed(&pasync), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&getdataflags)).into()
        }
        unsafe extern "system" fn SetPredication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppredicate: *mut core::ffi::c_void, predicatevalue: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::SetPredication(this, windows_core::from_raw_borrowed(&ppredicate), core::mem::transmute_copy(&predicatevalue))
        }
        unsafe extern "system" fn GSSetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSSetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMSetRenderTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMSetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), windows_core::from_raw_borrowed(&pdepthstencilview))
        }
        unsafe extern "system" fn OMSetRenderTargetsAndUnorderedAccessViews<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrtvs: u32, pprendertargetviews: *const *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *const *mut core::ffi::c_void, puavinitialcounts: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMSetRenderTargetsAndUnorderedAccessViews(this, core::mem::transmute_copy(&numrtvs), core::mem::transmute_copy(&pprendertargetviews), windows_core::from_raw_borrowed(&pdepthstencilview), core::mem::transmute_copy(&uavstartslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews), core::mem::transmute_copy(&puavinitialcounts))
        }
        unsafe extern "system" fn OMSetBlendState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblendstate: *mut core::ffi::c_void, blendfactor: *const f32, samplemask: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMSetBlendState(this, windows_core::from_raw_borrowed(&pblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&samplemask))
        }
        unsafe extern "system" fn OMSetDepthStencilState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilstate: *mut core::ffi::c_void, stencilref: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMSetDepthStencilState(this, windows_core::from_raw_borrowed(&pdepthstencilstate), core::mem::transmute_copy(&stencilref))
        }
        unsafe extern "system" fn SOSetTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *const *mut core::ffi::c_void, poffsets: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::SOSetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn DrawAuto<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DrawAuto(this)
        }
        unsafe extern "system" fn DrawIndexedInstancedIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferforargs: *mut core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DrawIndexedInstancedIndirect(this, windows_core::from_raw_borrowed(&pbufferforargs), core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn DrawInstancedIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferforargs: *mut core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DrawInstancedIndirect(this, windows_core::from_raw_borrowed(&pbufferforargs), core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn Dispatch<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::Dispatch(this, core::mem::transmute_copy(&threadgroupcountx), core::mem::transmute_copy(&threadgroupcounty), core::mem::transmute_copy(&threadgroupcountz))
        }
        unsafe extern "system" fn DispatchIndirect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbufferforargs: *mut core::ffi::c_void, alignedbyteoffsetforargs: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DispatchIndirect(this, windows_core::from_raw_borrowed(&pbufferforargs), core::mem::transmute_copy(&alignedbyteoffsetforargs))
        }
        unsafe extern "system" fn RSSetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prasterizerstate: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::RSSetState(this, windows_core::from_raw_borrowed(&prasterizerstate))
        }
        unsafe extern "system" fn RSSetViewports<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviewports: u32, pviewports: *const D3D11_VIEWPORT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::RSSetViewports(this, core::mem::transmute_copy(&numviewports), core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSSetScissorRects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrects: u32, prects: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::RSSetScissorRects(this, core::mem::transmute_copy(&numrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn CopySubresourceRegion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CopySubresourceRegion(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox))
        }
        unsafe extern "system" fn CopyResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, psrcresource: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CopyResource(this, windows_core::from_raw_borrowed(&pdstresource), windows_core::from_raw_borrowed(&psrcresource))
        }
        unsafe extern "system" fn UpdateSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::UpdateSubresource(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch))
        }
        unsafe extern "system" fn CopyStructureCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstbuffer: *mut core::ffi::c_void, dstalignedbyteoffset: u32, psrcview: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CopyStructureCount(this, windows_core::from_raw_borrowed(&pdstbuffer), core::mem::transmute_copy(&dstalignedbyteoffset), windows_core::from_raw_borrowed(&psrcview))
        }
        unsafe extern "system" fn ClearRenderTargetView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendertargetview: *mut core::ffi::c_void, colorrgba: *const f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ClearRenderTargetView(this, windows_core::from_raw_borrowed(&prendertargetview), core::mem::transmute_copy(&colorrgba))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewUint<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punorderedaccessview: *mut core::ffi::c_void, values: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ClearUnorderedAccessViewUint(this, windows_core::from_raw_borrowed(&punorderedaccessview), core::mem::transmute_copy(&values))
        }
        unsafe extern "system" fn ClearUnorderedAccessViewFloat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punorderedaccessview: *mut core::ffi::c_void, values: *const f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ClearUnorderedAccessViewFloat(this, windows_core::from_raw_borrowed(&punorderedaccessview), core::mem::transmute_copy(&values))
        }
        unsafe extern "system" fn ClearDepthStencilView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdepthstencilview: *mut core::ffi::c_void, clearflags: u32, depth: f32, stencil: u8) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ClearDepthStencilView(this, windows_core::from_raw_borrowed(&pdepthstencilview), core::mem::transmute_copy(&clearflags), core::mem::transmute_copy(&depth), core::mem::transmute_copy(&stencil))
        }
        unsafe extern "system" fn GenerateMips<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshaderresourceview: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GenerateMips(this, windows_core::from_raw_borrowed(&pshaderresourceview))
        }
        unsafe extern "system" fn SetResourceMinLOD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, minlod: f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::SetResourceMinLOD(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&minlod))
        }
        unsafe extern "system" fn GetResourceMinLOD<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GetResourceMinLOD(this, windows_core::from_raw_borrowed(&presource))
        }
        unsafe extern "system" fn ResolveSubresource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ResolveSubresource(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&format))
        }
        unsafe extern "system" fn ExecuteCommandList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandlist: *mut core::ffi::c_void, restorecontextstate: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ExecuteCommandList(this, windows_core::from_raw_borrowed(&pcommandlist), core::mem::transmute_copy(&restorecontextstate))
        }
        unsafe extern "system" fn HSSetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn HSSetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phullshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSSetShader(this, windows_core::from_raw_borrowed(&phullshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn HSSetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn HSSetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn DSSetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn DSSetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomainshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSSetShader(this, windows_core::from_raw_borrowed(&pdomainshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn DSSetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn DSSetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn CSSetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSSetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn CSSetUnorderedAccessViews<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *const *mut core::ffi::c_void, puavinitialcounts: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSSetUnorderedAccessViews(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews), core::mem::transmute_copy(&puavinitialcounts))
        }
        unsafe extern "system" fn CSSetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomputeshader: *mut core::ffi::c_void, ppclassinstances: *const *mut core::ffi::c_void, numclassinstances: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSSetShader(this, windows_core::from_raw_borrowed(&pcomputeshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&numclassinstances))
        }
        unsafe extern "system" fn CSSetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSSetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn CSSetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSSetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn VSGetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn PSGetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn PSGetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppixelshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSGetShader(this, core::mem::transmute_copy(&pppixelshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn PSGetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn VSGetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvertexshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSGetShader(this, core::mem::transmute_copy(&ppvertexshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn PSGetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::PSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn IAGetInputLayout<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinputlayout: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IAGetInputLayout(this, core::mem::transmute_copy(&ppinputlayout))
        }
        unsafe extern "system" fn IAGetVertexBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppvertexbuffers: *mut *mut core::ffi::c_void, pstrides: *mut u32, poffsets: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IAGetVertexBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppvertexbuffers), core::mem::transmute_copy(&pstrides), core::mem::transmute_copy(&poffsets))
        }
        unsafe extern "system" fn IAGetIndexBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexbuffer: *mut *mut core::ffi::c_void, format: *mut super::Dxgi::Common::DXGI_FORMAT, offset: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IAGetIndexBuffer(this, core::mem::transmute_copy(&pindexbuffer), core::mem::transmute_copy(&format), core::mem::transmute_copy(&offset))
        }
        unsafe extern "system" fn GSGetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn GSGetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgeometryshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSGetShader(this, core::mem::transmute_copy(&ppgeometryshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn IAGetPrimitiveTopology<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::IAGetPrimitiveTopology(this, core::mem::transmute_copy(&ptopology))
        }
        unsafe extern "system" fn VSGetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn VSGetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::VSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn GetPredication<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppredicate: *mut *mut core::ffi::c_void, ppredicatevalue: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GetPredication(this, core::mem::transmute_copy(&pppredicate), core::mem::transmute_copy(&ppredicatevalue))
        }
        unsafe extern "system" fn GSGetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn GSGetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn OMGetRenderTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numviews: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMGetRenderTargets(this, core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview))
        }
        unsafe extern "system" fn OMGetRenderTargetsAndUnorderedAccessViews<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numrtvs: u32, pprendertargetviews: *mut *mut core::ffi::c_void, ppdepthstencilview: *mut *mut core::ffi::c_void, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMGetRenderTargetsAndUnorderedAccessViews(this, core::mem::transmute_copy(&numrtvs), core::mem::transmute_copy(&pprendertargetviews), core::mem::transmute_copy(&ppdepthstencilview), core::mem::transmute_copy(&uavstartslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews))
        }
        unsafe extern "system" fn OMGetBlendState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppblendstate: *mut *mut core::ffi::c_void, blendfactor: *mut f32, psamplemask: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMGetBlendState(this, core::mem::transmute_copy(&ppblendstate), core::mem::transmute_copy(&blendfactor), core::mem::transmute_copy(&psamplemask))
        }
        unsafe extern "system" fn OMGetDepthStencilState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdepthstencilstate: *mut *mut core::ffi::c_void, pstencilref: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::OMGetDepthStencilState(this, core::mem::transmute_copy(&ppdepthstencilstate), core::mem::transmute_copy(&pstencilref))
        }
        unsafe extern "system" fn SOGetTargets<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numbuffers: u32, ppsotargets: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::SOGetTargets(this, core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppsotargets))
        }
        unsafe extern "system" fn RSGetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprasterizerstate: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::RSGetState(this, core::mem::transmute_copy(&pprasterizerstate))
        }
        unsafe extern "system" fn RSGetViewports<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumviewports: *mut u32, pviewports: *mut D3D11_VIEWPORT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::RSGetViewports(this, core::mem::transmute_copy(&pnumviewports), core::mem::transmute_copy(&pviewports))
        }
        unsafe extern "system" fn RSGetScissorRects<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumrects: *mut u32, prects: *mut super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::RSGetScissorRects(this, core::mem::transmute_copy(&pnumrects), core::mem::transmute_copy(&prects))
        }
        unsafe extern "system" fn HSGetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn HSGetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphullshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSGetShader(this, core::mem::transmute_copy(&pphullshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn HSGetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn HSGetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::HSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn DSGetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn DSGetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdomainshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSGetShader(this, core::mem::transmute_copy(&ppdomainshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn DSGetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn DSGetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::DSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn CSGetShaderResources<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numviews: u32, ppshaderresourceviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSGetShaderResources(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numviews), core::mem::transmute_copy(&ppshaderresourceviews))
        }
        unsafe extern "system" fn CSGetUnorderedAccessViews<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numuavs: u32, ppunorderedaccessviews: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSGetUnorderedAccessViews(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numuavs), core::mem::transmute_copy(&ppunorderedaccessviews))
        }
        unsafe extern "system" fn CSGetShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomputeshader: *mut *mut core::ffi::c_void, ppclassinstances: *mut *mut core::ffi::c_void, pnumclassinstances: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSGetShader(this, core::mem::transmute_copy(&ppcomputeshader), core::mem::transmute_copy(&ppclassinstances), core::mem::transmute_copy(&pnumclassinstances))
        }
        unsafe extern "system" fn CSGetSamplers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numsamplers: u32, ppsamplers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSGetSamplers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numsamplers), core::mem::transmute_copy(&ppsamplers))
        }
        unsafe extern "system" fn CSGetConstantBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::CSGetConstantBuffers(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers))
        }
        unsafe extern "system" fn ClearState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::ClearState(this)
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::Flush(this)
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GetType(this)
        }
        unsafe extern "system" fn GetContextFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::GetContextFlags(this)
        }
        unsafe extern "system" fn FinishCommandList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restoredeferredcontextstate: super::super::Foundation::BOOL, ppcommandlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext_Impl::FinishCommandList(this, core::mem::transmute_copy(&restoredeferredcontextstate), core::mem::transmute_copy(&ppcommandlist)).into()
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            VSSetConstantBuffers: VSSetConstantBuffers::<Identity, Impl, OFFSET>,
            PSSetShaderResources: PSSetShaderResources::<Identity, Impl, OFFSET>,
            PSSetShader: PSSetShader::<Identity, Impl, OFFSET>,
            PSSetSamplers: PSSetSamplers::<Identity, Impl, OFFSET>,
            VSSetShader: VSSetShader::<Identity, Impl, OFFSET>,
            DrawIndexed: DrawIndexed::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers: PSSetConstantBuffers::<Identity, Impl, OFFSET>,
            IASetInputLayout: IASetInputLayout::<Identity, Impl, OFFSET>,
            IASetVertexBuffers: IASetVertexBuffers::<Identity, Impl, OFFSET>,
            IASetIndexBuffer: IASetIndexBuffer::<Identity, Impl, OFFSET>,
            DrawIndexedInstanced: DrawIndexedInstanced::<Identity, Impl, OFFSET>,
            DrawInstanced: DrawInstanced::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers: GSSetConstantBuffers::<Identity, Impl, OFFSET>,
            GSSetShader: GSSetShader::<Identity, Impl, OFFSET>,
            IASetPrimitiveTopology: IASetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSSetShaderResources: VSSetShaderResources::<Identity, Impl, OFFSET>,
            VSSetSamplers: VSSetSamplers::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetPredication: SetPredication::<Identity, Impl, OFFSET>,
            GSSetShaderResources: GSSetShaderResources::<Identity, Impl, OFFSET>,
            GSSetSamplers: GSSetSamplers::<Identity, Impl, OFFSET>,
            OMSetRenderTargets: OMSetRenderTargets::<Identity, Impl, OFFSET>,
            OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews::<Identity, Impl, OFFSET>,
            OMSetBlendState: OMSetBlendState::<Identity, Impl, OFFSET>,
            OMSetDepthStencilState: OMSetDepthStencilState::<Identity, Impl, OFFSET>,
            SOSetTargets: SOSetTargets::<Identity, Impl, OFFSET>,
            DrawAuto: DrawAuto::<Identity, Impl, OFFSET>,
            DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect::<Identity, Impl, OFFSET>,
            DrawInstancedIndirect: DrawInstancedIndirect::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
            DispatchIndirect: DispatchIndirect::<Identity, Impl, OFFSET>,
            RSSetState: RSSetState::<Identity, Impl, OFFSET>,
            RSSetViewports: RSSetViewports::<Identity, Impl, OFFSET>,
            RSSetScissorRects: RSSetScissorRects::<Identity, Impl, OFFSET>,
            CopySubresourceRegion: CopySubresourceRegion::<Identity, Impl, OFFSET>,
            CopyResource: CopyResource::<Identity, Impl, OFFSET>,
            UpdateSubresource: UpdateSubresource::<Identity, Impl, OFFSET>,
            CopyStructureCount: CopyStructureCount::<Identity, Impl, OFFSET>,
            ClearRenderTargetView: ClearRenderTargetView::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint::<Identity, Impl, OFFSET>,
            ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat::<Identity, Impl, OFFSET>,
            ClearDepthStencilView: ClearDepthStencilView::<Identity, Impl, OFFSET>,
            GenerateMips: GenerateMips::<Identity, Impl, OFFSET>,
            SetResourceMinLOD: SetResourceMinLOD::<Identity, Impl, OFFSET>,
            GetResourceMinLOD: GetResourceMinLOD::<Identity, Impl, OFFSET>,
            ResolveSubresource: ResolveSubresource::<Identity, Impl, OFFSET>,
            ExecuteCommandList: ExecuteCommandList::<Identity, Impl, OFFSET>,
            HSSetShaderResources: HSSetShaderResources::<Identity, Impl, OFFSET>,
            HSSetShader: HSSetShader::<Identity, Impl, OFFSET>,
            HSSetSamplers: HSSetSamplers::<Identity, Impl, OFFSET>,
            HSSetConstantBuffers: HSSetConstantBuffers::<Identity, Impl, OFFSET>,
            DSSetShaderResources: DSSetShaderResources::<Identity, Impl, OFFSET>,
            DSSetShader: DSSetShader::<Identity, Impl, OFFSET>,
            DSSetSamplers: DSSetSamplers::<Identity, Impl, OFFSET>,
            DSSetConstantBuffers: DSSetConstantBuffers::<Identity, Impl, OFFSET>,
            CSSetShaderResources: CSSetShaderResources::<Identity, Impl, OFFSET>,
            CSSetUnorderedAccessViews: CSSetUnorderedAccessViews::<Identity, Impl, OFFSET>,
            CSSetShader: CSSetShader::<Identity, Impl, OFFSET>,
            CSSetSamplers: CSSetSamplers::<Identity, Impl, OFFSET>,
            CSSetConstantBuffers: CSSetConstantBuffers::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers: VSGetConstantBuffers::<Identity, Impl, OFFSET>,
            PSGetShaderResources: PSGetShaderResources::<Identity, Impl, OFFSET>,
            PSGetShader: PSGetShader::<Identity, Impl, OFFSET>,
            PSGetSamplers: PSGetSamplers::<Identity, Impl, OFFSET>,
            VSGetShader: VSGetShader::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers: PSGetConstantBuffers::<Identity, Impl, OFFSET>,
            IAGetInputLayout: IAGetInputLayout::<Identity, Impl, OFFSET>,
            IAGetVertexBuffers: IAGetVertexBuffers::<Identity, Impl, OFFSET>,
            IAGetIndexBuffer: IAGetIndexBuffer::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers: GSGetConstantBuffers::<Identity, Impl, OFFSET>,
            GSGetShader: GSGetShader::<Identity, Impl, OFFSET>,
            IAGetPrimitiveTopology: IAGetPrimitiveTopology::<Identity, Impl, OFFSET>,
            VSGetShaderResources: VSGetShaderResources::<Identity, Impl, OFFSET>,
            VSGetSamplers: VSGetSamplers::<Identity, Impl, OFFSET>,
            GetPredication: GetPredication::<Identity, Impl, OFFSET>,
            GSGetShaderResources: GSGetShaderResources::<Identity, Impl, OFFSET>,
            GSGetSamplers: GSGetSamplers::<Identity, Impl, OFFSET>,
            OMGetRenderTargets: OMGetRenderTargets::<Identity, Impl, OFFSET>,
            OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews::<Identity, Impl, OFFSET>,
            OMGetBlendState: OMGetBlendState::<Identity, Impl, OFFSET>,
            OMGetDepthStencilState: OMGetDepthStencilState::<Identity, Impl, OFFSET>,
            SOGetTargets: SOGetTargets::<Identity, Impl, OFFSET>,
            RSGetState: RSGetState::<Identity, Impl, OFFSET>,
            RSGetViewports: RSGetViewports::<Identity, Impl, OFFSET>,
            RSGetScissorRects: RSGetScissorRects::<Identity, Impl, OFFSET>,
            HSGetShaderResources: HSGetShaderResources::<Identity, Impl, OFFSET>,
            HSGetShader: HSGetShader::<Identity, Impl, OFFSET>,
            HSGetSamplers: HSGetSamplers::<Identity, Impl, OFFSET>,
            HSGetConstantBuffers: HSGetConstantBuffers::<Identity, Impl, OFFSET>,
            DSGetShaderResources: DSGetShaderResources::<Identity, Impl, OFFSET>,
            DSGetShader: DSGetShader::<Identity, Impl, OFFSET>,
            DSGetSamplers: DSGetSamplers::<Identity, Impl, OFFSET>,
            DSGetConstantBuffers: DSGetConstantBuffers::<Identity, Impl, OFFSET>,
            CSGetShaderResources: CSGetShaderResources::<Identity, Impl, OFFSET>,
            CSGetUnorderedAccessViews: CSGetUnorderedAccessViews::<Identity, Impl, OFFSET>,
            CSGetShader: CSGetShader::<Identity, Impl, OFFSET>,
            CSGetSamplers: CSGetSamplers::<Identity, Impl, OFFSET>,
            CSGetConstantBuffers: CSGetConstantBuffers::<Identity, Impl, OFFSET>,
            ClearState: ClearState::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetContextFlags: GetContextFlags::<Identity, Impl, OFFSET>,
            FinishCommandList: FinishCommandList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext1_Impl: Sized + ID3D11DeviceContext_Impl {
    fn CopySubresourceRegion1(&self, pdstresource: Option<&ID3D11Resource>, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: Option<&ID3D11Resource>, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32);
    fn UpdateSubresource1(&self, pdstresource: Option<&ID3D11Resource>, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32);
    fn DiscardResource(&self, presource: Option<&ID3D11Resource>);
    fn DiscardView(&self, presourceview: Option<&ID3D11View>);
    fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *const Option<ID3D11Buffer>, pfirstconstant: *const u32, pnumconstants: *const u32);
    fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut Option<ID3D11Buffer>, pfirstconstant: *mut u32, pnumconstants: *mut u32);
    fn SwapDeviceContextState(&self, pstate: Option<&ID3DDeviceContextState>, pppreviousstate: *mut Option<ID3DDeviceContextState>);
    fn ClearView(&self, pview: Option<&ID3D11View>, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32);
    fn DiscardView1(&self, presourceview: Option<&ID3D11View>, prects: *const super::super::Foundation::RECT, numrects: u32);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11DeviceContext1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>() -> ID3D11DeviceContext1_Vtbl {
        unsafe extern "system" fn CopySubresourceRegion1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: *mut core::ffi::c_void, srcsubresource: u32, psrcbox: *const D3D11_BOX, copyflags: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::CopySubresourceRegion1(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&dstx), core::mem::transmute_copy(&dsty), core::mem::transmute_copy(&dstz), windows_core::from_raw_borrowed(&psrcresource), core::mem::transmute_copy(&srcsubresource), core::mem::transmute_copy(&psrcbox), core::mem::transmute_copy(&copyflags))
        }
        unsafe extern "system" fn UpdateSubresource1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstresource: *mut core::ffi::c_void, dstsubresource: u32, pdstbox: *const D3D11_BOX, psrcdata: *const core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::UpdateSubresource1(this, windows_core::from_raw_borrowed(&pdstresource), core::mem::transmute_copy(&dstsubresource), core::mem::transmute_copy(&pdstbox), core::mem::transmute_copy(&psrcdata), core::mem::transmute_copy(&srcrowpitch), core::mem::transmute_copy(&srcdepthpitch), core::mem::transmute_copy(&copyflags))
        }
        unsafe extern "system" fn DiscardResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::DiscardResource(this, windows_core::from_raw_borrowed(&presource))
        }
        unsafe extern "system" fn DiscardView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceview: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::DiscardView(this, windows_core::from_raw_borrowed(&presourceview))
        }
        unsafe extern "system" fn VSSetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::VSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn HSSetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::HSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn DSSetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::DSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn GSSetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::GSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn PSSetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::PSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn CSSetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *const *mut core::ffi::c_void, pfirstconstant: *const u32, pnumconstants: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::CSSetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn VSGetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::VSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn HSGetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::HSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn DSGetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::DSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn GSGetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::GSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn PSGetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::PSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn CSGetConstantBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startslot: u32, numbuffers: u32, ppconstantbuffers: *mut *mut core::ffi::c_void, pfirstconstant: *mut u32, pnumconstants: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::CSGetConstantBuffers1(this, core::mem::transmute_copy(&startslot), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&ppconstantbuffers), core::mem::transmute_copy(&pfirstconstant), core::mem::transmute_copy(&pnumconstants))
        }
        unsafe extern "system" fn SwapDeviceContextState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void, pppreviousstate: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::SwapDeviceContextState(this, windows_core::from_raw_borrowed(&pstate), core::mem::transmute_copy(&pppreviousstate))
        }
        unsafe extern "system" fn ClearView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, color: *const f32, prect: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::ClearView(this, windows_core::from_raw_borrowed(&pview), core::mem::transmute_copy(&color), core::mem::transmute_copy(&prect), core::mem::transmute_copy(&numrects))
        }
        unsafe extern "system" fn DiscardView1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourceview: *mut core::ffi::c_void, prects: *const super::super::Foundation::RECT, numrects: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext1_Impl::DiscardView1(this, windows_core::from_raw_borrowed(&presourceview), core::mem::transmute_copy(&prects), core::mem::transmute_copy(&numrects))
        }
        Self {
            base__: ID3D11DeviceContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            CopySubresourceRegion1: CopySubresourceRegion1::<Identity, Impl, OFFSET>,
            UpdateSubresource1: UpdateSubresource1::<Identity, Impl, OFFSET>,
            DiscardResource: DiscardResource::<Identity, Impl, OFFSET>,
            DiscardView: DiscardView::<Identity, Impl, OFFSET>,
            VSSetConstantBuffers1: VSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            HSSetConstantBuffers1: HSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            DSSetConstantBuffers1: DSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            GSSetConstantBuffers1: GSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            PSSetConstantBuffers1: PSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            CSSetConstantBuffers1: CSSetConstantBuffers1::<Identity, Impl, OFFSET>,
            VSGetConstantBuffers1: VSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            HSGetConstantBuffers1: HSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            DSGetConstantBuffers1: DSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            GSGetConstantBuffers1: GSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            PSGetConstantBuffers1: PSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            CSGetConstantBuffers1: CSGetConstantBuffers1::<Identity, Impl, OFFSET>,
            SwapDeviceContextState: SwapDeviceContextState::<Identity, Impl, OFFSET>,
            ClearView: ClearView::<Identity, Impl, OFFSET>,
            DiscardView1: DiscardView1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext2_Impl: Sized + ID3D11DeviceContext1_Impl {
    fn UpdateTileMappings(&self, ptiledresource: Option<&ID3D11Resource>, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: Option<&ID3D11Buffer>, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> windows_core::Result<()>;
    fn CopyTileMappings(&self, pdesttiledresource: Option<&ID3D11Resource>, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: Option<&ID3D11Resource>, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> windows_core::Result<()>;
    fn CopyTiles(&self, ptiledresource: Option<&ID3D11Resource>, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: Option<&ID3D11Buffer>, bufferstartoffsetinbytes: u64, flags: u32);
    fn UpdateTiles(&self, pdesttiledresource: Option<&ID3D11Resource>, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const core::ffi::c_void, flags: u32);
    fn ResizeTilePool(&self, ptilepool: Option<&ID3D11Buffer>, newsizeinbytes: u64) -> windows_core::Result<()>;
    fn TiledResourceBarrier(&self, ptiledresourceorviewaccessbeforebarrier: Option<&ID3D11DeviceChild>, ptiledresourceorviewaccessafterbarrier: Option<&ID3D11DeviceChild>);
    fn IsAnnotationEnabled(&self) -> super::super::Foundation::BOOL;
    fn SetMarkerInt(&self, plabel: &windows_core::PCWSTR, data: i32);
    fn BeginEventInt(&self, plabel: &windows_core::PCWSTR, data: i32);
    fn EndEvent(&self);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11DeviceContext2 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>() -> ID3D11DeviceContext2_Vtbl {
        unsafe extern "system" fn UpdateTileMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: *mut core::ffi::c_void, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::UpdateTileMappings(
                this,
                windows_core::from_raw_borrowed(&ptiledresource),
                core::mem::transmute_copy(&numtiledresourceregions),
                core::mem::transmute_copy(&ptiledresourceregionstartcoordinates),
                core::mem::transmute_copy(&ptiledresourceregionsizes),
                windows_core::from_raw_borrowed(&ptilepool),
                core::mem::transmute_copy(&numranges),
                core::mem::transmute_copy(&prangeflags),
                core::mem::transmute_copy(&ptilepoolstartoffsets),
                core::mem::transmute_copy(&prangetilecounts),
                core::mem::transmute_copy(&flags),
            )
            .into()
        }
        unsafe extern "system" fn CopyTileMappings<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesttiledresource: *mut core::ffi::c_void, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: *mut core::ffi::c_void, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::CopyTileMappings(this, windows_core::from_raw_borrowed(&pdesttiledresource), core::mem::transmute_copy(&pdestregionstartcoordinate), windows_core::from_raw_borrowed(&psourcetiledresource), core::mem::transmute_copy(&psourceregionstartcoordinate), core::mem::transmute_copy(&ptileregionsize), core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn CopyTiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: *mut core::ffi::c_void, bufferstartoffsetinbytes: u64, flags: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::CopyTiles(this, windows_core::from_raw_borrowed(&ptiledresource), core::mem::transmute_copy(&ptileregionstartcoordinate), core::mem::transmute_copy(&ptileregionsize), windows_core::from_raw_borrowed(&pbuffer), core::mem::transmute_copy(&bufferstartoffsetinbytes), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn UpdateTiles<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesttiledresource: *mut core::ffi::c_void, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const core::ffi::c_void, flags: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::UpdateTiles(this, windows_core::from_raw_borrowed(&pdesttiledresource), core::mem::transmute_copy(&pdesttileregionstartcoordinate), core::mem::transmute_copy(&pdesttileregionsize), core::mem::transmute_copy(&psourcetiledata), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn ResizeTilePool<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptilepool: *mut core::ffi::c_void, newsizeinbytes: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::ResizeTilePool(this, windows_core::from_raw_borrowed(&ptilepool), core::mem::transmute_copy(&newsizeinbytes)).into()
        }
        unsafe extern "system" fn TiledResourceBarrier<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: *mut core::ffi::c_void, ptiledresourceorviewaccessafterbarrier: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::TiledResourceBarrier(this, windows_core::from_raw_borrowed(&ptiledresourceorviewaccessbeforebarrier), windows_core::from_raw_borrowed(&ptiledresourceorviewaccessafterbarrier))
        }
        unsafe extern "system" fn IsAnnotationEnabled<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::IsAnnotationEnabled(this)
        }
        unsafe extern "system" fn SetMarkerInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plabel: windows_core::PCWSTR, data: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::SetMarkerInt(this, core::mem::transmute(&plabel), core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn BeginEventInt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plabel: windows_core::PCWSTR, data: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::BeginEventInt(this, core::mem::transmute(&plabel), core::mem::transmute_copy(&data))
        }
        unsafe extern "system" fn EndEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext2_Impl::EndEvent(this)
        }
        Self {
            base__: ID3D11DeviceContext1_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Identity, Impl, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, Impl, OFFSET>,
            CopyTiles: CopyTiles::<Identity, Impl, OFFSET>,
            UpdateTiles: UpdateTiles::<Identity, Impl, OFFSET>,
            ResizeTilePool: ResizeTilePool::<Identity, Impl, OFFSET>,
            TiledResourceBarrier: TiledResourceBarrier::<Identity, Impl, OFFSET>,
            IsAnnotationEnabled: IsAnnotationEnabled::<Identity, Impl, OFFSET>,
            SetMarkerInt: SetMarkerInt::<Identity, Impl, OFFSET>,
            BeginEventInt: BeginEventInt::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext2 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext3_Impl: Sized + ID3D11DeviceContext2_Impl {
    fn Flush1(&self, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE);
    fn SetHardwareProtectionState(&self, hwprotectionenable: super::super::Foundation::BOOL);
    fn GetHardwareProtectionState(&self, phwprotectionenable: *mut super::super::Foundation::BOOL);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11DeviceContext3 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>() -> ID3D11DeviceContext3_Vtbl {
        unsafe extern "system" fn Flush1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contexttype: D3D11_CONTEXT_TYPE, hevent: super::super::Foundation::HANDLE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext3_Impl::Flush1(this, core::mem::transmute_copy(&contexttype), core::mem::transmute_copy(&hevent))
        }
        unsafe extern "system" fn SetHardwareProtectionState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwprotectionenable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext3_Impl::SetHardwareProtectionState(this, core::mem::transmute_copy(&hwprotectionenable))
        }
        unsafe extern "system" fn GetHardwareProtectionState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwprotectionenable: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext3_Impl::GetHardwareProtectionState(this, core::mem::transmute_copy(&phwprotectionenable))
        }
        Self {
            base__: ID3D11DeviceContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            Flush1: Flush1::<Identity, Impl, OFFSET>,
            SetHardwareProtectionState: SetHardwareProtectionState::<Identity, Impl, OFFSET>,
            GetHardwareProtectionState: GetHardwareProtectionState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext3 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11DeviceContext4_Impl: Sized + ID3D11DeviceContext3_Impl {
    fn Signal(&self, pfence: Option<&ID3D11Fence>, value: u64) -> windows_core::Result<()>;
    fn Wait(&self, pfence: Option<&ID3D11Fence>, value: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11DeviceContext4 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11DeviceContext4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>() -> ID3D11DeviceContext4_Vtbl {
        unsafe extern "system" fn Signal<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext4_Impl::Signal(this, windows_core::from_raw_borrowed(&pfence), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Wait<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfence: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11DeviceContext4_Impl::Wait(this, windows_core::from_raw_borrowed(&pfence), core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ID3D11DeviceContext3_Vtbl::new::<Identity, Impl, OFFSET>(),
            Signal: Signal::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext4 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext2 as windows_core::Interface>::IID || iid == &<ID3D11DeviceContext3 as windows_core::Interface>::IID
    }
}
pub trait ID3D11DomainShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11DomainShader {}
impl ID3D11DomainShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11DomainShader_Impl, const OFFSET: isize>() -> ID3D11DomainShader_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DomainShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security")]
pub trait ID3D11Fence_Impl: Sized + ID3D11DeviceChild_Impl {
    fn CreateSharedHandle(&self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetCompletedValue(&self) -> u64;
    fn SetEventOnCompletion(&self, value: u64, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Security")]
impl windows_core::RuntimeName for ID3D11Fence {}
#[cfg(feature = "Win32_Security")]
impl ID3D11Fence_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: isize>() -> ID3D11Fence_Vtbl {
        unsafe extern "system" fn CreateSharedHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: windows_core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Fence_Impl::CreateSharedHandle(this, core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&dwaccess), core::mem::transmute(&lpname)) {
                Ok(ok__) => {
                    core::ptr::write(phandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompletedValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Fence_Impl::GetCompletedValue(this)
        }
        unsafe extern "system" fn SetEventOnCompletion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Fence_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, hevent: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Fence_Impl::SetEventOnCompletion(this, core::mem::transmute_copy(&value), core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
            GetCompletedValue: GetCompletedValue::<Identity, Impl, OFFSET>,
            SetEventOnCompletion: SetEventOnCompletion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Fence as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionLinkingGraph_Impl: Sized {
    fn CreateModuleInstance(&self, ppmoduleinstance: *mut Option<ID3D11ModuleInstance>, pperrorbuffer: *mut Option<super::Direct3D::ID3DBlob>) -> windows_core::Result<()>;
    fn SetInputSignature(&self, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32) -> windows_core::Result<ID3D11LinkingNode>;
    fn SetOutputSignature(&self, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32) -> windows_core::Result<ID3D11LinkingNode>;
    fn CallFunction(&self, pmoduleinstancenamespace: &windows_core::PCSTR, pmodulewithfunctionprototype: Option<&ID3D11Module>, pfunctionname: &windows_core::PCSTR) -> windows_core::Result<ID3D11LinkingNode>;
    fn PassValue(&self, psrcnode: Option<&ID3D11LinkingNode>, srcparameterindex: i32, pdstnode: Option<&ID3D11LinkingNode>, dstparameterindex: i32) -> windows_core::Result<()>;
    fn PassValueWithSwizzle(&self, psrcnode: Option<&ID3D11LinkingNode>, srcparameterindex: i32, psrcswizzle: &windows_core::PCSTR, pdstnode: Option<&ID3D11LinkingNode>, dstparameterindex: i32, pdstswizzle: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn GetLastError(&self, pperrorbuffer: *mut Option<super::Direct3D::ID3DBlob>) -> windows_core::Result<()>;
    fn GenerateHlsl(&self, uflags: u32) -> windows_core::Result<super::Direct3D::ID3DBlob>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D11FunctionLinkingGraph {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionLinkingGraph_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>() -> ID3D11FunctionLinkingGraph_Vtbl {
        unsafe extern "system" fn CreateModuleInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmoduleinstance: *mut *mut core::ffi::c_void, pperrorbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11FunctionLinkingGraph_Impl::CreateModuleInstance(this, core::mem::transmute_copy(&ppmoduleinstance), core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn SetInputSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputparameters: *const D3D11_PARAMETER_DESC, cinputparameters: u32, ppinputnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11FunctionLinkingGraph_Impl::SetInputSignature(this, core::mem::transmute_copy(&pinputparameters), core::mem::transmute_copy(&cinputparameters)) {
                Ok(ok__) => {
                    core::ptr::write(ppinputnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputSignature<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputparameters: *const D3D11_PARAMETER_DESC, coutputparameters: u32, ppoutputnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11FunctionLinkingGraph_Impl::SetOutputSignature(this, core::mem::transmute_copy(&poutputparameters), core::mem::transmute_copy(&coutputparameters)) {
                Ok(ok__) => {
                    core::ptr::write(ppoutputnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallFunction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmoduleinstancenamespace: windows_core::PCSTR, pmodulewithfunctionprototype: *mut core::ffi::c_void, pfunctionname: windows_core::PCSTR, ppcallnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11FunctionLinkingGraph_Impl::CallFunction(this, core::mem::transmute(&pmoduleinstancenamespace), windows_core::from_raw_borrowed(&pmodulewithfunctionprototype), core::mem::transmute(&pfunctionname)) {
                Ok(ok__) => {
                    core::ptr::write(ppcallnode, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PassValue<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcnode: *mut core::ffi::c_void, srcparameterindex: i32, pdstnode: *mut core::ffi::c_void, dstparameterindex: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11FunctionLinkingGraph_Impl::PassValue(this, windows_core::from_raw_borrowed(&psrcnode), core::mem::transmute_copy(&srcparameterindex), windows_core::from_raw_borrowed(&pdstnode), core::mem::transmute_copy(&dstparameterindex)).into()
        }
        unsafe extern "system" fn PassValueWithSwizzle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcnode: *mut core::ffi::c_void, srcparameterindex: i32, psrcswizzle: windows_core::PCSTR, pdstnode: *mut core::ffi::c_void, dstparameterindex: i32, pdstswizzle: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11FunctionLinkingGraph_Impl::PassValueWithSwizzle(this, windows_core::from_raw_borrowed(&psrcnode), core::mem::transmute_copy(&srcparameterindex), core::mem::transmute(&psrcswizzle), windows_core::from_raw_borrowed(&pdstnode), core::mem::transmute_copy(&dstparameterindex), core::mem::transmute(&pdstswizzle)).into()
        }
        unsafe extern "system" fn GetLastError<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperrorbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11FunctionLinkingGraph_Impl::GetLastError(this, core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn GenerateHlsl<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11FunctionLinkingGraph_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11FunctionLinkingGraph_Impl::GenerateHlsl(this, core::mem::transmute_copy(&uflags)) {
                Ok(ok__) => {
                    core::ptr::write(ppbuffer, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateModuleInstance: CreateModuleInstance::<Identity, Impl, OFFSET>,
            SetInputSignature: SetInputSignature::<Identity, Impl, OFFSET>,
            SetOutputSignature: SetOutputSignature::<Identity, Impl, OFFSET>,
            CallFunction: CallFunction::<Identity, Impl, OFFSET>,
            PassValue: PassValue::<Identity, Impl, OFFSET>,
            PassValueWithSwizzle: PassValueWithSwizzle::<Identity, Impl, OFFSET>,
            GetLastError: GetLastError::<Identity, Impl, OFFSET>,
            GenerateHlsl: GenerateHlsl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11FunctionLinkingGraph as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionParameterReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_PARAMETER_DESC) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionParameterReflection_Vtbl {
    pub const fn new<Impl: ID3D11FunctionParameterReflection_Impl>() -> ID3D11FunctionParameterReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionParameterReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionParameterReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        Self { GetDesc: GetDesc::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11FunctionParameterReflection_ImplVtbl<T: ID3D11FunctionParameterReflection_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11FunctionParameterReflection_Impl> ID3D11FunctionParameterReflection_ImplVtbl<T> {
    const VTABLE: ID3D11FunctionParameterReflection_Vtbl = ID3D11FunctionParameterReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionParameterReflection {
    pub fn new<'a, T: ID3D11FunctionParameterReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11FunctionParameterReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11FunctionReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_FUNCTION_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, bufferindex: u32) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetFunctionParameter(&self, parameterindex: i32) -> Option<ID3D11FunctionParameterReflection>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionReflection_Vtbl {
    pub const fn new<Impl: ID3D11FunctionReflection_Impl>() -> ID3D11FunctionReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_FUNCTION_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, bufferindex: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&bufferindex))
        }
        unsafe extern "system" fn GetConstantBufferByName<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetFunctionParameter<Impl: ID3D11FunctionReflection_Impl>(this: *mut core::ffi::c_void, parameterindex: i32) -> Option<ID3D11FunctionParameterReflection> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11FunctionReflection_Impl::GetFunctionParameter(this, core::mem::transmute_copy(&parameterindex))
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Impl>,
            GetConstantBufferByName: GetConstantBufferByName::<Impl>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Impl>,
            GetVariableByName: GetVariableByName::<Impl>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Impl>,
            GetFunctionParameter: GetFunctionParameter::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11FunctionReflection_ImplVtbl<T: ID3D11FunctionReflection_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11FunctionReflection_Impl> ID3D11FunctionReflection_ImplVtbl<T> {
    const VTABLE: ID3D11FunctionReflection_Vtbl = ID3D11FunctionReflection_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11FunctionReflection {
    pub fn new<'a, T: ID3D11FunctionReflection_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11FunctionReflection_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D11GeometryShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11GeometryShader {}
impl ID3D11GeometryShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11GeometryShader_Impl, const OFFSET: isize>() -> ID3D11GeometryShader_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11GeometryShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11HullShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11HullShader {}
impl ID3D11HullShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11HullShader_Impl, const OFFSET: isize>() -> ID3D11HullShader_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11HullShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11InfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self);
    fn GetMessage(&self, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumMessagesAllowedByStorageFilter(&self) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self) -> u64;
    fn GetNumStoredMessages(&self) -> u64;
    fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64;
    fn GetMessageCountLimit(&self) -> u64;
    fn AddStorageFilterEntries(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self);
    fn PushEmptyStorageFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self);
    fn GetStorageFilterStackSize(&self) -> u32;
    fn AddRetrievalFilterEntries(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self);
    fn PushEmptyRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self);
    fn GetRetrievalFilterStackSize(&self) -> u32;
    fn AddMessage(&self, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: D3D11_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D11InfoQueue {}
impl ID3D11InfoQueue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>() -> ID3D11InfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagecountlimit: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::ClearStoredMessages(this)
        }
        unsafe extern "system" fn GetMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messageindex: u64, pmessage: *mut D3D11_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetMessage(this, core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this)
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this)
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetNumStoredMessages(this)
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilter(this)
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this)
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetMessageCountLimit(this)
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetStorageFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::ClearStorageFilter(this)
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PushEmptyStorageFilter(this).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PushCopyOfStorageFilter(this).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PushStorageFilter(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PopStorageFilter(this)
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetStorageFilterStackSize(this)
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *mut D3D11_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::ClearRetrievalFilter(this)
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PushEmptyRetrievalFilter(this).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PushCopyOfRetrievalFilter(this).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfilter: *const D3D11_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::PopRetrievalFilter(this)
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetRetrievalFilterStackSize(this)
        }
        unsafe extern "system" fn AddMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, severity: D3D11_MESSAGE_SEVERITY, id: D3D11_MESSAGE_ID, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::AddMessage(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D11_MESSAGE_ID, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::SetBreakOnID(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: D3D11_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: D3D11_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: D3D11_MESSAGE_ID) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetBreakOnID(this, core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11InfoQueue_Impl::GetMuteDebugOutput(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilter: GetNumStoredMessagesAllowedByRetrievalFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11InfoQueue as windows_core::Interface>::IID
    }
}
pub trait ID3D11InputLayout_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11InputLayout {}
impl ID3D11InputLayout_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11InputLayout_Impl, const OFFSET: isize>() -> ID3D11InputLayout_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11InputLayout as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11LibraryReflection_Impl: Sized {
    fn GetDesc(&self) -> windows_core::Result<D3D11_LIBRARY_DESC>;
    fn GetFunctionByIndex(&self, functionindex: i32) -> Option<ID3D11FunctionReflection>;
}
impl windows_core::RuntimeName for ID3D11LibraryReflection {}
impl ID3D11LibraryReflection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>() -> ID3D11LibraryReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_LIBRARY_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11LibraryReflection_Impl::GetDesc(this) {
                Ok(ok__) => {
                    core::ptr::write(pdesc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFunctionByIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11LibraryReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionindex: i32) -> Option<ID3D11FunctionReflection> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11LibraryReflection_Impl::GetFunctionByIndex(this, core::mem::transmute_copy(&functionindex))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetFunctionByIndex: GetFunctionByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11LibraryReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11Linker_Impl: Sized {
    fn Link(&self, pentry: Option<&ID3D11ModuleInstance>, pentryname: &windows_core::PCSTR, ptargetname: &windows_core::PCSTR, uflags: u32, ppshaderblob: *mut Option<super::Direct3D::ID3DBlob>, pperrorbuffer: *mut Option<super::Direct3D::ID3DBlob>) -> windows_core::Result<()>;
    fn UseLibrary(&self, plibrarymi: Option<&ID3D11ModuleInstance>) -> windows_core::Result<()>;
    fn AddClipPlaneFromCBuffer(&self, ucbufferslot: u32, ucbufferentry: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D11Linker {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11Linker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: isize>() -> ID3D11Linker_Vtbl {
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentry: *mut core::ffi::c_void, pentryname: windows_core::PCSTR, ptargetname: windows_core::PCSTR, uflags: u32, ppshaderblob: *mut *mut core::ffi::c_void, pperrorbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Linker_Impl::Link(this, windows_core::from_raw_borrowed(&pentry), core::mem::transmute(&pentryname), core::mem::transmute(&ptargetname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ppshaderblob), core::mem::transmute_copy(&pperrorbuffer)).into()
        }
        unsafe extern "system" fn UseLibrary<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibrarymi: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Linker_Impl::UseLibrary(this, windows_core::from_raw_borrowed(&plibrarymi)).into()
        }
        unsafe extern "system" fn AddClipPlaneFromCBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Linker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucbufferslot: u32, ucbufferentry: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Linker_Impl::AddClipPlaneFromCBuffer(this, core::mem::transmute_copy(&ucbufferslot), core::mem::transmute_copy(&ucbufferentry)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Link: Link::<Identity, Impl, OFFSET>,
            UseLibrary: UseLibrary::<Identity, Impl, OFFSET>,
            AddClipPlaneFromCBuffer: AddClipPlaneFromCBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Linker as windows_core::Interface>::IID
    }
}
pub trait ID3D11LinkingNode_Impl: Sized {}
impl windows_core::RuntimeName for ID3D11LinkingNode {}
impl ID3D11LinkingNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11LinkingNode_Impl, const OFFSET: isize>() -> ID3D11LinkingNode_Vtbl {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11LinkingNode as windows_core::Interface>::IID
    }
}
pub trait ID3D11Module_Impl: Sized {
    fn CreateInstance(&self, pnamespace: &windows_core::PCSTR) -> windows_core::Result<ID3D11ModuleInstance>;
}
impl windows_core::RuntimeName for ID3D11Module {}
impl ID3D11Module_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Module_Impl, const OFFSET: isize>() -> ID3D11Module_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Module_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: windows_core::PCSTR, ppmoduleinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11Module_Impl::CreateInstance(this, core::mem::transmute(&pnamespace)) {
                Ok(ok__) => {
                    core::ptr::write(ppmoduleinstance, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Module as windows_core::Interface>::IID
    }
}
pub trait ID3D11ModuleInstance_Impl: Sized {
    fn BindConstantBuffer(&self, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> windows_core::Result<()>;
    fn BindConstantBufferByName(&self, pname: &windows_core::PCSTR, udstslot: u32, cbdstoffset: u32) -> windows_core::Result<()>;
    fn BindResource(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindResourceByName(&self, pname: &windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindSampler(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindSamplerByName(&self, pname: &windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindUnorderedAccessView(&self, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindUnorderedAccessViewByName(&self, pname: &windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindResourceAsUnorderedAccessView(&self, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> windows_core::Result<()>;
    fn BindResourceAsUnorderedAccessViewByName(&self, psrvname: &windows_core::PCSTR, udstuavslot: u32, ucount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D11ModuleInstance {}
impl ID3D11ModuleInstance_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>() -> ID3D11ModuleInstance_Vtbl {
        unsafe extern "system" fn BindConstantBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, cbdstoffset: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindConstantBuffer(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&cbdstoffset)).into()
        }
        unsafe extern "system" fn BindConstantBufferByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, cbdstoffset: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindConstantBufferByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&cbdstoffset)).into()
        }
        unsafe extern "system" fn BindResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindResource(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindResourceByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindSampler<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindSampler(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindSamplerByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindSamplerByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcslot: u32, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindUnorderedAccessView(this, core::mem::transmute_copy(&usrcslot), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindUnorderedAccessViewByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCSTR, udstslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindUnorderedAccessViewByName(this, core::mem::transmute(&pname), core::mem::transmute_copy(&udstslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usrcsrvslot: u32, udstuavslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindResourceAsUnorderedAccessView(this, core::mem::transmute_copy(&usrcsrvslot), core::mem::transmute_copy(&udstuavslot), core::mem::transmute_copy(&ucount)).into()
        }
        unsafe extern "system" fn BindResourceAsUnorderedAccessViewByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ModuleInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrvname: windows_core::PCSTR, udstuavslot: u32, ucount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ModuleInstance_Impl::BindResourceAsUnorderedAccessViewByName(this, core::mem::transmute(&psrvname), core::mem::transmute_copy(&udstuavslot), core::mem::transmute_copy(&ucount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindConstantBuffer: BindConstantBuffer::<Identity, Impl, OFFSET>,
            BindConstantBufferByName: BindConstantBufferByName::<Identity, Impl, OFFSET>,
            BindResource: BindResource::<Identity, Impl, OFFSET>,
            BindResourceByName: BindResourceByName::<Identity, Impl, OFFSET>,
            BindSampler: BindSampler::<Identity, Impl, OFFSET>,
            BindSamplerByName: BindSamplerByName::<Identity, Impl, OFFSET>,
            BindUnorderedAccessView: BindUnorderedAccessView::<Identity, Impl, OFFSET>,
            BindUnorderedAccessViewByName: BindUnorderedAccessViewByName::<Identity, Impl, OFFSET>,
            BindResourceAsUnorderedAccessView: BindResourceAsUnorderedAccessView::<Identity, Impl, OFFSET>,
            BindResourceAsUnorderedAccessViewByName: BindResourceAsUnorderedAccessViewByName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ModuleInstance as windows_core::Interface>::IID
    }
}
pub trait ID3D11Multithread_Impl: Sized {
    fn Enter(&self);
    fn Leave(&self);
    fn SetMultithreadProtected(&self, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetMultithreadProtected(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D11Multithread {}
impl ID3D11Multithread_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: isize>() -> ID3D11Multithread_Vtbl {
        unsafe extern "system" fn Enter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Multithread_Impl::Enter(this)
        }
        unsafe extern "system" fn Leave<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Multithread_Impl::Leave(this)
        }
        unsafe extern "system" fn SetMultithreadProtected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmtprotect: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Multithread_Impl::SetMultithreadProtected(this, core::mem::transmute_copy(&bmtprotect))
        }
        unsafe extern "system" fn GetMultithreadProtected<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Multithread_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Multithread_Impl::GetMultithreadProtected(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enter: Enter::<Identity, Impl, OFFSET>,
            Leave: Leave::<Identity, Impl, OFFSET>,
            SetMultithreadProtected: SetMultithreadProtected::<Identity, Impl, OFFSET>,
            GetMultithreadProtected: GetMultithreadProtected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Multithread as windows_core::Interface>::IID
    }
}
pub trait ID3D11PixelShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11PixelShader {}
impl ID3D11PixelShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11PixelShader_Impl, const OFFSET: isize>() -> ID3D11PixelShader_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11PixelShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11Predicate_Impl: Sized + ID3D11Query_Impl {}
impl windows_core::RuntimeName for ID3D11Predicate {}
impl ID3D11Predicate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Predicate_Impl, const OFFSET: isize>() -> ID3D11Predicate_Vtbl {
        Self { base__: ID3D11Query_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Predicate as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID || iid == &<ID3D11Query as windows_core::Interface>::IID
    }
}
pub trait ID3D11Query_Impl: Sized + ID3D11Asynchronous_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_QUERY_DESC);
}
impl windows_core::RuntimeName for ID3D11Query {}
impl ID3D11Query_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Query_Impl, const OFFSET: isize>() -> ID3D11Query_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Query_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_QUERY_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Query_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Asynchronous_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Query as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID
    }
}
pub trait ID3D11Query1_Impl: Sized + ID3D11Query_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_QUERY_DESC1);
}
impl windows_core::RuntimeName for ID3D11Query1 {}
impl ID3D11Query1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Query1_Impl, const OFFSET: isize>() -> ID3D11Query1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Query1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_QUERY_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Query1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1))
        }
        Self { base__: ID3D11Query_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Query1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Asynchronous as windows_core::Interface>::IID || iid == &<ID3D11Query as windows_core::Interface>::IID
    }
}
pub trait ID3D11RasterizerState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC);
}
impl windows_core::RuntimeName for ID3D11RasterizerState {}
impl ID3D11RasterizerState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RasterizerState_Impl, const OFFSET: isize>() -> ID3D11RasterizerState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RasterizerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RasterizerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RasterizerState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11RasterizerState1_Impl: Sized + ID3D11RasterizerState_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_RASTERIZER_DESC1);
}
impl windows_core::RuntimeName for ID3D11RasterizerState1 {}
impl ID3D11RasterizerState1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RasterizerState1_Impl, const OFFSET: isize>() -> ID3D11RasterizerState1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RasterizerState1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RasterizerState1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11RasterizerState_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RasterizerState1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11RasterizerState as windows_core::Interface>::IID
    }
}
pub trait ID3D11RasterizerState2_Impl: Sized + ID3D11RasterizerState1_Impl {
    fn GetDesc2(&self, pdesc: *mut D3D11_RASTERIZER_DESC2);
}
impl windows_core::RuntimeName for ID3D11RasterizerState2 {}
impl ID3D11RasterizerState2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RasterizerState2_Impl, const OFFSET: isize>() -> ID3D11RasterizerState2_Vtbl {
        unsafe extern "system" fn GetDesc2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RasterizerState2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RASTERIZER_DESC2) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RasterizerState2_Impl::GetDesc2(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11RasterizerState1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RasterizerState2 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11RasterizerState as windows_core::Interface>::IID || iid == &<ID3D11RasterizerState1 as windows_core::Interface>::IID
    }
}
pub trait ID3D11RefDefaultTrackingOptions_Impl: Sized {
    fn SetTrackingOptions(&self, resourcetypeflags: u32, options: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D11RefDefaultTrackingOptions {}
impl ID3D11RefDefaultTrackingOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: isize>() -> ID3D11RefDefaultTrackingOptions_Vtbl {
        unsafe extern "system" fn SetTrackingOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RefDefaultTrackingOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcetypeflags: u32, options: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RefDefaultTrackingOptions_Impl::SetTrackingOptions(this, core::mem::transmute_copy(&resourcetypeflags), core::mem::transmute_copy(&options)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetTrackingOptions: SetTrackingOptions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RefDefaultTrackingOptions as windows_core::Interface>::IID
    }
}
pub trait ID3D11RefTrackingOptions_Impl: Sized {
    fn SetTrackingOptions(&self, uoptions: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D11RefTrackingOptions {}
impl ID3D11RefTrackingOptions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: isize>() -> ID3D11RefTrackingOptions_Vtbl {
        unsafe extern "system" fn SetTrackingOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RefTrackingOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uoptions: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RefTrackingOptions_Impl::SetTrackingOptions(this, core::mem::transmute_copy(&uoptions)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetTrackingOptions: SetTrackingOptions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RefTrackingOptions as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11RenderTargetView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RenderTargetView_Impl, const OFFSET: isize>() -> ID3D11RenderTargetView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RenderTargetView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RenderTargetView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11RenderTargetView1_Impl: Sized + ID3D11RenderTargetView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11RenderTargetView1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11RenderTargetView1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RenderTargetView1_Impl, const OFFSET: isize>() -> ID3D11RenderTargetView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11RenderTargetView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_RENDER_TARGET_VIEW_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11RenderTargetView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1))
        }
        Self { base__: ID3D11RenderTargetView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11RenderTargetView1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID || iid == &<ID3D11RenderTargetView as windows_core::Interface>::IID
    }
}
pub trait ID3D11Resource_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetType(&self, presourcedimension: *mut D3D11_RESOURCE_DIMENSION);
    fn SetEvictionPriority(&self, evictionpriority: u32);
    fn GetEvictionPriority(&self) -> u32;
}
impl windows_core::RuntimeName for ID3D11Resource {}
impl ID3D11Resource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: isize>() -> ID3D11Resource_Vtbl {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcedimension: *mut D3D11_RESOURCE_DIMENSION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Resource_Impl::GetType(this, core::mem::transmute_copy(&presourcedimension))
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, evictionpriority: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Resource_Impl::SetEvictionPriority(this, core::mem::transmute_copy(&evictionpriority))
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Resource_Impl::GetEvictionPriority(this)
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Resource as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11SamplerState_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SAMPLER_DESC);
}
impl windows_core::RuntimeName for ID3D11SamplerState {}
impl ID3D11SamplerState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11SamplerState_Impl, const OFFSET: isize>() -> ID3D11SamplerState_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11SamplerState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SAMPLER_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11SamplerState_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11SamplerState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflection_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_DESC) -> windows_core::Result<()>;
    fn GetConstantBufferByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetConstantBufferByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetResourceBindingDesc(&self, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetInputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetOutputParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetPatchConstantParameterDesc(&self, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::Result<()>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>;
    fn GetResourceBindingDescByName(&self, name: &windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::Result<()>;
    fn GetMovInstructionCount(&self) -> u32;
    fn GetMovcInstructionCount(&self) -> u32;
    fn GetConversionInstructionCount(&self) -> u32;
    fn GetBitwiseInstructionCount(&self) -> u32;
    fn GetGSInputPrimitive(&self) -> super::Direct3D::D3D_PRIMITIVE;
    fn IsSampleFrequencyShader(&self) -> super::super::Foundation::BOOL;
    fn GetNumInterfaceSlots(&self) -> u32;
    fn GetMinFeatureLevel(&self) -> windows_core::Result<super::Direct3D::D3D_FEATURE_LEVEL>;
    fn GetThreadGroupSize(&self, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32;
    fn GetRequiresFlags(&self) -> u64;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for ID3D11ShaderReflection {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>() -> ID3D11ShaderReflection_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetConstantBufferByIndex<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetConstantBufferByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetConstantBufferByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetConstantBufferByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceindex: u32, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetResourceBindingDesc(this, core::mem::transmute_copy(&resourceindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetInputParameterDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetInputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetOutputParameterDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetOutputParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetPatchConstantParameterDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameterindex: u32, pdesc: *mut D3D11_SIGNATURE_PARAMETER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetPatchConstantParameterDesc(this, core::mem::transmute_copy(&parameterindex), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetResourceBindingDescByName<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, pdesc: *mut D3D11_SHADER_INPUT_BIND_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetResourceBindingDescByName(this, core::mem::transmute(&name), core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMovInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetMovInstructionCount(this)
        }
        unsafe extern "system" fn GetMovcInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetMovcInstructionCount(this)
        }
        unsafe extern "system" fn GetConversionInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetConversionInstructionCount(this)
        }
        unsafe extern "system" fn GetBitwiseInstructionCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetBitwiseInstructionCount(this)
        }
        unsafe extern "system" fn GetGSInputPrimitive<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::Direct3D::D3D_PRIMITIVE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetGSInputPrimitive(this)
        }
        unsafe extern "system" fn IsSampleFrequencyShader<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::IsSampleFrequencyShader(this)
        }
        unsafe extern "system" fn GetNumInterfaceSlots<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetNumInterfaceSlots(this)
        }
        unsafe extern "system" fn GetMinFeatureLevel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11ShaderReflection_Impl::GetMinFeatureLevel(this) {
                Ok(ok__) => {
                    core::ptr::write(plevel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadGroupSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psizex: *mut u32, psizey: *mut u32, psizez: *mut u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetThreadGroupSize(this, core::mem::transmute_copy(&psizex), core::mem::transmute_copy(&psizey), core::mem::transmute_copy(&psizez))
        }
        unsafe extern "system" fn GetRequiresFlags<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderReflection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderReflection_Impl::GetRequiresFlags(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetConstantBufferByIndex: GetConstantBufferByIndex::<Identity, Impl, OFFSET>,
            GetConstantBufferByName: GetConstantBufferByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDesc: GetResourceBindingDesc::<Identity, Impl, OFFSET>,
            GetInputParameterDesc: GetInputParameterDesc::<Identity, Impl, OFFSET>,
            GetOutputParameterDesc: GetOutputParameterDesc::<Identity, Impl, OFFSET>,
            GetPatchConstantParameterDesc: GetPatchConstantParameterDesc::<Identity, Impl, OFFSET>,
            GetVariableByName: GetVariableByName::<Identity, Impl, OFFSET>,
            GetResourceBindingDescByName: GetResourceBindingDescByName::<Identity, Impl, OFFSET>,
            GetMovInstructionCount: GetMovInstructionCount::<Identity, Impl, OFFSET>,
            GetMovcInstructionCount: GetMovcInstructionCount::<Identity, Impl, OFFSET>,
            GetConversionInstructionCount: GetConversionInstructionCount::<Identity, Impl, OFFSET>,
            GetBitwiseInstructionCount: GetBitwiseInstructionCount::<Identity, Impl, OFFSET>,
            GetGSInputPrimitive: GetGSInputPrimitive::<Identity, Impl, OFFSET>,
            IsSampleFrequencyShader: IsSampleFrequencyShader::<Identity, Impl, OFFSET>,
            GetNumInterfaceSlots: GetNumInterfaceSlots::<Identity, Impl, OFFSET>,
            GetMinFeatureLevel: GetMinFeatureLevel::<Identity, Impl, OFFSET>,
            GetThreadGroupSize: GetThreadGroupSize::<Identity, Impl, OFFSET>,
            GetRequiresFlags: GetRequiresFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderReflection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflectionConstantBuffer_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::Result<()>;
    fn GetVariableByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionVariable>;
    fn GetVariableByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionConstantBuffer_Vtbl {
    pub const fn new<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>() -> ID3D11ShaderReflectionConstantBuffer_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_BUFFER_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionConstantBuffer_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetVariableByIndex<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionConstantBuffer_Impl::GetVariableByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetVariableByName<Impl: ID3D11ShaderReflectionConstantBuffer_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionVariable> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionConstantBuffer_Impl::GetVariableByName(this, core::mem::transmute(&name))
        }
        Self { GetDesc: GetDesc::<Impl>, GetVariableByIndex: GetVariableByIndex::<Impl>, GetVariableByName: GetVariableByName::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11ShaderReflectionConstantBuffer_ImplVtbl<T: ID3D11ShaderReflectionConstantBuffer_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11ShaderReflectionConstantBuffer_Impl> ID3D11ShaderReflectionConstantBuffer_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionConstantBuffer_Vtbl = ID3D11ShaderReflectionConstantBuffer_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionConstantBuffer {
    pub fn new<'a, T: ID3D11ShaderReflectionConstantBuffer_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionConstantBuffer_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait ID3D11ShaderReflectionType_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> windows_core::Result<()>;
    fn GetMemberTypeByIndex(&self, index: u32) -> Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeByName(&self, name: &windows_core::PCSTR) -> Option<ID3D11ShaderReflectionType>;
    fn GetMemberTypeName(&self, index: u32) -> windows_core::PCSTR;
    fn IsEqual(&self, ptype: Option<&ID3D11ShaderReflectionType>) -> windows_core::Result<()>;
    fn GetSubType(&self) -> Option<ID3D11ShaderReflectionType>;
    fn GetBaseClass(&self) -> Option<ID3D11ShaderReflectionType>;
    fn GetNumInterfaces(&self) -> u32;
    fn GetInterfaceByIndex(&self, uindex: u32) -> Option<ID3D11ShaderReflectionType>;
    fn IsOfType(&self, ptype: Option<&ID3D11ShaderReflectionType>) -> windows_core::Result<()>;
    fn ImplementsInterface(&self, pbase: Option<&ID3D11ShaderReflectionType>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionType_Vtbl {
    pub const fn new<Impl: ID3D11ShaderReflectionType_Impl>() -> ID3D11ShaderReflectionType_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_TYPE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetMemberTypeByIndex<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetMemberTypeByIndex(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn GetMemberTypeByName<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCSTR) -> Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetMemberTypeByName(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetMemberTypeName<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, index: u32) -> windows_core::PCSTR {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetMemberTypeName(this, core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn IsEqual<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::IsEqual(this, windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn GetSubType<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetSubType(this)
        }
        unsafe extern "system" fn GetBaseClass<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetBaseClass(this)
        }
        unsafe extern "system" fn GetNumInterfaces<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetNumInterfaces(this)
        }
        unsafe extern "system" fn GetInterfaceByIndex<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, uindex: u32) -> Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::GetInterfaceByIndex(this, core::mem::transmute_copy(&uindex))
        }
        unsafe extern "system" fn IsOfType<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::IsOfType(this, windows_core::from_raw_borrowed(&ptype)).into()
        }
        unsafe extern "system" fn ImplementsInterface<Impl: ID3D11ShaderReflectionType_Impl>(this: *mut core::ffi::c_void, pbase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionType_Impl::ImplementsInterface(this, windows_core::from_raw_borrowed(&pbase)).into()
        }
        Self {
            GetDesc: GetDesc::<Impl>,
            GetMemberTypeByIndex: GetMemberTypeByIndex::<Impl>,
            GetMemberTypeByName: GetMemberTypeByName::<Impl>,
            GetMemberTypeName: GetMemberTypeName::<Impl>,
            IsEqual: IsEqual::<Impl>,
            GetSubType: GetSubType::<Impl>,
            GetBaseClass: GetBaseClass::<Impl>,
            GetNumInterfaces: GetNumInterfaces::<Impl>,
            GetInterfaceByIndex: GetInterfaceByIndex::<Impl>,
            IsOfType: IsOfType::<Impl>,
            ImplementsInterface: ImplementsInterface::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Graphics_Direct3D")]
struct ID3D11ShaderReflectionType_ImplVtbl<T: ID3D11ShaderReflectionType_Impl>(std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl<T: ID3D11ShaderReflectionType_Impl> ID3D11ShaderReflectionType_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionType_Vtbl = ID3D11ShaderReflectionType_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ID3D11ShaderReflectionType {
    pub fn new<'a, T: ID3D11ShaderReflectionType_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionType_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ID3D11ShaderReflectionVariable_Impl: Sized {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::Result<()>;
    fn GetType(&self) -> Option<ID3D11ShaderReflectionType>;
    fn GetBuffer(&self) -> Option<ID3D11ShaderReflectionConstantBuffer>;
    fn GetInterfaceSlot(&self, uarrayindex: u32) -> u32;
}
impl ID3D11ShaderReflectionVariable_Vtbl {
    pub const fn new<Impl: ID3D11ShaderReflectionVariable_Impl>() -> ID3D11ShaderReflectionVariable_Vtbl {
        unsafe extern "system" fn GetDesc<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_VARIABLE_DESC) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionVariable_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetType<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionType> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionVariable_Impl::GetType(this)
        }
        unsafe extern "system" fn GetBuffer<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void) -> Option<ID3D11ShaderReflectionConstantBuffer> {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionVariable_Impl::GetBuffer(this)
        }
        unsafe extern "system" fn GetInterfaceSlot<Impl: ID3D11ShaderReflectionVariable_Impl>(this: *mut core::ffi::c_void, uarrayindex: u32) -> u32 {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ID3D11ShaderReflectionVariable_Impl::GetInterfaceSlot(this, core::mem::transmute_copy(&uarrayindex))
        }
        Self { GetDesc: GetDesc::<Impl>, GetType: GetType::<Impl>, GetBuffer: GetBuffer::<Impl>, GetInterfaceSlot: GetInterfaceSlot::<Impl> }
    }
}
#[doc(hidden)]
struct ID3D11ShaderReflectionVariable_ImplVtbl<T: ID3D11ShaderReflectionVariable_Impl>(std::marker::PhantomData<T>);
impl<T: ID3D11ShaderReflectionVariable_Impl> ID3D11ShaderReflectionVariable_ImplVtbl<T> {
    const VTABLE: ID3D11ShaderReflectionVariable_Vtbl = ID3D11ShaderReflectionVariable_Vtbl::new::<T>();
}
impl ID3D11ShaderReflectionVariable {
    pub fn new<'a, T: ID3D11ShaderReflectionVariable_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ID3D11ShaderReflectionVariable_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11ShaderResourceView {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderResourceView_Impl, const OFFSET: isize>() -> ID3D11ShaderResourceView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderResourceView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderResourceView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ID3D11ShaderResourceView1_Impl: Sized + ID3D11ShaderResourceView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1);
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11ShaderResourceView1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ID3D11ShaderResourceView1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>() -> ID3D11ShaderResourceView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderResourceView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_SHADER_RESOURCE_VIEW_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderResourceView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1))
        }
        Self { base__: ID3D11ShaderResourceView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderResourceView1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID || iid == &<ID3D11ShaderResourceView as windows_core::Interface>::IID
    }
}
pub trait ID3D11ShaderTrace_Impl: Sized {
    fn TraceReady(&self, ptestcount: *mut u64) -> windows_core::Result<()>;
    fn ResetTrace(&self);
    fn GetTraceStats(&self, ptracestats: *mut D3D11_TRACE_STATS) -> windows_core::Result<()>;
    fn PSSelectStamp(&self, stampindex: u32) -> windows_core::Result<()>;
    fn GetInitialRegisterContents(&self, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::Result<()>;
    fn GetStep(&self, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> windows_core::Result<()>;
    fn GetWrittenRegister(&self, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::Result<()>;
    fn GetReadRegister(&self, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D11ShaderTrace {}
impl ID3D11ShaderTrace_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>() -> ID3D11ShaderTrace_Vtbl {
        unsafe extern "system" fn TraceReady<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptestcount: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::TraceReady(this, core::mem::transmute_copy(&ptestcount)).into()
        }
        unsafe extern "system" fn ResetTrace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::ResetTrace(this)
        }
        unsafe extern "system" fn GetTraceStats<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptracestats: *mut D3D11_TRACE_STATS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::GetTraceStats(this, core::mem::transmute_copy(&ptracestats)).into()
        }
        unsafe extern "system" fn PSSelectStamp<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stampindex: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::PSSelectStamp(this, core::mem::transmute_copy(&stampindex)).into()
        }
        unsafe extern "system" fn GetInitialRegisterContents<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pregister: *const D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::GetInitialRegisterContents(this, core::mem::transmute_copy(&pregister), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetStep<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stepindex: u32, ptracestep: *mut D3D11_TRACE_STEP) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::GetStep(this, core::mem::transmute_copy(&stepindex), core::mem::transmute_copy(&ptracestep)).into()
        }
        unsafe extern "system" fn GetWrittenRegister<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stepindex: u32, writtenregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::GetWrittenRegister(this, core::mem::transmute_copy(&stepindex), core::mem::transmute_copy(&writtenregisterindex), core::mem::transmute_copy(&pregister), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetReadRegister<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stepindex: u32, readregisterindex: u32, pregister: *mut D3D11_TRACE_REGISTER, pvalue: *mut D3D11_TRACE_VALUE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11ShaderTrace_Impl::GetReadRegister(this, core::mem::transmute_copy(&stepindex), core::mem::transmute_copy(&readregisterindex), core::mem::transmute_copy(&pregister), core::mem::transmute_copy(&pvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TraceReady: TraceReady::<Identity, Impl, OFFSET>,
            ResetTrace: ResetTrace::<Identity, Impl, OFFSET>,
            GetTraceStats: GetTraceStats::<Identity, Impl, OFFSET>,
            PSSelectStamp: PSSelectStamp::<Identity, Impl, OFFSET>,
            GetInitialRegisterContents: GetInitialRegisterContents::<Identity, Impl, OFFSET>,
            GetStep: GetStep::<Identity, Impl, OFFSET>,
            GetWrittenRegister: GetWrittenRegister::<Identity, Impl, OFFSET>,
            GetReadRegister: GetReadRegister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderTrace as windows_core::Interface>::IID
    }
}
pub trait ID3D11ShaderTraceFactory_Impl: Sized {
    fn CreateShaderTrace(&self, pshader: Option<&windows_core::IUnknown>, ptracedesc: *const D3D11_SHADER_TRACE_DESC) -> windows_core::Result<ID3D11ShaderTrace>;
}
impl windows_core::RuntimeName for ID3D11ShaderTraceFactory {}
impl ID3D11ShaderTraceFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: isize>() -> ID3D11ShaderTraceFactory_Vtbl {
        unsafe extern "system" fn CreateShaderTrace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11ShaderTraceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, ptracedesc: *const D3D11_SHADER_TRACE_DESC, ppshadertrace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11ShaderTraceFactory_Impl::CreateShaderTrace(this, windows_core::from_raw_borrowed(&pshader), core::mem::transmute_copy(&ptracedesc)) {
                Ok(ok__) => {
                    core::ptr::write(ppshadertrace, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateShaderTrace: CreateShaderTrace::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11ShaderTraceFactory as windows_core::Interface>::IID
    }
}
pub trait ID3D11SwitchToRef_Impl: Sized {
    fn SetUseRef(&self, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    fn GetUseRef(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3D11SwitchToRef {}
impl ID3D11SwitchToRef_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>() -> ID3D11SwitchToRef_Vtbl {
        unsafe extern "system" fn SetUseRef<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useref: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11SwitchToRef_Impl::SetUseRef(this, core::mem::transmute_copy(&useref))
        }
        unsafe extern "system" fn GetUseRef<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11SwitchToRef_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11SwitchToRef_Impl::GetUseRef(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUseRef: SetUseRef::<Identity, Impl, OFFSET>,
            GetUseRef: GetUseRef::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11SwitchToRef as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture1D_Impl: Sized + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE1D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11Texture1D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture1D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture1D_Impl, const OFFSET: isize>() -> ID3D11Texture1D_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture1D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE1D_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Texture1D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture1D as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D_Impl: Sized + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11Texture2D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture2D_Impl, const OFFSET: isize>() -> ID3D11Texture2D_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture2D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Texture2D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture2D as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture2D1_Impl: Sized + ID3D11Texture2D_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE2D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11Texture2D1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture2D1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture2D1_Impl, const OFFSET: isize>() -> ID3D11Texture2D1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture2D1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE2D_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Texture2D1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Texture2D_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture2D1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID || iid == &<ID3D11Texture2D as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D_Impl: Sized + ID3D11Resource_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE3D_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11Texture3D {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture3D_Impl, const OFFSET: isize>() -> ID3D11Texture3D_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture3D_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Texture3D_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Resource_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture3D as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11Texture3D1_Impl: Sized + ID3D11Texture3D_Impl {
    fn GetDesc1(&self, pdesc: *mut D3D11_TEXTURE3D_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11Texture3D1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11Texture3D1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture3D1_Impl, const OFFSET: isize>() -> ID3D11Texture3D1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11Texture3D1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_TEXTURE3D_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11Texture3D1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11Texture3D_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Texture3D1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11Resource as windows_core::Interface>::IID || iid == &<ID3D11Texture3D as windows_core::Interface>::IID
    }
}
pub trait ID3D11TracingDevice_Impl: Sized {
    fn SetShaderTrackingOptionsByType(&self, resourcetypeflags: u32, options: u32) -> windows_core::Result<()>;
    fn SetShaderTrackingOptions(&self, pshader: Option<&windows_core::IUnknown>, options: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3D11TracingDevice {}
impl ID3D11TracingDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>() -> ID3D11TracingDevice_Vtbl {
        unsafe extern "system" fn SetShaderTrackingOptionsByType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcetypeflags: u32, options: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11TracingDevice_Impl::SetShaderTrackingOptionsByType(this, core::mem::transmute_copy(&resourcetypeflags), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetShaderTrackingOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11TracingDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshader: *mut core::ffi::c_void, options: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11TracingDevice_Impl::SetShaderTrackingOptions(this, windows_core::from_raw_borrowed(&pshader), core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetShaderTrackingOptionsByType: SetShaderTrackingOptionsByType::<Identity, Impl, OFFSET>,
            SetShaderTrackingOptions: SetShaderTrackingOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11TracingDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11UnorderedAccessView {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>() -> ID3D11UnorderedAccessView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11UnorderedAccessView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11UnorderedAccessView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11UnorderedAccessView1_Impl: Sized + ID3D11UnorderedAccessView_Impl {
    fn GetDesc1(&self, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11UnorderedAccessView1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11UnorderedAccessView1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>() -> ID3D11UnorderedAccessView1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11UnorderedAccessView1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc1: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC1) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11UnorderedAccessView1_Impl::GetDesc1(this, core::mem::transmute_copy(&pdesc1))
        }
        Self { base__: ID3D11UnorderedAccessView_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11UnorderedAccessView1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID || iid == &<ID3D11UnorderedAccessView as windows_core::Interface>::IID
    }
}
pub trait ID3D11VertexShader_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3D11VertexShader {}
impl ID3D11VertexShader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VertexShader_Impl, const OFFSET: isize>() -> ID3D11VertexShader_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VertexShader as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoContext_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetDecoderBuffer(&self, pdecoder: Option<&ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ReleaseDecoderBuffer(&self, pdecoder: Option<&ID3D11VideoDecoder>, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::Result<()>;
    fn DecoderBeginFrame(&self, pdecoder: Option<&ID3D11VideoDecoder>, pview: Option<&ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn DecoderEndFrame(&self, pdecoder: Option<&ID3D11VideoDecoder>) -> windows_core::Result<()>;
    fn SubmitDecoderBuffers(&self, pdecoder: Option<&ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> windows_core::Result<()>;
    fn DecoderExtension(&self, pdecoder: Option<&ID3D11VideoDecoder>, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32;
    fn VideoProcessorSetOutputTargetRect(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetOutputBackgroundColor(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR);
    fn VideoProcessorSetOutputColorSpace(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetOutputAlphaFillMode(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32);
    fn VideoProcessorSetOutputConstriction(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL, size: &super::super::Foundation::SIZE);
    fn VideoProcessorSetOutputStereoMode(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetOutputExtension(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> i32;
    fn VideoProcessorGetOutputTargetRect(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetOutputBackgroundColor(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR);
    fn VideoProcessorGetOutputColorSpace(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetOutputAlphaFillMode(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32);
    fn VideoProcessorGetOutputConstriction(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE);
    fn VideoProcessorGetOutputStereoMode(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputExtension(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> i32;
    fn VideoProcessorSetStreamFrameFormat(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorSetStreamColorSpace(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorSetStreamOutputRate(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamSourceRect(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamDestRect(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT);
    fn VideoProcessorSetStreamAlpha(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32);
    fn VideoProcessorSetStreamPalette(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *const u32);
    fn VideoProcessorSetStreamPixelAspectRatio(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorSetStreamLumaKey(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32);
    fn VideoProcessorSetStreamStereoFormat(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32);
    fn VideoProcessorSetStreamAutoProcessingMode(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamFilter(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32);
    fn VideoProcessorSetStreamExtension(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> i32;
    fn VideoProcessorGetStreamFrameFormat(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT);
    fn VideoProcessorGetStreamColorSpace(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE);
    fn VideoProcessorGetStreamOutputRate(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamSourceRect(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamDestRect(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT);
    fn VideoProcessorGetStreamAlpha(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32);
    fn VideoProcessorGetStreamPalette(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, count: u32, pentries: *mut u32);
    fn VideoProcessorGetStreamPixelAspectRatio(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL);
    fn VideoProcessorGetStreamLumaKey(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32);
    fn VideoProcessorGetStreamStereoFormat(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32);
    fn VideoProcessorGetStreamAutoProcessingMode(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penabled: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamFilter(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32);
    fn VideoProcessorGetStreamExtension(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> i32;
    fn VideoProcessorBlt(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pview: Option<&ID3D11VideoProcessorOutputView>, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchange(&self, pcryptosession: Option<&ID3D11CryptoSession>, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EncryptionBlt(&self, pcryptosession: Option<&ID3D11CryptoSession>, psrcsurface: Option<&ID3D11Texture2D>, pdstsurface: Option<&ID3D11Texture2D>, ivsize: u32, piv: *mut core::ffi::c_void);
    fn DecryptionBlt(&self, pcryptosession: Option<&ID3D11CryptoSession>, psrcsurface: Option<&ID3D11Texture2D>, pdstsurface: Option<&ID3D11Texture2D>, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, ivsize: u32, piv: *mut core::ffi::c_void);
    fn StartSessionKeyRefresh(&self, pcryptosession: Option<&ID3D11CryptoSession>, randomnumbersize: u32, prandomnumber: *mut core::ffi::c_void);
    fn FinishSessionKeyRefresh(&self, pcryptosession: Option<&ID3D11CryptoSession>);
    fn GetEncryptionBltKey(&self, pcryptosession: Option<&ID3D11CryptoSession>, keysize: u32, preadbackkey: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn NegotiateAuthenticatedChannelKeyExchange(&self, pchannel: Option<&ID3D11AuthenticatedChannel>, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn QueryAuthenticatedChannel(&self, pchannel: Option<&ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ConfigureAuthenticatedChannel(&self, pchannel: Option<&ID3D11AuthenticatedChannel>, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::Result<()>;
    fn VideoProcessorSetStreamRotation(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION);
    fn VideoProcessorGetStreamRotation(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoContext {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>() -> ID3D11VideoContext_Vtbl {
        unsafe extern "system" fn GetDecoderBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::GetDecoderBuffer(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pbuffersize), core::mem::transmute_copy(&ppbuffer)).into()
        }
        unsafe extern "system" fn ReleaseDecoderBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::ReleaseDecoderBuffer(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn DecoderBeginFrame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, contentkeysize: u32, pcontentkey: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::DecoderBeginFrame(this, windows_core::from_raw_borrowed(&pdecoder), windows_core::from_raw_borrowed(&pview), core::mem::transmute_copy(&contentkeysize), core::mem::transmute_copy(&pcontentkey)).into()
        }
        unsafe extern "system" fn DecoderEndFrame<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::DecoderEndFrame(this, windows_core::from_raw_borrowed(&pdecoder)).into()
        }
        unsafe extern "system" fn SubmitDecoderBuffers<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::SubmitDecoderBuffers(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&pbufferdesc)).into()
        }
        unsafe extern "system" fn DecoderExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::DecoderExtension(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&pextensiondata))
        }
        unsafe extern "system" fn VideoProcessorSetOutputTargetRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputTargetRect(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetOutputBackgroundColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, ycbcr: super::super::Foundation::BOOL, pcolor: *const D3D11_VIDEO_COLOR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputBackgroundColor(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&ycbcr), core::mem::transmute_copy(&pcolor))
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputColorSpace(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorSetOutputAlphaFillMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputAlphaFillMode(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&alphafillmode), core::mem::transmute_copy(&streamindex))
        }
        unsafe extern "system" fn VideoProcessorSetOutputConstriction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL, size: super::super::Foundation::SIZE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputConstriction(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&enable), core::mem::transmute(&size))
        }
        unsafe extern "system" fn VideoProcessorSetOutputStereoMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputStereoMode(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn VideoProcessorSetOutputExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetOutputExtension(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorGetOutputTargetRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputTargetRect(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&enabled), core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetOutputBackgroundColor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputBackgroundColor(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pycbcr), core::mem::transmute_copy(&pcolor))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputColorSpace(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetOutputAlphaFillMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputAlphaFillMode(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&palphafillmode), core::mem::transmute_copy(&pstreamindex))
        }
        unsafe extern "system" fn VideoProcessorGetOutputConstriction<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputConstriction(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&psize))
        }
        unsafe extern "system" fn VideoProcessorGetOutputStereoMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputStereoMode(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn VideoProcessorGetOutputExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetOutputExtension(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFrameFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamFrameFormat(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&frameformat))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamColorSpace(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorSetStreamOutputRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: super::super::Foundation::BOOL, pcustomrate: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamOutputRate(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&outputrate), core::mem::transmute_copy(&repeatframe), core::mem::transmute_copy(&pcustomrate))
        }
        unsafe extern "system" fn VideoProcessorSetStreamSourceRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamSourceRect(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetStreamDestRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, prect: *const super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamDestRect(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAlpha<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, alpha: f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamAlpha(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&alpha))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPalette<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, count: u32, pentries: *const u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamPalette(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&count), core::mem::transmute_copy(&pentries))
        }
        unsafe extern "system" fn VideoProcessorSetStreamPixelAspectRatio<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, psourceaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *const super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamPixelAspectRatio(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&psourceaspectratio), core::mem::transmute_copy(&pdestinationaspectratio))
        }
        unsafe extern "system" fn VideoProcessorSetStreamLumaKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, lower: f32, upper: f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamLumaKey(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&lower), core::mem::transmute_copy(&upper))
        }
        unsafe extern "system" fn VideoProcessorSetStreamStereoFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: super::super::Foundation::BOOL, baseviewframe0: super::super::Foundation::BOOL, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamStereoFormat(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&format), core::mem::transmute_copy(&leftviewframe0), core::mem::transmute_copy(&baseviewframe0), core::mem::transmute_copy(&flipmode), core::mem::transmute_copy(&monooffset))
        }
        unsafe extern "system" fn VideoProcessorSetStreamAutoProcessingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamAutoProcessingMode(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable))
        }
        unsafe extern "system" fn VideoProcessorSetStreamFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: super::super::Foundation::BOOL, level: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamFilter(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&filter), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&level))
        }
        unsafe extern "system" fn VideoProcessorSetStreamExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamExtension(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFrameFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pframeformat: *mut D3D11_VIDEO_FRAME_FORMAT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamFrameFormat(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pframeformat))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pcolorspace: *mut D3D11_VIDEO_PROCESSOR_COLOR_SPACE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamColorSpace(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetStreamOutputRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamOutputRate(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&poutputrate), core::mem::transmute_copy(&prepeatframe), core::mem::transmute_copy(&pcustomrate))
        }
        unsafe extern "system" fn VideoProcessorGetStreamSourceRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamSourceRect(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetStreamDestRect<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamDestRect(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&prect))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAlpha<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamAlpha(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&palpha))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPalette<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, count: u32, pentries: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamPalette(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&count), core::mem::transmute_copy(&pentries))
        }
        unsafe extern "system" fn VideoProcessorGetStreamPixelAspectRatio<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamPixelAspectRatio(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&psourceaspectratio), core::mem::transmute_copy(&pdestinationaspectratio))
        }
        unsafe extern "system" fn VideoProcessorGetStreamLumaKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamLumaKey(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&plower), core::mem::transmute_copy(&pupper))
        }
        unsafe extern "system" fn VideoProcessorGetStreamStereoFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamStereoFormat(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penable), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pleftviewframe0), core::mem::transmute_copy(&pbaseviewframe0), core::mem::transmute_copy(&pflipmode), core::mem::transmute_copy(&monooffset))
        }
        unsafe extern "system" fn VideoProcessorGetStreamAutoProcessingMode<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penabled: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamAutoProcessingMode(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn VideoProcessorGetStreamFilter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamFilter(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&filter), core::mem::transmute_copy(&penabled), core::mem::transmute_copy(&plevel))
        }
        unsafe extern "system" fn VideoProcessorGetStreamExtension<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pextensionguid: *const windows_core::GUID, datasize: u32, pdata: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamExtension(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pextensionguid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata))
        }
        unsafe extern "system" fn VideoProcessorBlt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, outputframe: u32, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorBlt(this, windows_core::from_raw_borrowed(&pvideoprocessor), windows_core::from_raw_borrowed(&pview), core::mem::transmute_copy(&outputframe), core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams)).into()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::NegotiateCryptoSessionKeyExchange(this, windows_core::from_raw_borrowed(&pcryptosession), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn EncryptionBlt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, ivsize: u32, piv: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::EncryptionBlt(this, windows_core::from_raw_borrowed(&pcryptosession), windows_core::from_raw_borrowed(&psrcsurface), windows_core::from_raw_borrowed(&pdstsurface), core::mem::transmute_copy(&ivsize), core::mem::transmute_copy(&piv))
        }
        unsafe extern "system" fn DecryptionBlt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, psrcsurface: *mut core::ffi::c_void, pdstsurface: *mut core::ffi::c_void, pencryptedblockinfo: *const D3D11_ENCRYPTED_BLOCK_INFO, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, ivsize: u32, piv: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::DecryptionBlt(this, windows_core::from_raw_borrowed(&pcryptosession), windows_core::from_raw_borrowed(&psrcsurface), windows_core::from_raw_borrowed(&pdstsurface), core::mem::transmute_copy(&pencryptedblockinfo), core::mem::transmute_copy(&contentkeysize), core::mem::transmute_copy(&pcontentkey), core::mem::transmute_copy(&ivsize), core::mem::transmute_copy(&piv))
        }
        unsafe extern "system" fn StartSessionKeyRefresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, randomnumbersize: u32, prandomnumber: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::StartSessionKeyRefresh(this, windows_core::from_raw_borrowed(&pcryptosession), core::mem::transmute_copy(&randomnumbersize), core::mem::transmute_copy(&prandomnumber))
        }
        unsafe extern "system" fn FinishSessionKeyRefresh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::FinishSessionKeyRefresh(this, windows_core::from_raw_borrowed(&pcryptosession))
        }
        unsafe extern "system" fn GetEncryptionBltKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, keysize: u32, preadbackkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::GetEncryptionBltKey(this, windows_core::from_raw_borrowed(&pcryptosession), core::mem::transmute_copy(&keysize), core::mem::transmute_copy(&preadbackkey)).into()
        }
        unsafe extern "system" fn NegotiateAuthenticatedChannelKeyExchange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::NegotiateAuthenticatedChannelKeyExchange(this, windows_core::from_raw_borrowed(&pchannel), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn QueryAuthenticatedChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, inputsize: u32, pinput: *const core::ffi::c_void, outputsize: u32, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::QueryAuthenticatedChannel(this, windows_core::from_raw_borrowed(&pchannel), core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&outputsize), core::mem::transmute_copy(&poutput)).into()
        }
        unsafe extern "system" fn ConfigureAuthenticatedChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, inputsize: u32, pinput: *const core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::ConfigureAuthenticatedChannel(this, windows_core::from_raw_borrowed(&pchannel), core::mem::transmute_copy(&inputsize), core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)).into()
        }
        unsafe extern "system" fn VideoProcessorSetStreamRotation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, rotation: D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorSetStreamRotation(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&rotation))
        }
        unsafe extern "system" fn VideoProcessorGetStreamRotation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext_Impl::VideoProcessorGetStreamRotation(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penable), core::mem::transmute_copy(&protation))
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDecoderBuffer: GetDecoderBuffer::<Identity, Impl, OFFSET>,
            ReleaseDecoderBuffer: ReleaseDecoderBuffer::<Identity, Impl, OFFSET>,
            DecoderBeginFrame: DecoderBeginFrame::<Identity, Impl, OFFSET>,
            DecoderEndFrame: DecoderEndFrame::<Identity, Impl, OFFSET>,
            SubmitDecoderBuffers: SubmitDecoderBuffers::<Identity, Impl, OFFSET>,
            DecoderExtension: DecoderExtension::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputTargetRect: VideoProcessorSetOutputTargetRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputBackgroundColor: VideoProcessorSetOutputBackgroundColor::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace: VideoProcessorSetOutputColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputAlphaFillMode: VideoProcessorSetOutputAlphaFillMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputConstriction: VideoProcessorSetOutputConstriction::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputStereoMode: VideoProcessorSetOutputStereoMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputExtension: VideoProcessorSetOutputExtension::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputTargetRect: VideoProcessorGetOutputTargetRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputBackgroundColor: VideoProcessorGetOutputBackgroundColor::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace: VideoProcessorGetOutputColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputAlphaFillMode: VideoProcessorGetOutputAlphaFillMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputConstriction: VideoProcessorGetOutputConstriction::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputStereoMode: VideoProcessorGetOutputStereoMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputExtension: VideoProcessorGetOutputExtension::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamFrameFormat: VideoProcessorSetStreamFrameFormat::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace: VideoProcessorSetStreamColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamOutputRate: VideoProcessorSetStreamOutputRate::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamSourceRect: VideoProcessorSetStreamSourceRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamDestRect: VideoProcessorSetStreamDestRect::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamAlpha: VideoProcessorSetStreamAlpha::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamPalette: VideoProcessorSetStreamPalette::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamPixelAspectRatio: VideoProcessorSetStreamPixelAspectRatio::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamLumaKey: VideoProcessorSetStreamLumaKey::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamStereoFormat: VideoProcessorSetStreamStereoFormat::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamAutoProcessingMode: VideoProcessorSetStreamAutoProcessingMode::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamFilter: VideoProcessorSetStreamFilter::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamExtension: VideoProcessorSetStreamExtension::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamFrameFormat: VideoProcessorGetStreamFrameFormat::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace: VideoProcessorGetStreamColorSpace::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamOutputRate: VideoProcessorGetStreamOutputRate::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamSourceRect: VideoProcessorGetStreamSourceRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamDestRect: VideoProcessorGetStreamDestRect::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamAlpha: VideoProcessorGetStreamAlpha::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamPalette: VideoProcessorGetStreamPalette::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamPixelAspectRatio: VideoProcessorGetStreamPixelAspectRatio::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamLumaKey: VideoProcessorGetStreamLumaKey::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamStereoFormat: VideoProcessorGetStreamStereoFormat::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamAutoProcessingMode: VideoProcessorGetStreamAutoProcessingMode::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamFilter: VideoProcessorGetStreamFilter::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamExtension: VideoProcessorGetStreamExtension::<Identity, Impl, OFFSET>,
            VideoProcessorBlt: VideoProcessorBlt::<Identity, Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchange: NegotiateCryptoSessionKeyExchange::<Identity, Impl, OFFSET>,
            EncryptionBlt: EncryptionBlt::<Identity, Impl, OFFSET>,
            DecryptionBlt: DecryptionBlt::<Identity, Impl, OFFSET>,
            StartSessionKeyRefresh: StartSessionKeyRefresh::<Identity, Impl, OFFSET>,
            FinishSessionKeyRefresh: FinishSessionKeyRefresh::<Identity, Impl, OFFSET>,
            GetEncryptionBltKey: GetEncryptionBltKey::<Identity, Impl, OFFSET>,
            NegotiateAuthenticatedChannelKeyExchange: NegotiateAuthenticatedChannelKeyExchange::<Identity, Impl, OFFSET>,
            QueryAuthenticatedChannel: QueryAuthenticatedChannel::<Identity, Impl, OFFSET>,
            ConfigureAuthenticatedChannel: ConfigureAuthenticatedChannel::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamRotation: VideoProcessorSetStreamRotation::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamRotation: VideoProcessorGetStreamRotation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoContext1_Impl: Sized + ID3D11VideoContext_Impl {
    fn SubmitDecoderBuffers1(&self, pdecoder: Option<&ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> windows_core::Result<()>;
    fn GetDataForNewHardwareKey(&self, pcryptosession: Option<&ID3D11CryptoSession>, privateinputsize: u32, pprivatinputdata: *const core::ffi::c_void) -> windows_core::Result<u64>;
    fn CheckCryptoSessionStatus(&self, pcryptosession: Option<&ID3D11CryptoSession>) -> windows_core::Result<D3D11_CRYPTO_SESSION_STATUS>;
    fn DecoderEnableDownsampling(&self, pdecoder: Option<&ID3D11VideoDecoder>, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> windows_core::Result<()>;
    fn DecoderUpdateDownsampling(&self, pdecoder: Option<&ID3D11VideoDecoder>, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::Result<()>;
    fn VideoProcessorSetOutputColorSpace1(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetOutputShaderUsage(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, shaderusage: super::super::Foundation::BOOL);
    fn VideoProcessorGetOutputColorSpace1(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetOutputShaderUsage(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, pshaderusage: *mut super::super::Foundation::BOOL);
    fn VideoProcessorSetStreamColorSpace1(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorSetStreamMirror(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL);
    fn VideoProcessorGetStreamColorSpace1(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE);
    fn VideoProcessorGetStreamMirror(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL);
    fn VideoProcessorGetBehaviorHints(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoContext1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoContext1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>() -> ID3D11VideoContext1_Vtbl {
        unsafe extern "system" fn SubmitDecoderBuffers1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC1) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::SubmitDecoderBuffers1(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&pbufferdesc)).into()
        }
        unsafe extern "system" fn GetDataForNewHardwareKey<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, privateinputsize: u32, pprivatinputdata: *const core::ffi::c_void, pprivateoutputdata: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoContext1_Impl::GetDataForNewHardwareKey(this, windows_core::from_raw_borrowed(&pcryptosession), core::mem::transmute_copy(&privateinputsize), core::mem::transmute_copy(&pprivatinputdata)) {
                Ok(ok__) => {
                    core::ptr::write(pprivateoutputdata, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCryptoSessionStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, pstatus: *mut D3D11_CRYPTO_SESSION_STATUS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoContext1_Impl::CheckCryptoSessionStatus(this, windows_core::from_raw_borrowed(&pcryptosession)) {
                Ok(ok__) => {
                    core::ptr::write(pstatus, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecoderEnableDownsampling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::DecoderEnableDownsampling(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&poutputdesc), core::mem::transmute_copy(&referenceframecount)).into()
        }
        unsafe extern "system" fn DecoderUpdateDownsampling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::DecoderUpdateDownsampling(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&poutputdesc)).into()
        }
        unsafe extern "system" fn VideoProcessorSetOutputColorSpace1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorSetOutputColorSpace1(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&colorspace))
        }
        unsafe extern "system" fn VideoProcessorSetOutputShaderUsage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, shaderusage: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorSetOutputShaderUsage(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&shaderusage))
        }
        unsafe extern "system" fn VideoProcessorGetOutputColorSpace1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorGetOutputColorSpace1(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetOutputShaderUsage<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, pshaderusage: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorGetOutputShaderUsage(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&pshaderusage))
        }
        unsafe extern "system" fn VideoProcessorSetStreamColorSpace1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorSetStreamColorSpace1(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&colorspace))
        }
        unsafe extern "system" fn VideoProcessorSetStreamMirror<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, enable: super::super::Foundation::BOOL, fliphorizontal: super::super::Foundation::BOOL, flipvertical: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorSetStreamMirror(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&enable), core::mem::transmute_copy(&fliphorizontal), core::mem::transmute_copy(&flipvertical))
        }
        unsafe extern "system" fn VideoProcessorGetStreamColorSpace1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, pcolorspace: *mut super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorGetStreamColorSpace1(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&pcolorspace))
        }
        unsafe extern "system" fn VideoProcessorGetStreamMirror<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext1_Impl::VideoProcessorGetStreamMirror(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&penable), core::mem::transmute_copy(&pfliphorizontal), core::mem::transmute_copy(&pflipvertical))
        }
        unsafe extern "system" fn VideoProcessorGetBehaviorHints<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, streamcount: u32, pstreams: *const D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT, pbehaviorhints: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoContext1_Impl::VideoProcessorGetBehaviorHints(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&outputwidth), core::mem::transmute_copy(&outputheight), core::mem::transmute_copy(&outputformat), core::mem::transmute_copy(&streamcount), core::mem::transmute_copy(&pstreams)) {
                Ok(ok__) => {
                    core::ptr::write(pbehaviorhints, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11VideoContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubmitDecoderBuffers1: SubmitDecoderBuffers1::<Identity, Impl, OFFSET>,
            GetDataForNewHardwareKey: GetDataForNewHardwareKey::<Identity, Impl, OFFSET>,
            CheckCryptoSessionStatus: CheckCryptoSessionStatus::<Identity, Impl, OFFSET>,
            DecoderEnableDownsampling: DecoderEnableDownsampling::<Identity, Impl, OFFSET>,
            DecoderUpdateDownsampling: DecoderUpdateDownsampling::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputColorSpace1: VideoProcessorSetOutputColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorSetOutputShaderUsage: VideoProcessorSetOutputShaderUsage::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputColorSpace1: VideoProcessorGetOutputColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputShaderUsage: VideoProcessorGetOutputShaderUsage::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamColorSpace1: VideoProcessorSetStreamColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamMirror: VideoProcessorSetStreamMirror::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamColorSpace1: VideoProcessorGetStreamColorSpace1::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamMirror: VideoProcessorGetStreamMirror::<Identity, Impl, OFFSET>,
            VideoProcessorGetBehaviorHints: VideoProcessorGetBehaviorHints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11VideoContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoContext2_Impl: Sized + ID3D11VideoContext1_Impl {
    fn VideoProcessorSetOutputHDRMetaData(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void);
    fn VideoProcessorGetOutputHDRMetaData(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void);
    fn VideoProcessorSetStreamHDRMetaData(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void);
    fn VideoProcessorGetStreamHDRMetaData(&self, pvideoprocessor: Option<&ID3D11VideoProcessor>, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoContext2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoContext2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>() -> ID3D11VideoContext2_Vtbl {
        unsafe extern "system" fn VideoProcessorSetOutputHDRMetaData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext2_Impl::VideoProcessorSetOutputHDRMetaData(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&phdrmetadata))
        }
        unsafe extern "system" fn VideoProcessorGetOutputHDRMetaData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext2_Impl::VideoProcessorGetOutputHDRMetaData(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata))
        }
        unsafe extern "system" fn VideoProcessorSetStreamHDRMetaData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: *const core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext2_Impl::VideoProcessorSetStreamHDRMetaData(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&phdrmetadata))
        }
        unsafe extern "system" fn VideoProcessorGetStreamHDRMetaData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideoprocessor: *mut core::ffi::c_void, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext2_Impl::VideoProcessorGetStreamHDRMetaData(this, windows_core::from_raw_borrowed(&pvideoprocessor), core::mem::transmute_copy(&streamindex), core::mem::transmute_copy(&ptype), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata))
        }
        Self {
            base__: ID3D11VideoContext1_Vtbl::new::<Identity, Impl, OFFSET>(),
            VideoProcessorSetOutputHDRMetaData: VideoProcessorSetOutputHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorGetOutputHDRMetaData: VideoProcessorGetOutputHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorSetStreamHDRMetaData: VideoProcessorSetStreamHDRMetaData::<Identity, Impl, OFFSET>,
            VideoProcessorGetStreamHDRMetaData: VideoProcessorGetStreamHDRMetaData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext2 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11VideoContext as windows_core::Interface>::IID || iid == &<ID3D11VideoContext1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoContext3_Impl: Sized + ID3D11VideoContext2_Impl {
    fn DecoderBeginFrame1(&self, pdecoder: Option<&ID3D11VideoDecoder>, pview: Option<&ID3D11VideoDecoderOutputView>, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const Option<ID3D11Buffer>) -> windows_core::Result<()>;
    fn SubmitDecoderBuffers2(&self, pdecoder: Option<&ID3D11VideoDecoder>, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoContext3 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoContext3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>() -> ID3D11VideoContext3_Vtbl {
        unsafe extern "system" fn DecoderBeginFrame1<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, pview: *mut core::ffi::c_void, contentkeysize: u32, pcontentkey: *const core::ffi::c_void, numcomponenthistograms: u32, phistogramoffsets: *const u32, pphistogrambuffers: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext3_Impl::DecoderBeginFrame1(this, windows_core::from_raw_borrowed(&pdecoder), windows_core::from_raw_borrowed(&pview), core::mem::transmute_copy(&contentkeysize), core::mem::transmute_copy(&pcontentkey), core::mem::transmute_copy(&numcomponenthistograms), core::mem::transmute_copy(&phistogramoffsets), core::mem::transmute_copy(&pphistogrambuffers)).into()
        }
        unsafe extern "system" fn SubmitDecoderBuffers2<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoder: *mut core::ffi::c_void, numbuffers: u32, pbufferdesc: *const D3D11_VIDEO_DECODER_BUFFER_DESC2) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoContext3_Impl::SubmitDecoderBuffers2(this, windows_core::from_raw_borrowed(&pdecoder), core::mem::transmute_copy(&numbuffers), core::mem::transmute_copy(&pbufferdesc)).into()
        }
        Self {
            base__: ID3D11VideoContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            DecoderBeginFrame1: DecoderBeginFrame1::<Identity, Impl, OFFSET>,
            SubmitDecoderBuffers2: SubmitDecoderBuffers2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoContext3 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11VideoContext as windows_core::Interface>::IID || iid == &<ID3D11VideoContext1 as windows_core::Interface>::IID || iid == &<ID3D11VideoContext2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoDecoder_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetCreationParameters(&self, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<()>;
    fn GetDriverHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoDecoder {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoDecoder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>() -> ID3D11VideoDecoder_Vtbl {
        unsafe extern "system" fn GetCreationParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodesc: *mut D3D11_VIDEO_DECODER_DESC, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDecoder_Impl::GetCreationParameters(this, core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pconfig)).into()
        }
        unsafe extern "system" fn GetDriverHandle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDecoder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdriverhandle: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDecoder_Impl::GetDriverHandle(this) {
                Ok(ok__) => {
                    core::ptr::write(pdriverhandle, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCreationParameters: GetCreationParameters::<Identity, Impl, OFFSET>,
            GetDriverHandle: GetDriverHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDecoder as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3D11VideoDecoderOutputView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC);
}
impl windows_core::RuntimeName for ID3D11VideoDecoderOutputView {}
impl ID3D11VideoDecoderOutputView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>() -> ID3D11VideoDecoderOutputView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDecoderOutputView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDecoderOutputView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDecoderOutputView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoDevice_Impl: Sized {
    fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<ID3D11VideoDecoder>;
    fn CreateVideoProcessor(&self, penum: Option<&ID3D11VideoProcessorEnumerator>, rateconversionindex: u32) -> windows_core::Result<ID3D11VideoProcessor>;
    fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> windows_core::Result<ID3D11AuthenticatedChannel>;
    fn CreateCryptoSession(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID) -> windows_core::Result<ID3D11CryptoSession>;
    fn CreateVideoDecoderOutputView(&self, presource: Option<&ID3D11Resource>, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut Option<ID3D11VideoDecoderOutputView>) -> windows_core::Result<()>;
    fn CreateVideoProcessorInputView(&self, presource: Option<&ID3D11Resource>, penum: Option<&ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut Option<ID3D11VideoProcessorInputView>) -> windows_core::Result<()>;
    fn CreateVideoProcessorOutputView(&self, presource: Option<&ID3D11Resource>, penum: Option<&ID3D11VideoProcessorEnumerator>, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut Option<ID3D11VideoProcessorOutputView>) -> windows_core::Result<()>;
    fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<ID3D11VideoProcessorEnumerator>;
    fn GetVideoDecoderProfileCount(&self) -> u32;
    fn GetVideoDecoderProfile(&self, index: u32) -> windows_core::Result<windows_core::GUID>;
    fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const windows_core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> windows_core::Result<u32>;
    fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::Result<()>;
    fn GetContentProtectionCaps(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::Result<()>;
    fn CheckCryptoKeyExchange(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, index: u32) -> windows_core::Result<windows_core::GUID>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoDevice {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>() -> ID3D11VideoDevice_Vtbl {
        unsafe extern "system" fn CreateVideoDecoder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG, ppdecoder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CreateVideoDecoder(this, core::mem::transmute_copy(&pvideodesc), core::mem::transmute_copy(&pconfig)) {
                Ok(ok__) => {
                    core::ptr::write(ppdecoder, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoProcessor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penum: *mut core::ffi::c_void, rateconversionindex: u32, ppvideoprocessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CreateVideoProcessor(this, windows_core::from_raw_borrowed(&penum), core::mem::transmute_copy(&rateconversionindex)) {
                Ok(ok__) => {
                    core::ptr::write(ppvideoprocessor, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAuthenticatedChannel<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppauthenticatedchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CreateAuthenticatedChannel(this, core::mem::transmute_copy(&channeltype)) {
                Ok(ok__) => {
                    core::ptr::write(ppauthenticatedchannel, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCryptoSession<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID, ppcryptosession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CreateCryptoSession(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&pkeyexchangetype)) {
                Ok(ok__) => {
                    core::ptr::write(ppcryptosession, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVideoDecoderOutputView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::CreateVideoDecoderOutputView(this, windows_core::from_raw_borrowed(&presource), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppvdovview)).into()
        }
        unsafe extern "system" fn CreateVideoProcessorInputView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, penum: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::CreateVideoProcessorInputView(this, windows_core::from_raw_borrowed(&presource), windows_core::from_raw_borrowed(&penum), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppvpiview)).into()
        }
        unsafe extern "system" fn CreateVideoProcessorOutputView<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, penum: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::CreateVideoProcessorOutputView(this, windows_core::from_raw_borrowed(&presource), windows_core::from_raw_borrowed(&penum), core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&ppvpoview)).into()
        }
        unsafe extern "system" fn CreateVideoProcessorEnumerator<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CreateVideoProcessorEnumerator(this, core::mem::transmute_copy(&pdesc)) {
                Ok(ok__) => {
                    core::ptr::write(ppenum, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderProfileCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::GetVideoDecoderProfileCount(this)
        }
        unsafe extern "system" fn GetVideoDecoderProfile<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pdecoderprofile: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::GetVideoDecoderProfile(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pdecoderprofile, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoderprofile: *const windows_core::GUID, format: super::Dxgi::Common::DXGI_FORMAT, psupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CheckVideoDecoderFormat(this, core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    core::ptr::write(psupported, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfigCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, pcount: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::GetVideoDecoderConfigCount(this, core::mem::transmute_copy(&pdesc)) {
                Ok(ok__) => {
                    core::ptr::write(pcount, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoDecoderConfig<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::GetVideoDecoderConfig(this, core::mem::transmute_copy(&pdesc), core::mem::transmute_copy(&index), core::mem::transmute_copy(&pconfig)).into()
        }
        unsafe extern "system" fn GetContentProtectionCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::GetContentProtectionCaps(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn CheckCryptoKeyExchange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, index: u32, pkeyexchangetype: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice_Impl::CheckCryptoKeyExchange(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    core::ptr::write(pkeyexchangetype, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), windows_core::from_raw_borrowed(&pdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateVideoDecoder: CreateVideoDecoder::<Identity, Impl, OFFSET>,
            CreateVideoProcessor: CreateVideoProcessor::<Identity, Impl, OFFSET>,
            CreateAuthenticatedChannel: CreateAuthenticatedChannel::<Identity, Impl, OFFSET>,
            CreateCryptoSession: CreateCryptoSession::<Identity, Impl, OFFSET>,
            CreateVideoDecoderOutputView: CreateVideoDecoderOutputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorInputView: CreateVideoProcessorInputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorOutputView: CreateVideoProcessorOutputView::<Identity, Impl, OFFSET>,
            CreateVideoProcessorEnumerator: CreateVideoProcessorEnumerator::<Identity, Impl, OFFSET>,
            GetVideoDecoderProfileCount: GetVideoDecoderProfileCount::<Identity, Impl, OFFSET>,
            GetVideoDecoderProfile: GetVideoDecoderProfile::<Identity, Impl, OFFSET>,
            CheckVideoDecoderFormat: CheckVideoDecoderFormat::<Identity, Impl, OFFSET>,
            GetVideoDecoderConfigCount: GetVideoDecoderConfigCount::<Identity, Impl, OFFSET>,
            GetVideoDecoderConfig: GetVideoDecoderConfig::<Identity, Impl, OFFSET>,
            GetContentProtectionCaps: GetContentProtectionCaps::<Identity, Impl, OFFSET>,
            CheckCryptoKeyExchange: CheckCryptoKeyExchange::<Identity, Impl, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoDevice1_Impl: Sized + ID3D11VideoDevice_Impl {
    fn GetCryptoSessionPrivateDataSize(&self, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> windows_core::Result<()>;
    fn GetVideoDecoderCaps(&self, pdecoderprofile: *const windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn CheckVideoDecoderDownsampling(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RecommendVideoDecoderDownsampleParameters(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL) -> windows_core::Result<D3D11_VIDEO_SAMPLE_DESC>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoDevice1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoDevice1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>() -> ID3D11VideoDevice1_Vtbl {
        unsafe extern "system" fn GetCryptoSessionPrivateDataSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptotype: *const windows_core::GUID, pdecoderprofile: *const windows_core::GUID, pkeyexchangetype: *const windows_core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice1_Impl::GetCryptoSessionPrivateDataSize(this, core::mem::transmute_copy(&pcryptotype), core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&pkeyexchangetype), core::mem::transmute_copy(&pprivateinputsize), core::mem::transmute_copy(&pprivateoutputsize)).into()
        }
        unsafe extern "system" fn GetVideoDecoderCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdecoderprofile: *const windows_core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: *const windows_core::GUID, pdecodercaps: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice1_Impl::GetVideoDecoderCaps(this, core::mem::transmute_copy(&pdecoderprofile), core::mem::transmute_copy(&samplewidth), core::mem::transmute_copy(&sampleheight), core::mem::transmute_copy(&pframerate), core::mem::transmute_copy(&bitrate), core::mem::transmute_copy(&pcryptotype)) {
                Ok(ok__) => {
                    core::ptr::write(pdecodercaps, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckVideoDecoderDownsampling<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice1_Impl::CheckVideoDecoderDownsampling(this, core::mem::transmute_copy(&pinputdesc), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&pinputconfig), core::mem::transmute_copy(&pframerate), core::mem::transmute_copy(&poutputdesc), core::mem::transmute_copy(&psupported), core::mem::transmute_copy(&prealtimehint)).into()
        }
        unsafe extern "system" fn RecommendVideoDecoderDownsampleParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, precommendedoutputdesc: *mut D3D11_VIDEO_SAMPLE_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoDevice1_Impl::RecommendVideoDecoderDownsampleParameters(this, core::mem::transmute_copy(&pinputdesc), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&pinputconfig), core::mem::transmute_copy(&pframerate)) {
                Ok(ok__) => {
                    core::ptr::write(precommendedoutputdesc, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11VideoDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCryptoSessionPrivateDataSize: GetCryptoSessionPrivateDataSize::<Identity, Impl, OFFSET>,
            GetVideoDecoderCaps: GetVideoDecoderCaps::<Identity, Impl, OFFSET>,
            CheckVideoDecoderDownsampling: CheckVideoDecoderDownsampling::<Identity, Impl, OFFSET>,
            RecommendVideoDecoderDownsampleParameters: RecommendVideoDecoderDownsampleParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice1 as windows_core::Interface>::IID || iid == &<ID3D11VideoDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoDevice2_Impl: Sized + ID3D11VideoDevice1_Impl {
    fn CheckFeatureSupport(&self, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
    fn NegotiateCryptoSessionKeyExchangeMT(&self, pcryptosession: Option<&ID3D11CryptoSession>, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoDevice2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>() -> ID3D11VideoDevice2_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: D3D11_FEATURE_VIDEO, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice2_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        unsafe extern "system" fn NegotiateCryptoSessionKeyExchangeMT<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcryptosession: *mut core::ffi::c_void, flags: D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS, datasize: u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoDevice2_Impl::NegotiateCryptoSessionKeyExchangeMT(this, windows_core::from_raw_borrowed(&pcryptosession), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
        }
        Self {
            base__: ID3D11VideoDevice1_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET>,
            NegotiateCryptoSessionKeyExchangeMT: NegotiateCryptoSessionKeyExchangeMT::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoDevice2 as windows_core::Interface>::IID || iid == &<ID3D11VideoDevice as windows_core::Interface>::IID || iid == &<ID3D11VideoDevice1 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessor_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetContentDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC);
    fn GetRateConversionCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoProcessor {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoProcessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>() -> ID3D11VideoProcessor_Vtbl {
        unsafe extern "system" fn GetContentDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessor_Impl::GetContentDesc(this, core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn GetRateConversionCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessor_Impl::GetRateConversionCaps(this, core::mem::transmute_copy(&pcaps))
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContentDesc: GetContentDesc::<Identity, Impl, OFFSET>,
            GetRateConversionCaps: GetRateConversionCaps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessor as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessorEnumerator_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetVideoProcessorContentDesc(&self, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::Result<()>;
    fn CheckVideoProcessorFormat(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32>;
    fn GetVideoProcessorCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::Result<()>;
    fn GetVideoProcessorRateConversionCaps(&self, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::Result<()>;
    fn GetVideoProcessorCustomRate(&self, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::Result<()>;
    fn GetVideoProcessorFilterRange(&self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> windows_core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoProcessorEnumerator {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoProcessorEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorEnumerator_Vtbl {
        unsafe extern "system" fn GetVideoProcessorContentDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorContentDesc(this, core::mem::transmute_copy(&pcontentdesc)).into()
        }
        unsafe extern "system" fn CheckVideoProcessorFormat<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::Dxgi::Common::DXGI_FORMAT, pflags: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoProcessorEnumerator_Impl::CheckVideoProcessorFormat(this, core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    core::ptr::write(pflags, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoProcessorCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorCaps(this, core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetVideoProcessorRateConversionCaps<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorRateConversionCaps(this, core::mem::transmute_copy(&typeindex), core::mem::transmute_copy(&pcaps)).into()
        }
        unsafe extern "system" fn GetVideoProcessorCustomRate<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorCustomRate(this, core::mem::transmute_copy(&typeindex), core::mem::transmute_copy(&customrateindex), core::mem::transmute_copy(&prate)).into()
        }
        unsafe extern "system" fn GetVideoProcessorFilterRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: D3D11_VIDEO_PROCESSOR_FILTER, prange: *mut D3D11_VIDEO_PROCESSOR_FILTER_RANGE) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoProcessorEnumerator_Impl::GetVideoProcessorFilterRange(this, core::mem::transmute_copy(&filter)) {
                Ok(ok__) => {
                    core::ptr::write(prange, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetVideoProcessorContentDesc: GetVideoProcessorContentDesc::<Identity, Impl, OFFSET>,
            CheckVideoProcessorFormat: CheckVideoProcessorFormat::<Identity, Impl, OFFSET>,
            GetVideoProcessorCaps: GetVideoProcessorCaps::<Identity, Impl, OFFSET>,
            GetVideoProcessorRateConversionCaps: GetVideoProcessorRateConversionCaps::<Identity, Impl, OFFSET>,
            GetVideoProcessorCustomRate: GetVideoProcessorCustomRate::<Identity, Impl, OFFSET>,
            GetVideoProcessorFilterRange: GetVideoProcessorFilterRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ID3D11VideoProcessorEnumerator1_Impl: Sized + ID3D11VideoProcessorEnumerator_Impl {
    fn CheckVideoProcessorFormatConversion(&self, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ID3D11VideoProcessorEnumerator1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ID3D11VideoProcessorEnumerator1_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorEnumerator1_Vtbl {
        unsafe extern "system" fn CheckVideoProcessorFormatConversion<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorEnumerator1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputformat: super::Dxgi::Common::DXGI_FORMAT, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, outputformat: super::Dxgi::Common::DXGI_FORMAT, outputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, psupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match ID3D11VideoProcessorEnumerator1_Impl::CheckVideoProcessorFormatConversion(this, core::mem::transmute_copy(&inputformat), core::mem::transmute_copy(&inputcolorspace), core::mem::transmute_copy(&outputformat), core::mem::transmute_copy(&outputcolorspace)) {
                Ok(ok__) => {
                    core::ptr::write(psupported, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ID3D11VideoProcessorEnumerator_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckVideoProcessorFormatConversion: CheckVideoProcessorFormatConversion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorEnumerator1 as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11VideoProcessorEnumerator as windows_core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorInputView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC);
}
impl windows_core::RuntimeName for ID3D11VideoProcessorInputView {}
impl ID3D11VideoProcessorInputView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorInputView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorInputView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessorInputView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorInputView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
pub trait ID3D11VideoProcessorOutputView_Impl: Sized + ID3D11View_Impl {
    fn GetDesc(&self, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC);
}
impl windows_core::RuntimeName for ID3D11VideoProcessorOutputView {}
impl ID3D11VideoProcessorOutputView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>() -> ID3D11VideoProcessorOutputView_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11VideoProcessorOutputView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesc: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11VideoProcessorOutputView_Impl::GetDesc(this, core::mem::transmute_copy(&pdesc))
        }
        Self { base__: ID3D11View_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc: GetDesc::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11VideoProcessorOutputView as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<ID3D11View as windows_core::Interface>::IID
    }
}
pub trait ID3D11View_Impl: Sized + ID3D11DeviceChild_Impl {
    fn GetResource(&self, ppresource: *mut Option<ID3D11Resource>);
}
impl windows_core::RuntimeName for ID3D11View {}
impl ID3D11View_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11View_Impl, const OFFSET: isize>() -> ID3D11View_Vtbl {
        unsafe extern "system" fn GetResource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3D11View_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresource: *mut *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3D11View_Impl::GetResource(this, core::mem::transmute_copy(&ppresource))
        }
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>(), GetResource: GetResource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11View as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3DDeviceContextState_Impl: Sized + ID3D11DeviceChild_Impl {}
impl windows_core::RuntimeName for ID3DDeviceContextState {}
impl ID3DDeviceContextState_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DDeviceContextState_Impl, const OFFSET: isize>() -> ID3DDeviceContextState_Vtbl {
        Self { base__: ID3D11DeviceChild_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DDeviceContextState as windows_core::Interface>::IID || iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
pub trait ID3DUserDefinedAnnotation_Impl: Sized {
    fn BeginEvent(&self, name: &windows_core::PCWSTR) -> i32;
    fn EndEvent(&self) -> i32;
    fn SetMarker(&self, name: &windows_core::PCWSTR);
    fn GetStatus(&self) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for ID3DUserDefinedAnnotation {}
impl ID3DUserDefinedAnnotation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>() -> ID3DUserDefinedAnnotation_Vtbl {
        unsafe extern "system" fn BeginEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DUserDefinedAnnotation_Impl::BeginEvent(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn EndEvent<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DUserDefinedAnnotation_Impl::EndEvent(this)
        }
        unsafe extern "system" fn SetMarker<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DUserDefinedAnnotation_Impl::SetMarker(this, core::mem::transmute(&name))
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DUserDefinedAnnotation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DUserDefinedAnnotation_Impl::GetStatus(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginEvent: BeginEvent::<Identity, Impl, OFFSET>,
            EndEvent: EndEvent::<Identity, Impl, OFFSET>,
            SetMarker: SetMarker::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DUserDefinedAnnotation as windows_core::Interface>::IID
    }
}
pub trait ID3DX11FFT_Impl: Sized {
    fn SetForwardScale(&self, forwardscale: f32) -> windows_core::Result<()>;
    fn GetForwardScale(&self) -> f32;
    fn SetInverseScale(&self, inversescale: f32) -> windows_core::Result<()>;
    fn GetInverseScale(&self) -> f32;
    fn AttachBuffersAndPrecompute(&self, numtempbuffers: u32, pptempbuffers: *const Option<ID3D11UnorderedAccessView>, numprecomputebuffers: u32, ppprecomputebuffersizes: *const Option<ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
    fn ForwardTransform(&self, pinputbuffer: Option<&ID3D11UnorderedAccessView>, ppoutputbuffer: *mut Option<ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
    fn InverseTransform(&self, pinputbuffer: Option<&ID3D11UnorderedAccessView>, ppoutputbuffer: *mut Option<ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3DX11FFT {}
impl ID3DX11FFT_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>() -> ID3DX11FFT_Vtbl {
        unsafe extern "system" fn SetForwardScale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forwardscale: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::SetForwardScale(this, core::mem::transmute_copy(&forwardscale)).into()
        }
        unsafe extern "system" fn GetForwardScale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::GetForwardScale(this)
        }
        unsafe extern "system" fn SetInverseScale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inversescale: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::SetInverseScale(this, core::mem::transmute_copy(&inversescale)).into()
        }
        unsafe extern "system" fn GetInverseScale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::GetInverseScale(this)
        }
        unsafe extern "system" fn AttachBuffersAndPrecompute<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numtempbuffers: u32, pptempbuffers: *const *mut core::ffi::c_void, numprecomputebuffers: u32, ppprecomputebuffersizes: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::AttachBuffersAndPrecompute(this, core::mem::transmute_copy(&numtempbuffers), core::mem::transmute_copy(&pptempbuffers), core::mem::transmute_copy(&numprecomputebuffers), core::mem::transmute_copy(&ppprecomputebuffersizes)).into()
        }
        unsafe extern "system" fn ForwardTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputbuffer: *mut core::ffi::c_void, ppoutputbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::ForwardTransform(this, windows_core::from_raw_borrowed(&pinputbuffer), core::mem::transmute_copy(&ppoutputbuffer)).into()
        }
        unsafe extern "system" fn InverseTransform<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11FFT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputbuffer: *mut core::ffi::c_void, ppoutputbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11FFT_Impl::InverseTransform(this, windows_core::from_raw_borrowed(&pinputbuffer), core::mem::transmute_copy(&ppoutputbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetForwardScale: SetForwardScale::<Identity, Impl, OFFSET>,
            GetForwardScale: GetForwardScale::<Identity, Impl, OFFSET>,
            SetInverseScale: SetInverseScale::<Identity, Impl, OFFSET>,
            GetInverseScale: GetInverseScale::<Identity, Impl, OFFSET>,
            AttachBuffersAndPrecompute: AttachBuffersAndPrecompute::<Identity, Impl, OFFSET>,
            ForwardTransform: ForwardTransform::<Identity, Impl, OFFSET>,
            InverseTransform: InverseTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DX11FFT as windows_core::Interface>::IID
    }
}
pub trait ID3DX11Scan_Impl: Sized {
    fn SetScanDirection(&self, direction: D3DX11_SCAN_DIRECTION) -> windows_core::Result<()>;
    fn Scan(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: Option<&ID3D11UnorderedAccessView>, pdst: Option<&ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
    fn Multiscan(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: Option<&ID3D11UnorderedAccessView>, pdst: Option<&ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3DX11Scan {}
impl ID3DX11Scan_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: isize>() -> ID3DX11Scan_Vtbl {
        unsafe extern "system" fn SetScanDirection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11Scan_Impl::SetScanDirection(this, core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn Scan<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11Scan_Impl::Scan(this, core::mem::transmute_copy(&elementtype), core::mem::transmute_copy(&opcode), core::mem::transmute_copy(&elementscansize), windows_core::from_raw_borrowed(&psrc), windows_core::from_raw_borrowed(&pdst)).into()
        }
        unsafe extern "system" fn Multiscan<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11Scan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, elementscanpitch: u32, scancount: u32, psrc: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11Scan_Impl::Multiscan(this, core::mem::transmute_copy(&elementtype), core::mem::transmute_copy(&opcode), core::mem::transmute_copy(&elementscansize), core::mem::transmute_copy(&elementscanpitch), core::mem::transmute_copy(&scancount), windows_core::from_raw_borrowed(&psrc), windows_core::from_raw_borrowed(&pdst)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScanDirection: SetScanDirection::<Identity, Impl, OFFSET>,
            Scan: Scan::<Identity, Impl, OFFSET>,
            Multiscan: Multiscan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DX11Scan as windows_core::Interface>::IID
    }
}
pub trait ID3DX11SegmentedScan_Impl: Sized {
    fn SetScanDirection(&self, direction: D3DX11_SCAN_DIRECTION) -> windows_core::Result<()>;
    fn SegScan(&self, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: Option<&ID3D11UnorderedAccessView>, psrcelementflags: Option<&ID3D11UnorderedAccessView>, pdst: Option<&ID3D11UnorderedAccessView>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ID3DX11SegmentedScan {}
impl ID3DX11SegmentedScan_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>() -> ID3DX11SegmentedScan_Vtbl {
        unsafe extern "system" fn SetScanDirection<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: D3DX11_SCAN_DIRECTION) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11SegmentedScan_Impl::SetScanDirection(this, core::mem::transmute_copy(&direction)).into()
        }
        unsafe extern "system" fn SegScan<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: ID3DX11SegmentedScan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elementtype: D3DX11_SCAN_DATA_TYPE, opcode: D3DX11_SCAN_OPCODE, elementscansize: u32, psrc: *mut core::ffi::c_void, psrcelementflags: *mut core::ffi::c_void, pdst: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            ID3DX11SegmentedScan_Impl::SegScan(this, core::mem::transmute_copy(&elementtype), core::mem::transmute_copy(&opcode), core::mem::transmute_copy(&elementscansize), windows_core::from_raw_borrowed(&psrc), windows_core::from_raw_borrowed(&psrcelementflags), windows_core::from_raw_borrowed(&pdst)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScanDirection: SetScanDirection::<Identity, Impl, OFFSET>,
            SegScan: SegScan::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3DX11SegmentedScan as windows_core::Interface>::IID
    }
}
