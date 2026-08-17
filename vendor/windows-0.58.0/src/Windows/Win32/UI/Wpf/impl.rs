#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffect_Impl: Sized {
    fn GetOutput(&self, uiindex: u32, pcontext: Option<&IMILBitmapEffectRenderContext>) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetParentEffect(&self) -> windows_core::Result<IMILBitmapEffectGroup>;
    fn SetInputSource(&self, uiindex: u32, pbitmapsource: Option<&super::super::Graphics::Imaging::IWICBitmapSource>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl windows_core::RuntimeName for IMILBitmapEffect {}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffect_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffect_Vtbl
    where
        Identity: IMILBitmapEffect_Impl,
    {
        unsafe extern "system" fn GetOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pcontext: *mut core::ffi::c_void, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffect_Impl::GetOutput(this, core::mem::transmute_copy(&uiindex), windows_core::from_raw_borrowed(&pcontext)) {
                Ok(ok__) => {
                    ppbitmapsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparenteffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffect_Impl::GetParentEffect(this) {
                Ok(ok__) => {
                    ppparenteffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pbitmapsource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffect_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffect_Impl::SetInputSource(this, core::mem::transmute_copy(&uiindex), windows_core::from_raw_borrowed(&pbitmapsource)).into()
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
pub trait IMILBitmapEffectConnections_Impl: Sized {
    fn GetInputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector>;
    fn GetOutputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl windows_core::RuntimeName for IMILBitmapEffectConnections {}
impl IMILBitmapEffectConnections_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectConnections_Vtbl
    where
        Identity: IMILBitmapEffectConnections_Impl,
    {
        unsafe extern "system" fn GetInputConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnections_Impl::GetInputConnector(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnections_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnections_Impl::GetOutputConnector(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectConnectionsInfo_Impl: Sized {
    fn GetNumberInputs(&self) -> windows_core::Result<u32>;
    fn GetNumberOutputs(&self) -> windows_core::Result<u32>;
    fn GetInputConnectorInfo(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectConnectorInfo>;
    fn GetOutputConnectorInfo(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectConnectorInfo>;
}
impl windows_core::RuntimeName for IMILBitmapEffectConnectionsInfo {}
impl IMILBitmapEffectConnectionsInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectConnectionsInfo_Vtbl
    where
        Identity: IMILBitmapEffectConnectionsInfo_Impl,
    {
        unsafe extern "system" fn GetNumberInputs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinuminputs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectionsInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectionsInfo_Impl::GetNumberInputs(this) {
                Ok(ok__) => {
                    puinuminputs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberOutputs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinumoutputs: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectionsInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectionsInfo_Impl::GetNumberOutputs(this) {
                Ok(ok__) => {
                    puinumoutputs.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputConnectorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectionsInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectionsInfo_Impl::GetInputConnectorInfo(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnectorinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputConnectorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectionsInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectionsInfo_Impl::GetOutputConnectorInfo(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnectorinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectConnector_Impl: Sized + IMILBitmapEffectConnectorInfo_Impl {
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetBitmapEffect(&self) -> windows_core::Result<IMILBitmapEffect>;
}
impl windows_core::RuntimeName for IMILBitmapEffectConnector {}
impl IMILBitmapEffectConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectConnector_Vtbl
    where
        Identity: IMILBitmapEffectConnector_Impl,
    {
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnector_Impl::IsConnected(this) {
                Ok(ok__) => {
                    pfconnected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmapEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnector_Impl::GetBitmapEffect(this) {
                Ok(ok__) => {
                    ppeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectConnectorInfo_Impl: Sized {
    fn GetIndex(&self) -> windows_core::Result<u32>;
    fn GetOptimalFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetNumberFormats(&self) -> windows_core::Result<u32>;
    fn GetFormat(&self, ulindex: u32) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IMILBitmapEffectConnectorInfo {}
impl IMILBitmapEffectConnectorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectConnectorInfo_Vtbl
    where
        Identity: IMILBitmapEffectConnectorInfo_Impl,
    {
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiindex: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectorInfo_Impl::GetIndex(this) {
                Ok(ok__) => {
                    puiindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOptimalFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectorInfo_Impl::GetOptimalFormat(this) {
                Ok(ok__) => {
                    pformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberFormats<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnumberformats: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectorInfo_Impl::GetNumberFormats(this) {
                Ok(ok__) => {
                    pulnumberformats.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, pformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectConnectorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectConnectorInfo_Impl::GetFormat(this, core::mem::transmute_copy(&ulindex)) {
                Ok(ok__) => {
                    pformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectEvents_Impl: Sized {
    fn PropertyChange(&self, peffect: Option<&IMILBitmapEffect>, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DirtyRegion(&self, peffect: Option<&IMILBitmapEffect>, prect: *const MilRectD) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMILBitmapEffectEvents {}
impl IMILBitmapEffectEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectEvents_Vtbl
    where
        Identity: IMILBitmapEffectEvents_Impl,
    {
        unsafe extern "system" fn PropertyChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectEvents_Impl::PropertyChange(this, windows_core::from_raw_borrowed(&peffect), core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn DirtyRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, prect: *const MilRectD) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectEvents_Impl::DirtyRegion(this, windows_core::from_raw_borrowed(&peffect), core::mem::transmute_copy(&prect)).into()
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
pub trait IMILBitmapEffectFactory_Impl: Sized {
    fn CreateEffect(&self, pguideffect: *const windows_core::GUID) -> windows_core::Result<IMILBitmapEffect>;
    fn CreateContext(&self) -> windows_core::Result<IMILBitmapEffectRenderContext>;
    fn CreateEffectOuter(&self) -> windows_core::Result<IMILBitmapEffect>;
}
impl windows_core::RuntimeName for IMILBitmapEffectFactory {}
impl IMILBitmapEffectFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectFactory_Vtbl
    where
        Identity: IMILBitmapEffectFactory_Impl,
    {
        unsafe extern "system" fn CreateEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideffect: *const windows_core::GUID, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectFactory_Impl::CreateEffect(this, core::mem::transmute_copy(&pguideffect)) {
                Ok(ok__) => {
                    ppeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectFactory_Impl::CreateContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEffectOuter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectFactory_Impl::CreateEffectOuter(this) {
                Ok(ok__) => {
                    ppeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectGroup_Impl: Sized {
    fn GetInteriorInputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
    fn GetInteriorOutputConnector(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector>;
    fn Add(&self, peffect: Option<&IMILBitmapEffect>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMILBitmapEffectGroup {}
impl IMILBitmapEffectGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectGroup_Vtbl
    where
        Identity: IMILBitmapEffectGroup_Impl,
    {
        unsafe extern "system" fn GetInteriorInputConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectGroup_Impl::GetInteriorInputConnector(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInteriorOutputConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectGroup_Impl::GetInteriorOutputConnector(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectGroup_Impl::Add(this, windows_core::from_raw_borrowed(&peffect)).into()
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
pub trait IMILBitmapEffectGroupImpl_Impl: Sized {
    fn Preprocess(&self, pcontext: Option<&IMILBitmapEffectRenderContext>) -> windows_core::Result<()>;
    fn GetNumberChildren(&self) -> windows_core::Result<u32>;
    fn GetChildren(&self) -> windows_core::Result<IMILBitmapEffects>;
}
impl windows_core::RuntimeName for IMILBitmapEffectGroupImpl {}
impl IMILBitmapEffectGroupImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectGroupImpl_Vtbl
    where
        Identity: IMILBitmapEffectGroupImpl_Impl,
    {
        unsafe extern "system" fn Preprocess<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectGroupImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectGroupImpl_Impl::Preprocess(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn GetNumberChildren<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinumberchildren: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectGroupImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectGroupImpl_Impl::GetNumberChildren(this) {
                Ok(ok__) => {
                    puinumberchildren.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchildren: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectGroupImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectGroupImpl_Impl::GetChildren(this) {
                Ok(ok__) => {
                    pchildren.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
#[cfg(feature = "Win32_Graphics_Imaging")]
pub trait IMILBitmapEffectImpl_Impl: Sized {
    fn IsInPlaceModificationAllowed(&self, poutputconnector: Option<&IMILBitmapEffectOutputConnector>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetParentEffect(&self, pparenteffect: Option<&IMILBitmapEffectGroup>) -> windows_core::Result<()>;
    fn GetInputSource(&self, uiindex: u32) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> windows_core::Result<()>;
    fn GetInputBitmapSource(&self, uiindex: u32, prendercontext: Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn GetOutputBitmapSource(&self, uiindex: u32, prendercontext: Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn Initialize(&self, pinner: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl windows_core::RuntimeName for IMILBitmapEffectImpl {}
#[cfg(feature = "Win32_Graphics_Imaging")]
impl IMILBitmapEffectImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectImpl_Vtbl
    where
        Identity: IMILBitmapEffectImpl_Impl,
    {
        unsafe extern "system" fn IsInPlaceModificationAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputconnector: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectImpl_Impl::IsInPlaceModificationAllowed(this, windows_core::from_raw_borrowed(&poutputconnector)) {
                Ok(ok__) => {
                    pfmodifyinplace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentEffect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparenteffect: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectImpl_Impl::SetParentEffect(this, windows_core::from_raw_borrowed(&pparenteffect)).into()
        }
        unsafe extern "system" fn GetInputSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectImpl_Impl::GetInputSource(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppbitmapsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSourceBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectImpl_Impl::GetInputSourceBounds(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn GetInputBitmapSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, prendercontext: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectImpl_Impl::GetInputBitmapSource(this, core::mem::transmute_copy(&uiindex), windows_core::from_raw_borrowed(&prendercontext), core::mem::transmute_copy(&pfmodifyinplace)) {
                Ok(ok__) => {
                    ppbitmapsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputBitmapSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, prendercontext: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectImpl_Impl::GetOutputBitmapSource(this, core::mem::transmute_copy(&uiindex), windows_core::from_raw_borrowed(&prendercontext), core::mem::transmute_copy(&pfmodifyinplace)) {
                Ok(ok__) => {
                    ppbitmapsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinner: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectImpl_Impl::Initialize(this, windows_core::from_raw_borrowed(&pinner)).into()
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
pub trait IMILBitmapEffectInputConnector_Impl: Sized + IMILBitmapEffectConnector_Impl {
    fn ConnectTo(&self, pconnector: Option<&IMILBitmapEffectOutputConnector>) -> windows_core::Result<()>;
    fn GetConnection(&self) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl windows_core::RuntimeName for IMILBitmapEffectInputConnector {}
impl IMILBitmapEffectInputConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectInputConnector_Vtbl
    where
        Identity: IMILBitmapEffectInputConnector_Impl,
    {
        unsafe extern "system" fn ConnectTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnector: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectInputConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectInputConnector_Impl::ConnectTo(this, windows_core::from_raw_borrowed(&pconnector)).into()
        }
        unsafe extern "system" fn GetConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectInputConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectInputConnector_Impl::GetConnection(this) {
                Ok(ok__) => {
                    ppconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectInteriorInputConnector_Impl: Sized {
    fn GetInputConnector(&self) -> windows_core::Result<IMILBitmapEffectInputConnector>;
}
impl windows_core::RuntimeName for IMILBitmapEffectInteriorInputConnector {}
impl IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectInteriorInputConnector_Vtbl
    where
        Identity: IMILBitmapEffectInteriorInputConnector_Impl,
    {
        unsafe extern "system" fn GetInputConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectInteriorInputConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectInteriorInputConnector_Impl::GetInputConnector(this) {
                Ok(ok__) => {
                    pinputconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInputConnector: GetInputConnector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorInputConnector as windows_core::Interface>::IID
    }
}
pub trait IMILBitmapEffectInteriorOutputConnector_Impl: Sized {
    fn GetOutputConnector(&self) -> windows_core::Result<IMILBitmapEffectOutputConnector>;
}
impl windows_core::RuntimeName for IMILBitmapEffectInteriorOutputConnector {}
impl IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectInteriorOutputConnector_Vtbl
    where
        Identity: IMILBitmapEffectInteriorOutputConnector_Impl,
    {
        unsafe extern "system" fn GetOutputConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectInteriorOutputConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectInteriorOutputConnector_Impl::GetOutputConnector(this) {
                Ok(ok__) => {
                    poutputconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOutputConnector: GetOutputConnector::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectInteriorOutputConnector as windows_core::Interface>::IID
    }
}
pub trait IMILBitmapEffectOutputConnector_Impl: Sized + IMILBitmapEffectConnector_Impl {
    fn GetNumberConnections(&self) -> windows_core::Result<u32>;
    fn GetConnection(&self, uiindex: u32) -> windows_core::Result<IMILBitmapEffectInputConnector>;
}
impl windows_core::RuntimeName for IMILBitmapEffectOutputConnector {}
impl IMILBitmapEffectOutputConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectOutputConnector_Vtbl
    where
        Identity: IMILBitmapEffectOutputConnector_Impl,
    {
        unsafe extern "system" fn GetNumberConnections<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puinumberconnections: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectOutputConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectOutputConnector_Impl::GetNumberConnections(this) {
                Ok(ok__) => {
                    puinumberconnections.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectOutputConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectOutputConnector_Impl::GetConnection(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    ppconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMILBitmapEffectOutputConnectorImpl_Impl: Sized {
    fn AddBackLink(&self, pconnection: Option<&IMILBitmapEffectInputConnector>) -> windows_core::Result<()>;
    fn RemoveBackLink(&self, pconnection: Option<&IMILBitmapEffectInputConnector>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMILBitmapEffectOutputConnectorImpl {}
impl IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectOutputConnectorImpl_Vtbl
    where
        Identity: IMILBitmapEffectOutputConnectorImpl_Impl,
    {
        unsafe extern "system" fn AddBackLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectOutputConnectorImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectOutputConnectorImpl_Impl::AddBackLink(this, windows_core::from_raw_borrowed(&pconnection)).into()
        }
        unsafe extern "system" fn RemoveBackLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectOutputConnectorImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectOutputConnectorImpl_Impl::RemoveBackLink(this, windows_core::from_raw_borrowed(&pconnection)).into()
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
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
pub trait IMILBitmapEffectPrimitive_Impl: Sized {
    fn GetOutput(&self, uiindex: u32, pcontext: Option<&IMILBitmapEffectRenderContext>, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>;
    fn TransformPoint(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: Option<&IMILBitmapEffectRenderContext>, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TransformRect(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: Option<&IMILBitmapEffectRenderContext>) -> windows_core::Result<()>;
    fn HasAffineTransform(&self, uiindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn HasInverseTransform(&self, uiindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl windows_core::RuntimeName for IMILBitmapEffectPrimitive {}
#[cfg(all(feature = "Win32_Graphics_Dwm", feature = "Win32_Graphics_Imaging"))]
impl IMILBitmapEffectPrimitive_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectPrimitive_Vtbl
    where
        Identity: IMILBitmapEffectPrimitive_Impl,
    {
        unsafe extern "system" fn GetOutput<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pcontext: *mut core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectPrimitive_Impl::GetOutput(this, core::mem::transmute_copy(&uiindex), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pfmodifyinplace)) {
                Ok(ok__) => {
                    ppbitmapsource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransformPoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut core::ffi::c_void, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectPrimitive_Impl::TransformPoint(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&p), core::mem::transmute_copy(&fforwardtransform), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pfpointtransformed)).into()
        }
        unsafe extern "system" fn TransformRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectPrimitive_Impl::TransformRect(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&p), core::mem::transmute_copy(&fforwardtransform), windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn HasAffineTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pfaffine: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectPrimitive_Impl::HasAffineTransform(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    pfaffine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInverseTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pfhasinverse: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectPrimitive_Impl::HasInverseTransform(this, core::mem::transmute_copy(&uiindex)) {
                Ok(ok__) => {
                    pfhasinverse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAffineMatrix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitive_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectPrimitive_Impl::GetAffineMatrix(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&pmatrix)).into()
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
pub trait IMILBitmapEffectPrimitiveImpl_Impl: Sized {
    fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT;
    fn IsVolatile(&self, uioutputindex: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl windows_core::RuntimeName for IMILBitmapEffectPrimitiveImpl {}
impl IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectPrimitiveImpl_Vtbl
    where
        Identity: IMILBitmapEffectPrimitiveImpl_Impl,
    {
        unsafe extern "system" fn IsDirty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitiveImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectPrimitiveImpl_Impl::IsDirty(this, core::mem::transmute_copy(&uioutputindex), core::mem::transmute_copy(&pfdirty))
        }
        unsafe extern "system" fn IsVolatile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectPrimitiveImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectPrimitiveImpl_Impl::IsVolatile(this, core::mem::transmute_copy(&uioutputindex)) {
                Ok(ok__) => {
                    pfvolatile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsDirty: IsDirty::<Identity, OFFSET>, IsVolatile: IsVolatile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMILBitmapEffectPrimitiveImpl as windows_core::Interface>::IID
    }
}
pub trait IMILBitmapEffectRenderContext_Impl: Sized {
    fn SetOutputPixelFormat(&self, format: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetOutputPixelFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetUseSoftwareRenderer(&self, fsoftware: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> windows_core::Result<()>;
    fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> windows_core::Result<()>;
    fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> windows_core::Result<()>;
    fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> windows_core::Result<()>;
    fn SetRegionOfInterest(&self, prect: *const MilRectD) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMILBitmapEffectRenderContext {}
impl IMILBitmapEffectRenderContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectRenderContext_Vtbl
    where
        Identity: IMILBitmapEffectRenderContext_Impl,
    {
        unsafe extern "system" fn SetOutputPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::SetOutputPixelFormat(this, core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn GetOutputPixelFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectRenderContext_Impl::GetOutputPixelFormat(this) {
                Ok(ok__) => {
                    pformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseSoftwareRenderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsoftware: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::SetUseSoftwareRenderer(this, core::mem::transmute_copy(&fsoftware)).into()
        }
        unsafe extern "system" fn SetInitialTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const MILMatrixF) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::SetInitialTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetFinalTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut MILMatrixF) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::GetFinalTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn SetOutputDPI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::SetOutputDPI(this, core::mem::transmute_copy(&dbldpix), core::mem::transmute_copy(&dbldpiy)).into()
        }
        unsafe extern "system" fn GetOutputDPI<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::GetOutputDPI(this, core::mem::transmute_copy(&pdbldpix), core::mem::transmute_copy(&pdbldpiy)).into()
        }
        unsafe extern "system" fn SetRegionOfInterest<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const MilRectD) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContext_Impl::SetRegionOfInterest(this, core::mem::transmute_copy(&prect)).into()
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
pub trait IMILBitmapEffectRenderContextImpl_Impl: Sized {
    fn GetUseSoftwareRenderer(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> windows_core::Result<()>;
    fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> windows_core::Result<()>;
    fn GetOutputBounds(&self, prect: *mut MilRectD) -> windows_core::Result<()>;
    fn UpdateOutputBounds(&self, prect: *const MilRectD) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMILBitmapEffectRenderContextImpl {}
impl IMILBitmapEffectRenderContextImpl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffectRenderContextImpl_Vtbl
    where
        Identity: IMILBitmapEffectRenderContextImpl_Impl,
    {
        unsafe extern "system" fn GetUseSoftwareRenderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfsoftware: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContextImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffectRenderContextImpl_Impl::GetUseSoftwareRenderer(this) {
                Ok(ok__) => {
                    pfsoftware.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *mut MILMatrixF) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContextImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContextImpl_Impl::GetTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn UpdateTransform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmatrix: *const MILMatrixF) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContextImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContextImpl_Impl::UpdateTransform(this, core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetOutputBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut MilRectD) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContextImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContextImpl_Impl::GetOutputBounds(this, core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn UpdateOutputBounds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *const MilRectD) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffectRenderContextImpl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMILBitmapEffectRenderContextImpl_Impl::UpdateOutputBounds(this, core::mem::transmute_copy(&prect)).into()
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
pub trait IMILBitmapEffects_Impl: Sized {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Parent(&self) -> windows_core::Result<IMILBitmapEffectGroup>;
    fn Item(&self, uindex: u32) -> windows_core::Result<IMILBitmapEffect>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMILBitmapEffects {}
impl IMILBitmapEffects_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMILBitmapEffects_Vtbl
    where
        Identity: IMILBitmapEffects_Impl,
    {
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiureturn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffects_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppiureturn.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffects_Impl::Parent(this) {
                Ok(ok__) => {
                    ppeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, ppeffect: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffects_Impl::Item(this, core::mem::transmute_copy(&uindex)) {
                Ok(ok__) => {
                    ppeffect.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puicount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMILBitmapEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMILBitmapEffects_Impl::Count(this) {
                Ok(ok__) => {
                    puicount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
