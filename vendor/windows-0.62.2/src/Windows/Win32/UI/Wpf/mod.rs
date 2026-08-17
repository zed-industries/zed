pub const CLSID_MILBitmapEffectBevel: windows_core::GUID = windows_core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
pub const CLSID_MILBitmapEffectBlur: windows_core::GUID = windows_core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
pub const CLSID_MILBitmapEffectDropShadow: windows_core::GUID = windows_core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
pub const CLSID_MILBitmapEffectEmboss: windows_core::GUID = windows_core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
pub const CLSID_MILBitmapEffectGroup: windows_core::GUID = windows_core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
pub const CLSID_MILBitmapEffectOuterGlow: windows_core::GUID = windows_core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
windows_core::imp::define_interface!(IMILBitmapEffect, IMILBitmapEffect_Vtbl, 0x8a6ff321_c944_4a1b_9944_9954af301258);
windows_core::imp::interface_hierarchy!(IMILBitmapEffect, windows_core::IUnknown);
impl IMILBitmapEffect {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<P1>(&self, uiindex: u32, pcontext: P1) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P1: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), uiindex, pcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParentEffect(&self) -> windows_core::Result<IMILBitmapEffectGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<P1>(&self, uiindex: u32, pbitmapsource: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::Graphics::Imaging::IWICBitmapSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputSource)(windows_core::Interface::as_raw(self), uiindex, pbitmapsource.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffect_Impl: windows_core::IUnknownImpl {
    fn GetOutput(&self, uiindex: u32, pcontext: windows_core::Ref<IMILBitmapEffectRenderContext>) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetParentEffect(&self) -> windows_core::Result<IMILBitmapEffectGroup>;
    fn SetInputSource(&self, uiindex: u32, pbitmapsource: windows_core::Ref<super::super::Graphics::Imaging::IWICBitmapSource>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffect_Vtbl {
    pub const fn new<Identity: IMILBitmapEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutput<Identity: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pcontext: *mut core::ffi::c_void, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffect_Impl::GetOutput(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&pcontext)) {
                    Ok(ok__) => {
                        ppbitmapsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentEffect<Identity: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparenteffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffect_Impl::GetParentEffect(this) {
                    Ok(ok__) => {
                        ppparenteffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputSource<Identity: IMILBitmapEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pbitmapsource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffect_Impl::SetInputSource(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&pbitmapsource)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutput: GetOutput::<Identity, OFFSET>,
            GetParentEffect: GetParentEffect::<Identity, OFFSET>,
            SetInputSource: SetInputSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffect as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl windows_core::RuntimeName for IMILBitmapEffect {}
windows_core::imp::define_interface!(IMILBitmapEffectConnections, IMILBitmapEffectConnections_Vtbl, 0xc2b5d861_9b1a_4374_89b0_dec4874d6a81);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnections, windows_core::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputConnector)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectOutputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputConnector)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectConnections_Impl: windows_core::IUnknownImpl {
    fn GetInputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector>;
    fn GetOutputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectConnections_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectConnections_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputConnector<Identity: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnections_Impl::GetInputConnector(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputConnector<Identity: IMILBitmapEffectConnections_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnections_Impl::GetOutputConnector(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputConnector: GetInputConnector::<Identity, OFFSET>,
            GetOutputConnector: GetOutputConnector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnections as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectConnections {}
windows_core::imp::define_interface!(IMILBitmapEffectConnectionsInfo, IMILBitmapEffectConnectionsInfo_Vtbl, 0x476b538a_c765_4237_ba4a_d6a880ff0cfc);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnectionsInfo, windows_core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberInputs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNumberOutputs(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOutputs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectConnectorInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputConnectorInfo)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectConnectorInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputConnectorInfo)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectionsInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberInputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectConnectionsInfo_Impl: windows_core::IUnknownImpl {
    fn GetNumberInputs(&self) -> windows_core::Result<u32>;
    fn GetNumberOutputs(&self) -> windows_core::Result<u32>;
    fn GetInputConnectorInfo(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectConnectorInfo>;
    fn GetOutputConnectorInfo(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectConnectorInfo>;
}
impl IMILBitmapEffectConnectionsInfo_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberInputs<Identity: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinuminputs: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectionsInfo_Impl::GetNumberInputs(this) {
                    Ok(ok__) => {
                        puinuminputs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOutputs<Identity: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinumoutputs: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectionsInfo_Impl::GetNumberOutputs(this) {
                    Ok(ok__) => {
                        puinumoutputs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputConnectorInfo<Identity: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectionsInfo_Impl::GetInputConnectorInfo(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnectorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Identity: IMILBitmapEffectConnectionsInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectionsInfo_Impl::GetOutputConnectorInfo(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnectorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberInputs: GetNumberInputs::<Identity, OFFSET>,
            GetNumberOutputs: GetNumberOutputs::<Identity, OFFSET>,
            GetInputConnectorInfo: GetInputConnectorInfo::<Identity, OFFSET>,
            GetOutputConnectorInfo: GetOutputConnectorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectionsInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectConnectionsInfo {}
windows_core::imp::define_interface!(IMILBitmapEffectConnector, IMILBitmapEffectConnector_Vtbl, 0xf59567b3_76c1_4d47_ba1e_79f955e350ef);
impl core::ops::Deref for IMILBitmapEffectConnector {
    type Target = IMILBitmapEffectConnectorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnector, windows_core::IUnknown, IMILBitmapEffectConnectorInfo);
impl IMILBitmapEffectConnector {
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBitmapEffect(&self) -> windows_core::Result<IMILBitmapEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmapEffect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnector_Vtbl {
    pub base__: IMILBitmapEffectConnectorInfo_Vtbl,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetBitmapEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectConnector_Impl: IMILBitmapEffectConnectorInfo_Impl {
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetBitmapEffect(&self) -> windows_core::Result<IMILBitmapEffect>;
}
impl IMILBitmapEffectConnector_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsConnected<Identity: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnector_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pfconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitmapEffect<Identity: IMILBitmapEffectConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnector_Impl::GetBitmapEffect(this) {
                    Ok(ok__) => {
                        ppeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMILBitmapEffectConnectorInfo_Vtbl::new::<Identity, OFFSET>(),
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetBitmapEffect: GetBitmapEffect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnector as windows_core::Interface>::IID || iid == &<IMILBitmapEffectConnectorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectConnector {}
windows_core::imp::define_interface!(IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnectorInfo_Vtbl, 0xf66d2e4b_b46b_42fc_859e_3da0ecdb3c43);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnectorInfo, windows_core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOptimalFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptimalFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNumberFormats(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberFormats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectConnectorInfo_Impl: windows_core::IUnknownImpl {
    fn GetIndex(&self) -> windows_core::Result<u32>;
    fn GetOptimalFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetNumberFormats(&self) -> windows_core::Result<u32>;
    fn GetFormat(&self, ulindex: u32) -> windows_core::Result<windows_core::GUID>;
}
impl IMILBitmapEffectConnectorInfo_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIndex<Identity: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectorInfo_Impl::GetIndex(this) {
                    Ok(ok__) => {
                        puiindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOptimalFormat<Identity: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectorInfo_Impl::GetOptimalFormat(this) {
                    Ok(ok__) => {
                        pformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberFormats<Identity: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnumberformats: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectorInfo_Impl::GetNumberFormats(this) {
                    Ok(ok__) => {
                        pulnumberformats.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IMILBitmapEffectConnectorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, pformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectConnectorInfo_Impl::GetFormat(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        pformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetOptimalFormat: GetOptimalFormat::<Identity, OFFSET>,
            GetNumberFormats: GetNumberFormats::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectConnectorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectConnectorInfo {}
windows_core::imp::define_interface!(IMILBitmapEffectEvents, IMILBitmapEffectEvents_Vtbl, 0x2e880dd8_f8ce_457b_8199_d60bb3d7ef98);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectEvents, windows_core::IUnknown);
impl IMILBitmapEffectEvents {
    pub unsafe fn PropertyChange<P0>(&self, peffect: P0, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffect>,
    {
        unsafe { (windows_core::Interface::vtable(self).PropertyChange)(windows_core::Interface::as_raw(self), peffect.param().abi(), core::mem::transmute_copy(bstrpropertyname)).ok() }
    }
    pub unsafe fn DirtyRegion<P0>(&self, peffect: P0, prect: *const MilRectD) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffect>,
    {
        unsafe { (windows_core::Interface::vtable(self).DirtyRegion)(windows_core::Interface::as_raw(self), peffect.param().abi(), prect).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PropertyChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DirtyRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const MilRectD) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectEvents_Impl: windows_core::IUnknownImpl {
    fn PropertyChange(&self, peffect: windows_core::Ref<IMILBitmapEffect>, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DirtyRegion(&self, peffect: windows_core::Ref<IMILBitmapEffect>, prect: *const MilRectD) -> windows_core::Result<()>;
}
impl IMILBitmapEffectEvents_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyChange<Identity: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, bstrpropertyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectEvents_Impl::PropertyChange(this, core::mem::transmute_copy(&peffect), core::mem::transmute(&bstrpropertyname)).into()
            }
        }
        unsafe extern "system" fn DirtyRegion<Identity: IMILBitmapEffectEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, prect: *const MilRectD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectEvents_Impl::DirtyRegion(this, core::mem::transmute_copy(&peffect), core::mem::transmute_copy(&prect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropertyChange: PropertyChange::<Identity, OFFSET>,
            DirtyRegion: DirtyRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectEvents {}
windows_core::imp::define_interface!(IMILBitmapEffectFactory, IMILBitmapEffectFactory_Vtbl, 0x33a9df34_a403_4ec7_b07e_bc0682370845);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectFactory, windows_core::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const windows_core::GUID) -> windows_core::Result<IMILBitmapEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), pguideffect, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateContext(&self) -> windows_core::Result<IMILBitmapEffectRenderContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEffectOuter(&self) -> windows_core::Result<IMILBitmapEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffectOuter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectFactory_Impl: windows_core::IUnknownImpl {
    fn CreateEffect(&self, pguideffect: *const windows_core::GUID) -> windows_core::Result<IMILBitmapEffect>;
    fn CreateContext(&self) -> windows_core::Result<IMILBitmapEffectRenderContext>;
    fn CreateEffectOuter(&self) -> windows_core::Result<IMILBitmapEffect>;
}
impl IMILBitmapEffectFactory_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEffect<Identity: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideffect: *const windows_core::GUID, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectFactory_Impl::CreateEffect(this, core::mem::transmute_copy(&pguideffect)) {
                    Ok(ok__) => {
                        ppeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateContext<Identity: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectFactory_Impl::CreateContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEffectOuter<Identity: IMILBitmapEffectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectFactory_Impl::CreateEffectOuter(this) {
                    Ok(ok__) => {
                        ppeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            CreateContext: CreateContext::<Identity, OFFSET>,
            CreateEffectOuter: CreateEffectOuter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectFactory {}
windows_core::imp::define_interface!(IMILBitmapEffectGroup, IMILBitmapEffectGroup_Vtbl, 0x2f952360_698a_4ac6_81a1_bcfdf08eb8e8);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectGroup, windows_core::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectOutputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInteriorInputConnector)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInteriorOutputConnector)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, peffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffect>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), peffect.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInteriorInputConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectGroup_Impl: windows_core::IUnknownImpl {
    fn GetInteriorInputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
    fn GetInteriorOutputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector>;
    fn Add(&self, peffect: windows_core::Ref<IMILBitmapEffect>) -> windows_core::Result<()>;
}
impl IMILBitmapEffectGroup_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInteriorInputConnector<Identity: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectGroup_Impl::GetInteriorInputConnector(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Identity: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectGroup_Impl::GetInteriorOutputConnector(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IMILBitmapEffectGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectGroup_Impl::Add(this, core::mem::transmute_copy(&peffect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInteriorInputConnector: GetInteriorInputConnector::<Identity, OFFSET>,
            GetInteriorOutputConnector: GetInteriorOutputConnector::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectGroup {}
windows_core::imp::define_interface!(IMILBitmapEffectGroupImpl, IMILBitmapEffectGroupImpl_Vtbl, 0x78fed518_1cfc_4807_8b85_6b6e51398f62);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectGroupImpl, windows_core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Preprocess)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok() }
    }
    pub unsafe fn GetNumberChildren(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChildren(&self) -> windows_core::Result<IMILBitmapEffects> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroupImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Preprocess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectGroupImpl_Impl: windows_core::IUnknownImpl {
    fn Preprocess(&self, pcontext: windows_core::Ref<IMILBitmapEffectRenderContext>) -> windows_core::Result<()>;
    fn GetNumberChildren(&self) -> windows_core::Result<u32>;
    fn GetChildren(&self) -> windows_core::Result<IMILBitmapEffects>;
}
impl IMILBitmapEffectGroupImpl_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Preprocess<Identity: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectGroupImpl_Impl::Preprocess(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn GetNumberChildren<Identity: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinumberchildren: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectGroupImpl_Impl::GetNumberChildren(this) {
                    Ok(ok__) => {
                        puinumberchildren.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChildren<Identity: IMILBitmapEffectGroupImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildren: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectGroupImpl_Impl::GetChildren(this) {
                    Ok(ok__) => {
                        pchildren.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Preprocess: Preprocess::<Identity, OFFSET>,
            GetNumberChildren: GetNumberChildren::<Identity, OFFSET>,
            GetChildren: GetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectGroupImpl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectGroupImpl {}
windows_core::imp::define_interface!(IMILBitmapEffectImpl, IMILBitmapEffectImpl_Vtbl, 0xcc2468f2_9936_47be_b4af_06b5df5dbcbb);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectImpl, windows_core::IUnknown);
impl IMILBitmapEffectImpl {
    pub unsafe fn IsInPlaceModificationAllowed<P0>(&self, poutputconnector: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<IMILBitmapEffectOutputConnector>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInPlaceModificationAllowed)(windows_core::Interface::as_raw(self), poutputconnector.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParentEffect<P0>(&self, pparenteffect: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffectGroup>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetParentEffect)(windows_core::Interface::as_raw(self), pparenteffect.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputSource)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputSourceBounds)(windows_core::Interface::as_raw(self), uiindex, prect as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputBitmapSource<P1>(&self, uiindex: u32, prendercontext: P1, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P1: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputBitmapSource)(windows_core::Interface::as_raw(self), uiindex, prendercontext.param().abi(), pfmodifyinplace as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutputBitmapSource<P1>(&self, uiindex: u32, prendercontext: P1, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P1: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputBitmapSource)(windows_core::Interface::as_raw(self), uiindex, prendercontext.param().abi(), pfmodifyinplace as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Initialize<P0>(&self, pinner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pinner.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetParentEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MilRectD) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputBitmapSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputBitmapSource: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutputBitmapSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffectImpl_Impl: windows_core::IUnknownImpl {
    fn IsInPlaceModificationAllowed(&self, poutputconnector: windows_core::Ref<IMILBitmapEffectOutputConnector>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetParentEffect(&self, pparenteffect: windows_core::Ref<IMILBitmapEffectGroup>) -> windows_core::Result<()>;
    fn GetInputSource(&self, uiindex: u32) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> windows_core::Result<()>;
    fn GetInputBitmapSource(&self, uiindex: u32, prendercontext: windows_core::Ref<IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetOutputBitmapSource(&self, uiindex: u32, prendercontext: windows_core::Ref<IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn Initialize(&self, pinner: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffectImpl_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputconnector: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectImpl_Impl::IsInPlaceModificationAllowed(this, core::mem::transmute_copy(&poutputconnector)) {
                    Ok(ok__) => {
                        pfmodifyinplace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParentEffect<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparenteffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectImpl_Impl::SetParentEffect(this, core::mem::transmute_copy(&pparenteffect)).into()
            }
        }
        unsafe extern "system" fn GetInputSource<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectImpl_Impl::GetInputSource(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppbitmapsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputSourceBounds<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectImpl_Impl::GetInputSourceBounds(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn GetInputBitmapSource<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, prendercontext: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectImpl_Impl::GetInputBitmapSource(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&prendercontext), core::mem::transmute_copy(&pfmodifyinplace)) {
                    Ok(ok__) => {
                        ppbitmapsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputBitmapSource<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, prendercontext: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectImpl_Impl::GetOutputBitmapSource(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&prendercontext), core::mem::transmute_copy(&pfmodifyinplace)) {
                    Ok(ok__) => {
                        ppbitmapsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Initialize<Identity: IMILBitmapEffectImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinner: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectImpl_Impl::Initialize(this, core::mem::transmute_copy(&pinner)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInPlaceModificationAllowed: IsInPlaceModificationAllowed::<Identity, OFFSET>,
            SetParentEffect: SetParentEffect::<Identity, OFFSET>,
            GetInputSource: GetInputSource::<Identity, OFFSET>,
            GetInputSourceBounds: GetInputSourceBounds::<Identity, OFFSET>,
            GetInputBitmapSource: GetInputBitmapSource::<Identity, OFFSET>,
            GetOutputBitmapSource: GetOutputBitmapSource::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectImpl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl windows_core::RuntimeName for IMILBitmapEffectImpl {}
windows_core::imp::define_interface!(IMILBitmapEffectInputConnector, IMILBitmapEffectInputConnector_Vtbl, 0xa9b4ecaa_7a3c_45e7_8573_f4b81b60dd6c);
impl core::ops::Deref for IMILBitmapEffectInputConnector {
    type Target = IMILBitmapEffectConnector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMILBitmapEffectInputConnector, windows_core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn ConnectTo<P0>(&self, pconnector: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffectOutputConnector>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectTo)(windows_core::Interface::as_raw(self), pconnector.param().abi()).ok() }
    }
    pub unsafe fn GetConnection(&self) -> windows_core::Result<IMILBitmapEffectOutputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectInputConnector_Impl: IMILBitmapEffectConnector_Impl {
    fn ConnectTo(&self, pconnector: windows_core::Ref<IMILBitmapEffectOutputConnector>) -> windows_core::Result<()>;
    fn GetConnection(&self) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectInputConnector_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectTo<Identity: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnector: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectInputConnector_Impl::ConnectTo(this, core::mem::transmute_copy(&pconnector)).into()
            }
        }
        unsafe extern "system" fn GetConnection<Identity: IMILBitmapEffectInputConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectInputConnector_Impl::GetConnection(this) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMILBitmapEffectConnector_Vtbl::new::<Identity, OFFSET>(),
            ConnectTo: ConnectTo::<Identity, OFFSET>,
            GetConnection: GetConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectInputConnector as windows_core::Interface>::IID || iid == &<IMILBitmapEffectConnectorInfo as windows_core::Interface>::IID || iid == &<IMILBitmapEffectConnector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectInputConnector {}
windows_core::imp::define_interface!(IMILBitmapEffectInteriorInputConnector, IMILBitmapEffectInteriorInputConnector_Vtbl, 0x20287e9e_86a2_4e15_953d_eb1438a5b842);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectInteriorInputConnector, windows_core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> windows_core::Result<IMILBitmapEffectInputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputConnector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectInteriorInputConnector_Impl: windows_core::IUnknownImpl {
    fn GetInputConnector(&self) -> windows_core::Result<IMILBitmapEffectInputConnector>;
}
impl IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputConnector<Identity: IMILBitmapEffectInteriorInputConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectInteriorInputConnector_Impl::GetInputConnector(this) {
                    Ok(ok__) => {
                        pinputconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInputConnector: GetInputConnector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorInputConnector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectInteriorInputConnector {}
windows_core::imp::define_interface!(IMILBitmapEffectInteriorOutputConnector, IMILBitmapEffectInteriorOutputConnector_Vtbl, 0x00bbb6dc_acc9_4bfc_b344_8bee383dfefa);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectInteriorOutputConnector, windows_core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> windows_core::Result<IMILBitmapEffectOutputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputConnector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOutputConnector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectInteriorOutputConnector_Impl: windows_core::IUnknownImpl {
    fn GetOutputConnector(&self) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputConnector<Identity: IMILBitmapEffectInteriorOutputConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectInteriorOutputConnector_Impl::GetOutputConnector(this) {
                    Ok(ok__) => {
                        poutputconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOutputConnector: GetOutputConnector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorOutputConnector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectInteriorOutputConnector {}
windows_core::imp::define_interface!(IMILBitmapEffectOutputConnector, IMILBitmapEffectOutputConnector_Vtbl, 0x92957aad_841b_4866_82ec_8752468b07fd);
impl core::ops::Deref for IMILBitmapEffectOutputConnector {
    type Target = IMILBitmapEffectConnector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMILBitmapEffectOutputConnector, windows_core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetNumberConnections(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberConnections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnection)(windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub GetNumberConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectOutputConnector_Impl: IMILBitmapEffectConnector_Impl {
    fn GetNumberConnections(&self) -> windows_core::Result<u32>;
    fn GetConnection(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector>;
}
impl IMILBitmapEffectOutputConnector_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberConnections<Identity: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinumberconnections: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectOutputConnector_Impl::GetNumberConnections(this) {
                    Ok(ok__) => {
                        puinumberconnections.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnection<Identity: IMILBitmapEffectOutputConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectOutputConnector_Impl::GetConnection(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        ppconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMILBitmapEffectConnector_Vtbl::new::<Identity, OFFSET>(),
            GetNumberConnections: GetNumberConnections::<Identity, OFFSET>,
            GetConnection: GetConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnector as windows_core::Interface>::IID || iid == &<IMILBitmapEffectConnectorInfo as windows_core::Interface>::IID || iid == &<IMILBitmapEffectConnector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectOutputConnector {}
windows_core::imp::define_interface!(IMILBitmapEffectOutputConnectorImpl, IMILBitmapEffectOutputConnectorImpl_Vtbl, 0x21fae777_8b39_4bfa_9f2d_f3941ed36913);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectOutputConnectorImpl, windows_core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<P0>(&self, pconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffectInputConnector>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddBackLink)(windows_core::Interface::as_raw(self), pconnection.param().abi()).ok() }
    }
    pub unsafe fn RemoveBackLink<P0>(&self, pconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMILBitmapEffectInputConnector>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveBackLink)(windows_core::Interface::as_raw(self), pconnection.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddBackLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectOutputConnectorImpl_Impl: windows_core::IUnknownImpl {
    fn AddBackLink(&self, pconnection: windows_core::Ref<IMILBitmapEffectInputConnector>) -> windows_core::Result<()>;
    fn RemoveBackLink(&self, pconnection: windows_core::Ref<IMILBitmapEffectInputConnector>) -> windows_core::Result<()>;
}
impl IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddBackLink<Identity: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectOutputConnectorImpl_Impl::AddBackLink(this, core::mem::transmute_copy(&pconnection)).into()
            }
        }
        unsafe extern "system" fn RemoveBackLink<Identity: IMILBitmapEffectOutputConnectorImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectOutputConnectorImpl_Impl::RemoveBackLink(this, core::mem::transmute_copy(&pconnection)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddBackLink: AddBackLink::<Identity, OFFSET>,
            RemoveBackLink: RemoveBackLink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectOutputConnectorImpl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectOutputConnectorImpl {}
windows_core::imp::define_interface!(IMILBitmapEffectPrimitive, IMILBitmapEffectPrimitive_Vtbl, 0x67e31025_3091_4dfc_98d6_dd494551461d);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectPrimitive, windows_core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<P1>(&self, uiindex: u32, pcontext: P1, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P1: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), uiindex, pcontext.param().abi(), pfmodifyinplace as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TransformPoint<P3>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: P3, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransformPoint)(windows_core::Interface::as_raw(self), uiindex, p as _, fforwardtransform, pcontext.param().abi(), pfpointtransformed as _).ok() }
    }
    pub unsafe fn TransformRect<P3>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<IMILBitmapEffectRenderContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransformRect)(windows_core::Interface::as_raw(self), uiindex, p as _, fforwardtransform, pcontext.param().abi()).ok() }
    }
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasAffineTransform)(windows_core::Interface::as_raw(self), uiindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasInverseTransform)(windows_core::Interface::as_raw(self), uiindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAffineMatrix)(windows_core::Interface::as_raw(self), uiindex, pmatrix as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitive_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub TransformPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MilPoint2D, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TransformRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MilRectD, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasAffineTransform: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub HasInverseTransform: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub GetAffineMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))]
    GetAffineMatrix: usize,
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectPrimitive_Impl: windows_core::IUnknownImpl {
    fn GetOutput(&self, uiindex: u32, pcontext: windows_core::Ref<IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn TransformPoint(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: windows_core::Ref<IMILBitmapEffectRenderContext>, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TransformRect(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: windows_core::Ref<IMILBitmapEffectRenderContext>) -> windows_core::Result<()>;
    fn HasAffineTransform(&self, uiindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn HasInverseTransform(&self, uiindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl IMILBitmapEffectPrimitive_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutput<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pcontext: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectPrimitive_Impl::GetOutput(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pfmodifyinplace)) {
                    Ok(ok__) => {
                        ppbitmapsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransformPoint<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut core::ffi::c_void, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectPrimitive_Impl::TransformPoint(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&p), core::mem::transmute_copy(&fforwardtransform), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pfpointtransformed)).into()
            }
        }
        unsafe extern "system" fn TransformRect<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectPrimitive_Impl::TransformRect(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&p), core::mem::transmute_copy(&fforwardtransform), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn HasAffineTransform<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pfaffine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectPrimitive_Impl::HasAffineTransform(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        pfaffine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasInverseTransform<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pfhasinverse: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectPrimitive_Impl::HasInverseTransform(this, core::mem::transmute_copy(&uiindex)) {
                    Ok(ok__) => {
                        pfhasinverse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAffineMatrix<Identity: IMILBitmapEffectPrimitive_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectPrimitive_Impl::GetAffineMatrix(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutput: GetOutput::<Identity, OFFSET>,
            TransformPoint: TransformPoint::<Identity, OFFSET>,
            TransformRect: TransformRect::<Identity, OFFSET>,
            HasAffineTransform: HasAffineTransform::<Identity, OFFSET>,
            HasInverseTransform: HasInverseTransform::<Identity, OFFSET>,
            GetAffineMatrix: GetAffineMatrix::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitive as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for IMILBitmapEffectPrimitive {}
windows_core::imp::define_interface!(IMILBitmapEffectPrimitiveImpl, IMILBitmapEffectPrimitiveImpl_Vtbl, 0xce41e00b_efa6_44e7_b007_dd042e3ae126);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectPrimitiveImpl, windows_core::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    pub unsafe fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self), uioutputindex, pfdirty as _) }
    }
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsVolatile)(windows_core::Interface::as_raw(self), uioutputindex, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsVolatile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectPrimitiveImpl_Impl: windows_core::IUnknownImpl {
    fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT;
    fn IsVolatile(&self, uioutputindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectPrimitiveImpl_Impl::IsDirty(this, core::mem::transmute_copy(&uioutputindex), core::mem::transmute_copy(&pfdirty))
            }
        }
        unsafe extern "system" fn IsVolatile<Identity: IMILBitmapEffectPrimitiveImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectPrimitiveImpl_Impl::IsVolatile(this, core::mem::transmute_copy(&uioutputindex)) {
                    Ok(ok__) => {
                        pfvolatile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsDirty: IsDirty::<Identity, OFFSET>, IsVolatile: IsVolatile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitiveImpl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectPrimitiveImpl {}
windows_core::imp::define_interface!(IMILBitmapEffectRenderContext, IMILBitmapEffectRenderContext_Vtbl, 0x12a2ec7e_2d33_44b2_b334_1abb7846e390);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectRenderContext, windows_core::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputPixelFormat)(windows_core::Interface::as_raw(self), format).ok() }
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUseSoftwareRenderer(&self, fsoftware: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseSoftwareRenderer)(windows_core::Interface::as_raw(self), fsoftware).ok() }
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialTransform)(windows_core::Interface::as_raw(self), pmatrix).ok() }
    }
    pub unsafe fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFinalTransform)(windows_core::Interface::as_raw(self), pmatrix as _).ok() }
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputDPI)(windows_core::Interface::as_raw(self), dbldpix, dbldpiy).ok() }
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputDPI)(windows_core::Interface::as_raw(self), pdbldpix as _, pdbldpiy as _).ok() }
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRegionOfInterest)(windows_core::Interface::as_raw(self), prect).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOutputPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetOutputPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetUseSoftwareRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetInitialTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const MILMatrixF) -> windows_core::HRESULT,
    pub GetFinalTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MILMatrixF) -> windows_core::HRESULT,
    pub SetOutputDPI: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub GetOutputDPI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    pub SetRegionOfInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *const MilRectD) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectRenderContext_Impl: windows_core::IUnknownImpl {
    fn SetOutputPixelFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetOutputPixelFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetUseSoftwareRenderer(&self, fsoftware: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> windows_core::Result<()>;
    fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> windows_core::Result<()>;
    fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> windows_core::Result<()>;
    fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> windows_core::Result<()>;
    fn SetRegionOfInterest(&self, prect: *const MilRectD) -> windows_core::Result<()>;
}
impl IMILBitmapEffectRenderContext_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutputPixelFormat<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::SetOutputPixelFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn GetOutputPixelFormat<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectRenderContext_Impl::GetOutputPixelFormat(this) {
                    Ok(ok__) => {
                        pformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsoftware: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::SetUseSoftwareRenderer(this, core::mem::transmute_copy(&fsoftware)).into()
            }
        }
        unsafe extern "system" fn SetInitialTransform<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const MILMatrixF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::SetInitialTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetFinalTransform<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut MILMatrixF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::GetFinalTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn SetOutputDPI<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::SetOutputDPI(this, core::mem::transmute_copy(&dbldpix), core::mem::transmute_copy(&dbldpiy)).into()
            }
        }
        unsafe extern "system" fn GetOutputDPI<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::GetOutputDPI(this, core::mem::transmute_copy(&pdbldpix), core::mem::transmute_copy(&pdbldpiy)).into()
            }
        }
        unsafe extern "system" fn SetRegionOfInterest<Identity: IMILBitmapEffectRenderContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const MilRectD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContext_Impl::SetRegionOfInterest(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOutputPixelFormat: SetOutputPixelFormat::<Identity, OFFSET>,
            GetOutputPixelFormat: GetOutputPixelFormat::<Identity, OFFSET>,
            SetUseSoftwareRenderer: SetUseSoftwareRenderer::<Identity, OFFSET>,
            SetInitialTransform: SetInitialTransform::<Identity, OFFSET>,
            GetFinalTransform: GetFinalTransform::<Identity, OFFSET>,
            SetOutputDPI: SetOutputDPI::<Identity, OFFSET>,
            GetOutputDPI: GetOutputDPI::<Identity, OFFSET>,
            SetRegionOfInterest: SetRegionOfInterest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectRenderContext {}
windows_core::imp::define_interface!(IMILBitmapEffectRenderContextImpl, IMILBitmapEffectRenderContextImpl_Vtbl, 0x4d25accb_797d_4fd2_b128_dffeff84fcc3);
windows_core::imp::interface_hierarchy!(IMILBitmapEffectRenderContextImpl, windows_core::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    pub unsafe fn GetUseSoftwareRenderer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUseSoftwareRenderer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), pmatrix as _).ok() }
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateTransform)(windows_core::Interface::as_raw(self), pmatrix).ok() }
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOutputBounds)(windows_core::Interface::as_raw(self), prect as _).ok() }
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateOutputBounds)(windows_core::Interface::as_raw(self), prect).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContextImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MILMatrixF) -> windows_core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const MILMatrixF) -> windows_core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MilRectD) -> windows_core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *const MilRectD) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffectRenderContextImpl_Impl: windows_core::IUnknownImpl {
    fn GetUseSoftwareRenderer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> windows_core::Result<()>;
    fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> windows_core::Result<()>;
    fn GetOutputBounds(&self, prect: *mut MilRectD) -> windows_core::Result<()>;
    fn UpdateOutputBounds(&self, prect: *const MilRectD) -> windows_core::Result<()>;
}
impl IMILBitmapEffectRenderContextImpl_Vtbl {
    pub const fn new<Identity: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUseSoftwareRenderer<Identity: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsoftware: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffectRenderContextImpl_Impl::GetUseSoftwareRenderer(this) {
                    Ok(ok__) => {
                        pfsoftware.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransform<Identity: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut MILMatrixF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContextImpl_Impl::GetTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn UpdateTransform<Identity: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const MILMatrixF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContextImpl_Impl::UpdateTransform(this, core::mem::transmute_copy(&pmatrix)).into()
            }
        }
        unsafe extern "system" fn GetOutputBounds<Identity: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut MilRectD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContextImpl_Impl::GetOutputBounds(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        unsafe extern "system" fn UpdateOutputBounds<Identity: IMILBitmapEffectRenderContextImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const MilRectD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMILBitmapEffectRenderContextImpl_Impl::UpdateOutputBounds(this, core::mem::transmute_copy(&prect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUseSoftwareRenderer: GetUseSoftwareRenderer::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            UpdateTransform: UpdateTransform::<Identity, OFFSET>,
            GetOutputBounds: GetOutputBounds::<Identity, OFFSET>,
            UpdateOutputBounds: UpdateOutputBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectRenderContextImpl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffectRenderContextImpl {}
windows_core::imp::define_interface!(IMILBitmapEffects, IMILBitmapEffects_Vtbl, 0x51ac3dce_67c5_448b_9180_ad3eabddd5dd);
windows_core::imp::interface_hierarchy!(IMILBitmapEffects, windows_core::IUnknown);
impl IMILBitmapEffects {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<IMILBitmapEffectGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, uindex: u32) -> windows_core::Result<IMILBitmapEffect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffects_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMILBitmapEffects_Impl: windows_core::IUnknownImpl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Parent(&self) -> windows_core::Result<IMILBitmapEffectGroup>;
    fn Item(&self, uindex: u32) -> windows_core::Result<IMILBitmapEffect>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl IMILBitmapEffects_Vtbl {
    pub const fn new<Identity: IMILBitmapEffects_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiureturn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffects_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppiureturn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffects_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffects_Impl::Item(this, core::mem::transmute_copy(&uindex)) {
                    Ok(ok__) => {
                        ppeffect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IMILBitmapEffects_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puicount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMILBitmapEffects_Impl::Count(this) {
                    Ok(ok__) => {
                        puicount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffects as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMILBitmapEffects {}
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
