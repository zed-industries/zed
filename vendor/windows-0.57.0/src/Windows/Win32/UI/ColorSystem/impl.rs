pub trait IDeviceModelPlugIn_Impl: Sized {
    fn Initialize(&self, bstrxml: &windows_core::BSTR, cnummodels: u32, imodelposition: u32) -> windows_core::Result<()>;
    fn GetNumChannels(&self) -> windows_core::Result<u32>;
    fn DeviceToColorimetricColors(&self, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> windows_core::Result<()>;
    fn ColorimetricToDeviceColors(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF) -> windows_core::Result<f32>;
    fn ColorimetricToDeviceColorsWithBlack(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> windows_core::Result<f32>;
    fn SetTransformDeviceModelInfo(&self, imodelposition: u32, pidevicemodelother: Option<&IDeviceModelPlugIn>) -> windows_core::Result<()>;
    fn GetPrimarySamples(&self, pprimarycolor: *mut PrimaryXYZColors) -> windows_core::Result<()>;
    fn GetGamutBoundaryMeshSize(&self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> windows_core::Result<()>;
    fn GetGamutBoundaryMesh(&self, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> windows_core::Result<()>;
    fn GetNeutralAxisSize(&self) -> windows_core::Result<u32>;
    fn GetNeutralAxis(&self, ccolors: u32, pxyzcolors: *mut XYZColorF) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDeviceModelPlugIn {}
impl IDeviceModelPlugIn_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>() -> IDeviceModelPlugIn_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: core::mem::MaybeUninit<windows_core::BSTR>, cnummodels: u32, imodelposition: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::Initialize(this, core::mem::transmute(&bstrxml), core::mem::transmute_copy(&cnummodels), core::mem::transmute_copy(&imodelposition)).into()
        }
        unsafe extern "system" fn GetNumChannels<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumchannels: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceModelPlugIn_Impl::GetNumChannels(this) {
                Ok(ok__) => {
                    core::ptr::write(pnumchannels, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::DeviceToColorimetricColors(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pdevicevalues), core::mem::transmute_copy(&pxyzcolors)).into()
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceModelPlugIn_Impl::ColorimetricToDeviceColors(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pxyzcolors)) {
                Ok(ok__) => {
                    core::ptr::write(pdevicevalues, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceModelPlugIn_Impl::ColorimetricToDeviceColorsWithBlack(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pxyzcolors), core::mem::transmute_copy(&pblackinformation)) {
                Ok(ok__) => {
                    core::ptr::write(pdevicevalues, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imodelposition: u32, pidevicemodelother: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::SetTransformDeviceModelInfo(this, core::mem::transmute_copy(&imodelposition), windows_core::from_raw_borrowed(&pidevicemodelother)).into()
        }
        unsafe extern "system" fn GetPrimarySamples<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::GetPrimarySamples(this, core::mem::transmute_copy(&pprimarycolor)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::GetGamutBoundaryMeshSize(this, core::mem::transmute_copy(&pnumvertices), core::mem::transmute_copy(&pnumtriangles)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::GetGamutBoundaryMesh(this, core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&cvertices), core::mem::transmute_copy(&ctriangles), core::mem::transmute_copy(&pvertices), core::mem::transmute_copy(&ptriangles)).into()
        }
        unsafe extern "system" fn GetNeutralAxisSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccolors: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDeviceModelPlugIn_Impl::GetNeutralAxisSize(this) {
                Ok(ok__) => {
                    core::ptr::write(pccolors, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNeutralAxis<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDeviceModelPlugIn_Impl::GetNeutralAxis(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&pxyzcolors)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetNumChannels: GetNumChannels::<Identity, Impl, OFFSET>,
            DeviceToColorimetricColors: DeviceToColorimetricColors::<Identity, Impl, OFFSET>,
            ColorimetricToDeviceColors: ColorimetricToDeviceColors::<Identity, Impl, OFFSET>,
            ColorimetricToDeviceColorsWithBlack: ColorimetricToDeviceColorsWithBlack::<Identity, Impl, OFFSET>,
            SetTransformDeviceModelInfo: SetTransformDeviceModelInfo::<Identity, Impl, OFFSET>,
            GetPrimarySamples: GetPrimarySamples::<Identity, Impl, OFFSET>,
            GetGamutBoundaryMeshSize: GetGamutBoundaryMeshSize::<Identity, Impl, OFFSET>,
            GetGamutBoundaryMesh: GetGamutBoundaryMesh::<Identity, Impl, OFFSET>,
            GetNeutralAxisSize: GetNeutralAxisSize::<Identity, Impl, OFFSET>,
            GetNeutralAxis: GetNeutralAxis::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceModelPlugIn as windows_core::Interface>::IID
    }
}
pub trait IGamutMapModelPlugIn_Impl: Sized {
    fn Initialize(&self, bstrxml: &windows_core::BSTR, psrcplugin: Option<&IDeviceModelPlugIn>, pdestplugin: Option<&IDeviceModelPlugIn>, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> windows_core::Result<()>;
    fn SourceToDestinationAppearanceColors(&self, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IGamutMapModelPlugIn {}
impl IGamutMapModelPlugIn_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>() -> IGamutMapModelPlugIn_Vtbl {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: core::mem::MaybeUninit<windows_core::BSTR>, psrcplugin: *mut core::ffi::c_void, pdestplugin: *mut core::ffi::c_void, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGamutMapModelPlugIn_Impl::Initialize(this, core::mem::transmute(&bstrxml), windows_core::from_raw_borrowed(&psrcplugin), windows_core::from_raw_borrowed(&pdestplugin), core::mem::transmute_copy(&psrcgbd), core::mem::transmute_copy(&pdestgbd)).into()
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IGamutMapModelPlugIn_Impl::SourceToDestinationAppearanceColors(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&pinputcolors), core::mem::transmute_copy(&poutputcolors)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SourceToDestinationAppearanceColors: SourceToDestinationAppearanceColors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGamutMapModelPlugIn as windows_core::Interface>::IID
    }
}
