pub trait IDirectSound_Impl: Sized {
    fn CreateSoundBuffer(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut Option<IDirectSoundBuffer>, punkouter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCaps(&self, pdscaps: *mut DSCAPS) -> windows_core::Result<()>;
    fn DuplicateSoundBuffer(&self, pdsbufferoriginal: Option<&IDirectSoundBuffer>) -> windows_core::Result<IDirectSoundBuffer>;
    fn SetCooperativeLevel(&self, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> windows_core::Result<()>;
    fn Compact(&self) -> windows_core::Result<()>;
    fn GetSpeakerConfig(&self) -> windows_core::Result<u32>;
    fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> windows_core::Result<()>;
    fn Initialize(&self, pcguiddevice: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSound {}
impl IDirectSound_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSound_Vtbl
    where
        Identity: IDirectSound_Impl,
    {
        unsafe extern "system" fn CreateSoundBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound_Impl::CreateSoundBuffer(this, core::mem::transmute_copy(&pcdsbufferdesc), core::mem::transmute_copy(&ppdsbuffer), windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscaps: *mut DSCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound_Impl::GetCaps(this, core::mem::transmute_copy(&pdscaps)).into()
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsbufferoriginal: *mut core::ffi::c_void, ppdsbufferduplicate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound_Impl::DuplicateSoundBuffer(this, windows_core::from_raw_borrowed(&pdsbufferoriginal)) {
                Ok(ok__) => {
                    ppdsbufferduplicate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwlevel)).into()
        }
        unsafe extern "system" fn Compact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound_Impl::Compact(this).into()
        }
        unsafe extern "system" fn GetSpeakerConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwspeakerconfig: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound_Impl::GetSpeakerConfig(this) {
                Ok(ok__) => {
                    pdwspeakerconfig.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeakerConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspeakerconfig: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound_Impl::SetSpeakerConfig(this, core::mem::transmute_copy(&dwspeakerconfig)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcguiddevice: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectSound_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound_Impl::Initialize(this, core::mem::transmute_copy(&pcguiddevice)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSoundBuffer: CreateSoundBuffer::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            DuplicateSoundBuffer: DuplicateSoundBuffer::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            Compact: Compact::<Identity, OFFSET>,
            GetSpeakerConfig: GetSpeakerConfig::<Identity, OFFSET>,
            SetSpeakerConfig: SetSpeakerConfig::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DBuffer_Impl: Sized {
    fn GetAllParameters(&self, pds3dbuffer: *mut DS3DBUFFER) -> windows_core::Result<()>;
    fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> windows_core::Result<()>;
    fn GetConeOrientation(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetConeOutsideVolume(&self) -> windows_core::Result<i32>;
    fn GetMaxDistance(&self) -> windows_core::Result<f32>;
    fn GetMinDistance(&self) -> windows_core::Result<f32>;
    fn GetMode(&self) -> windows_core::Result<u32>;
    fn GetPosition(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetVelocity(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> windows_core::Result<()>;
    fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> windows_core::Result<()>;
    fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> windows_core::Result<()>;
    fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetMode(&self, dwmode: u32, dwapply: u32) -> windows_core::Result<()>;
    fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for IDirectSound3DBuffer {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSound3DBuffer_Vtbl
    where
        Identity: IDirectSound3DBuffer_Impl,
    {
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::GetAllParameters(this, core::mem::transmute_copy(&pds3dbuffer)).into()
        }
        unsafe extern "system" fn GetConeAngles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::GetConeAngles(this, core::mem::transmute_copy(&pdwinsideconeangle), core::mem::transmute_copy(&pdwoutsideconeangle)).into()
        }
        unsafe extern "system" fn GetConeOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetConeOrientation(this) {
                Ok(ok__) => {
                    pvorientation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeOutsideVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plconeoutsidevolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetConeOutsideVolume(this) {
                Ok(ok__) => {
                    plconeoutsidevolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDistance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflmaxdistance: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetMaxDistance(this) {
                Ok(ok__) => {
                    pflmaxdistance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDistance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflmindistance: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetMinDistance(this) {
                Ok(ok__) => {
                    pflmindistance.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmode: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetMode(this) {
                Ok(ok__) => {
                    pdwmode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetPosition(this) {
                Ok(ok__) => {
                    pvposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DBuffer_Impl::GetVelocity(this) {
                Ok(ok__) => {
                    pvvelocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcds3dbuffer), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeAngles<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetConeAngles(this, core::mem::transmute_copy(&dwinsideconeangle), core::mem::transmute_copy(&dwoutsideconeangle), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetConeOrientation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeOutsideVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetConeOutsideVolume(this, core::mem::transmute_copy(&lconeoutsidevolume), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMaxDistance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetMaxDistance(this, core::mem::transmute_copy(&flmaxdistance), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMinDistance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flmindistance: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetMinDistance(this, core::mem::transmute_copy(&flmindistance), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetMode(this, core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DBuffer_Impl::SetVelocity(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            GetConeAngles: GetConeAngles::<Identity, OFFSET>,
            GetConeOrientation: GetConeOrientation::<Identity, OFFSET>,
            GetConeOutsideVolume: GetConeOutsideVolume::<Identity, OFFSET>,
            GetMaxDistance: GetMaxDistance::<Identity, OFFSET>,
            GetMinDistance: GetMinDistance::<Identity, OFFSET>,
            GetMode: GetMode::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetVelocity: GetVelocity::<Identity, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            SetConeAngles: SetConeAngles::<Identity, OFFSET>,
            SetConeOrientation: SetConeOrientation::<Identity, OFFSET>,
            SetConeOutsideVolume: SetConeOutsideVolume::<Identity, OFFSET>,
            SetMaxDistance: SetMaxDistance::<Identity, OFFSET>,
            SetMinDistance: SetMinDistance::<Identity, OFFSET>,
            SetMode: SetMode::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetVelocity: SetVelocity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound3DBuffer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DListener_Impl: Sized {
    fn GetAllParameters(&self, plistener: *mut DS3DLISTENER) -> windows_core::Result<()>;
    fn GetDistanceFactor(&self) -> windows_core::Result<f32>;
    fn GetDopplerFactor(&self) -> windows_core::Result<f32>;
    fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::Result<()>;
    fn GetPosition(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetRolloffFactor(&self) -> windows_core::Result<f32>;
    fn GetVelocity(&self) -> windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> windows_core::Result<()>;
    fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> windows_core::Result<()>;
    fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::Result<()>;
    fn CommitDeferredSettings(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl windows_core::RuntimeName for IDirectSound3DListener {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSound3DListener_Vtbl
    where
        Identity: IDirectSound3DListener_Impl,
    {
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plistener: *mut DS3DLISTENER) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::GetAllParameters(this, core::mem::transmute_copy(&plistener)).into()
        }
        unsafe extern "system" fn GetDistanceFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfldistancefactor: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DListener_Impl::GetDistanceFactor(this) {
                Ok(ok__) => {
                    pfldistancefactor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDopplerFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfldopplerfactor: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DListener_Impl::GetDopplerFactor(this) {
                Ok(ok__) => {
                    pfldopplerfactor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::GetOrientation(this, core::mem::transmute_copy(&pvorientfront), core::mem::transmute_copy(&pvorienttop)).into()
        }
        unsafe extern "system" fn GetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DListener_Impl::GetPosition(this) {
                Ok(ok__) => {
                    pvposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRolloffFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflrollofffactor: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DListener_Impl::GetRolloffFactor(this) {
                Ok(ok__) => {
                    pflrollofffactor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound3DListener_Impl::GetVelocity(this) {
                Ok(ok__) => {
                    pvvelocity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetAllParameters(this, core::mem::transmute_copy(&pclistener), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetDistanceFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetDistanceFactor(this, core::mem::transmute_copy(&fldistancefactor), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetDopplerFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetDopplerFactor(this, core::mem::transmute_copy(&fldopplerfactor), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetOrientation(this, core::mem::transmute_copy(&xfront), core::mem::transmute_copy(&yfront), core::mem::transmute_copy(&zfront), core::mem::transmute_copy(&xtop), core::mem::transmute_copy(&ytop), core::mem::transmute_copy(&ztop), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetRolloffFactor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetRolloffFactor(this, core::mem::transmute_copy(&flrollofffactor), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetVelocity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::SetVelocity(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn CommitDeferredSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSound3DListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSound3DListener_Impl::CommitDeferredSettings(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            GetDistanceFactor: GetDistanceFactor::<Identity, OFFSET>,
            GetDopplerFactor: GetDopplerFactor::<Identity, OFFSET>,
            GetOrientation: GetOrientation::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetRolloffFactor: GetRolloffFactor::<Identity, OFFSET>,
            GetVelocity: GetVelocity::<Identity, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            SetDistanceFactor: SetDistanceFactor::<Identity, OFFSET>,
            SetDopplerFactor: SetDopplerFactor::<Identity, OFFSET>,
            SetOrientation: SetOrientation::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetRolloffFactor: SetRolloffFactor::<Identity, OFFSET>,
            SetVelocity: SetVelocity::<Identity, OFFSET>,
            CommitDeferredSettings: CommitDeferredSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound3DListener as windows_core::Interface>::IID
    }
}
pub trait IDirectSound8_Impl: Sized + IDirectSound_Impl {
    fn VerifyCertification(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDirectSound8 {}
impl IDirectSound8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSound8_Vtbl
    where
        Identity: IDirectSound8_Impl,
    {
        unsafe extern "system" fn VerifyCertification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcertified: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSound8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSound8_Impl::VerifyCertification(this) {
                Ok(ok__) => {
                    pdwcertified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IDirectSound_Vtbl::new::<Identity, OFFSET>(), VerifyCertification: VerifyCertification::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSound8 as windows_core::Interface>::IID || iid == &<IDirectSound as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundBuffer_Impl: Sized {
    fn GetCaps(&self, pdsbuffercaps: *mut DSBCAPS) -> windows_core::Result<()>;
    fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::Result<()>;
    fn GetVolume(&self) -> windows_core::Result<i32>;
    fn GetPan(&self) -> windows_core::Result<i32>;
    fn GetFrequency(&self) -> windows_core::Result<u32>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, pdirectsound: Option<&IDirectSound>, pcdsbufferdesc: *const DSBUFFERDESC) -> windows_core::Result<()>;
    fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> windows_core::Result<()>;
    fn SetCurrentPosition(&self, dwnewposition: u32) -> windows_core::Result<()>;
    fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetVolume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn SetPan(&self, lpan: i32) -> windows_core::Result<()>;
    fn SetFrequency(&self, dwfrequency: u32) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::Result<()>;
    fn Restore(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundBuffer {}
impl IDirectSoundBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundBuffer_Vtbl
    where
        Identity: IDirectSoundBuffer_Impl,
    {
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::GetCaps(this, core::mem::transmute_copy(&pdsbuffercaps)).into()
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pdwcurrentplaycursor), core::mem::transmute_copy(&pdwcurrentwritecursor)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::GetFormat(this, core::mem::transmute_copy(&pwfxformat), core::mem::transmute_copy(&dwsizeallocated), core::mem::transmute_copy(&pdwsizewritten)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundBuffer_Impl::GetVolume(this) {
                Ok(ok__) => {
                    plvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpan: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundBuffer_Impl::GetPan(this) {
                Ok(ok__) => {
                    plpan.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwfrequency: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundBuffer_Impl::GetFrequency(this) {
                Ok(ok__) => {
                    pdwfrequency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundBuffer_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsound: *mut core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::Initialize(this, windows_core::from_raw_borrowed(&pdirectsound), core::mem::transmute_copy(&pcdsbufferdesc)).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::Lock(this, core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwbytes), core::mem::transmute_copy(&ppvaudioptr1), core::mem::transmute_copy(&pdwaudiobytes1), core::mem::transmute_copy(&ppvaudioptr2), core::mem::transmute_copy(&pdwaudiobytes2), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Play<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::Play(this, core::mem::transmute_copy(&dwreserved1), core::mem::transmute_copy(&dwpriority), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnewposition: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::SetCurrentPosition(this, core::mem::transmute_copy(&dwnewposition)).into()
        }
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::SetFormat(this, core::mem::transmute_copy(&pcfxformat)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::SetVolume(this, core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn SetPan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpan: i32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::SetPan(this, core::mem::transmute_copy(&lpan)).into()
        }
        unsafe extern "system" fn SetFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfrequency: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::SetFrequency(this, core::mem::transmute_copy(&dwfrequency)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::Unlock(this, core::mem::transmute_copy(&pvaudioptr1), core::mem::transmute_copy(&dwaudiobytes1), core::mem::transmute_copy(&pvaudioptr2), core::mem::transmute_copy(&dwaudiobytes2)).into()
        }
        unsafe extern "system" fn Restore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer_Impl::Restore(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            GetPan: GetPan::<Identity, OFFSET>,
            GetFrequency: GetFrequency::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            SetPan: SetPan::<Identity, OFFSET>,
            SetFrequency: SetFrequency::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
            Restore: Restore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundBuffer as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundBuffer8_Impl: Sized + IDirectSoundBuffer_Impl {
    fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> windows_core::Result<()>;
    fn AcquireResources(&self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> windows_core::Result<()>;
    fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundBuffer8 {}
impl IDirectSoundBuffer8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundBuffer8_Vtbl
    where
        Identity: IDirectSoundBuffer8_Impl,
    {
        unsafe extern "system" fn SetFX<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer8_Impl::SetFX(this, core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdsfxdesc), core::mem::transmute_copy(&pdwresultcodes)).into()
        }
        unsafe extern "system" fn AcquireResources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer8_Impl::AcquireResources(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdwresultcodes)).into()
        }
        unsafe extern "system" fn GetObjectInPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundBuffer8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundBuffer8_Impl::GetObjectInPath(this, core::mem::transmute_copy(&rguidobject), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&rguidinterface), core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base__: IDirectSoundBuffer_Vtbl::new::<Identity, OFFSET>(),
            SetFX: SetFX::<Identity, OFFSET>,
            AcquireResources: AcquireResources::<Identity, OFFSET>,
            GetObjectInPath: GetObjectInPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundBuffer8 as windows_core::Interface>::IID || iid == &<IDirectSoundBuffer as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundCapture_Impl: Sized {
    fn CreateCaptureBuffer(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut Option<IDirectSoundCaptureBuffer>, punkouter: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCaps(&self) -> windows_core::Result<DSCCAPS>;
    fn Initialize(&self, pcguiddevice: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundCapture {}
impl IDirectSoundCapture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundCapture_Vtbl
    where
        Identity: IDirectSoundCapture_Impl,
    {
        unsafe extern "system" fn CreateCaptureBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCapture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCapture_Impl::CreateCaptureBuffer(this, core::mem::transmute_copy(&pcdscbufferdesc), core::mem::transmute_copy(&ppdscbuffer), windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCapture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundCapture_Impl::GetCaps(this) {
                Ok(ok__) => {
                    pdsccaps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcguiddevice: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCapture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCapture_Impl::Initialize(this, core::mem::transmute_copy(&pcguiddevice)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCaptureBuffer: CreateCaptureBuffer::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCapture as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureBuffer_Impl: Sized {
    fn GetCaps(&self) -> windows_core::Result<DSCBCAPS>;
    fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Initialize(&self, pdirectsoundcapture: Option<&IDirectSoundCapture>, pcdscbufferdesc: *const DSCBUFFERDESC) -> windows_core::Result<()>;
    fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::Result<()>;
    fn Start(&self, dwflags: u32) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundCaptureBuffer {}
impl IDirectSoundCaptureBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundCaptureBuffer_Vtbl
    where
        Identity: IDirectSoundCaptureBuffer_Impl,
    {
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundCaptureBuffer_Impl::GetCaps(this) {
                Ok(ok__) => {
                    pdscbcaps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::GetCurrentPosition(this, core::mem::transmute_copy(&pdwcaptureposition), core::mem::transmute_copy(&pdwreadposition)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::GetFormat(this, core::mem::transmute_copy(&pwfxformat), core::mem::transmute_copy(&dwsizeallocated), core::mem::transmute_copy(&pdwsizewritten)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundCaptureBuffer_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectsoundcapture: *mut core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::Initialize(this, windows_core::from_raw_borrowed(&pdirectsoundcapture), core::mem::transmute_copy(&pcdscbufferdesc)).into()
        }
        unsafe extern "system" fn Lock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::Lock(this, core::mem::transmute_copy(&dwoffset), core::mem::transmute_copy(&dwbytes), core::mem::transmute_copy(&ppvaudioptr1), core::mem::transmute_copy(&pdwaudiobytes1), core::mem::transmute_copy(&ppvaudioptr2), core::mem::transmute_copy(&pdwaudiobytes2), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::Start(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Unlock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvaudioptr1: *const core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const core::ffi::c_void, dwaudiobytes2: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer_Impl::Unlock(this, core::mem::transmute_copy(&pvaudioptr1), core::mem::transmute_copy(&dwaudiobytes1), core::mem::transmute_copy(&pvaudioptr2), core::mem::transmute_copy(&dwaudiobytes2)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Lock: Lock::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Unlock: Unlock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureBuffer8_Impl: Sized + IDirectSoundCaptureBuffer_Impl {
    fn GetObjectInPath(&self, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFXStatus(&self, dweffectscount: u32, pdwfxstatus: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundCaptureBuffer8 {}
impl IDirectSoundCaptureBuffer8_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundCaptureBuffer8_Vtbl
    where
        Identity: IDirectSoundCaptureBuffer8_Impl,
    {
        unsafe extern "system" fn GetObjectInPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidobject: *const windows_core::GUID, dwindex: u32, rguidinterface: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer8_Impl::GetObjectInPath(this, core::mem::transmute_copy(&rguidobject), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&rguidinterface), core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn GetFXStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureBuffer8_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureBuffer8_Impl::GetFXStatus(this, core::mem::transmute_copy(&dweffectscount), core::mem::transmute_copy(&pdwfxstatus)).into()
        }
        Self {
            base__: IDirectSoundCaptureBuffer_Vtbl::new::<Identity, OFFSET>(),
            GetObjectInPath: GetObjectInPath::<Identity, OFFSET>,
            GetFXStatus: GetFXStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer8 as windows_core::Interface>::IID || iid == &<IDirectSoundCaptureBuffer as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureFXAec_Impl: Sized {
    fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSCFXAec>;
    fn GetStatus(&self) -> windows_core::Result<u32>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundCaptureFXAec {}
impl IDirectSoundCaptureFXAec_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundCaptureFXAec_Vtbl
    where
        Identity: IDirectSoundCaptureFXAec_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXAec_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureFXAec_Impl::SetAllParameters(this, core::mem::transmute_copy(&pdscfxaec)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXAec_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundCaptureFXAec_Impl::GetAllParameters(this) {
                Ok(ok__) => {
                    pdscfxaec.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXAec_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundCaptureFXAec_Impl::GetStatus(this) {
                Ok(ok__) => {
                    pdwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXAec_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureFXAec_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXAec as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureFXNoiseSuppress_Impl: Sized {
    fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSCFXNoiseSuppress>;
    fn Reset(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundCaptureFXNoiseSuppress {}
impl IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundCaptureFXNoiseSuppress_Vtbl
    where
        Identity: IDirectSoundCaptureFXNoiseSuppress_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXNoiseSuppress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureFXNoiseSuppress_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdscfxnoisesuppress)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXNoiseSuppress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundCaptureFXNoiseSuppress_Impl::GetAllParameters(this) {
                Ok(ok__) => {
                    pdscfxnoisesuppress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundCaptureFXNoiseSuppress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundCaptureFXNoiseSuppress_Impl::Reset(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXNoiseSuppress as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXChorus_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxchorus: *mut DSFXChorus) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundFXChorus {}
impl IDirectSoundFXChorus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXChorus_Vtbl
    where
        Identity: IDirectSoundFXChorus_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXChorus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXChorus_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxchorus)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXChorus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXChorus_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxchorus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXChorus as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXCompressor_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxcompressor: *mut DSFXCompressor) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundFXCompressor {}
impl IDirectSoundFXCompressor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXCompressor_Vtbl
    where
        Identity: IDirectSoundFXCompressor_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXCompressor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXCompressor_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxcompressor)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXCompressor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXCompressor_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxcompressor)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXCompressor as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXDistortion_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxdistortion: *mut DSFXDistortion) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundFXDistortion {}
impl IDirectSoundFXDistortion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXDistortion_Vtbl
    where
        Identity: IDirectSoundFXDistortion_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXDistortion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXDistortion_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxdistortion)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXDistortion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXDistortion_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxdistortion)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXDistortion as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXEcho_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxecho: *mut DSFXEcho) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundFXEcho {}
impl IDirectSoundFXEcho_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXEcho_Vtbl
    where
        Identity: IDirectSoundFXEcho_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXEcho_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXEcho_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxecho)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXEcho_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXEcho_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxecho)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXEcho as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXFlanger_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxflanger: *mut DSFXFlanger) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundFXFlanger {}
impl IDirectSoundFXFlanger_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXFlanger_Vtbl
    where
        Identity: IDirectSoundFXFlanger_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXFlanger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXFlanger_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxflanger)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXFlanger_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXFlanger_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxflanger)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXFlanger as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXGargle_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXGargle>;
}
impl windows_core::RuntimeName for IDirectSoundFXGargle {}
impl IDirectSoundFXGargle_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXGargle_Vtbl
    where
        Identity: IDirectSoundFXGargle_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXGargle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXGargle_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxgargle)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXGargle_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundFXGargle_Impl::GetAllParameters(this) {
                Ok(ok__) => {
                    pdsfxgargle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXGargle as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXI3DL2Reverb_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> windows_core::Result<()>;
    fn SetPreset(&self, dwpreset: u32) -> windows_core::Result<()>;
    fn GetPreset(&self) -> windows_core::Result<u32>;
    fn SetQuality(&self, lquality: i32) -> windows_core::Result<()>;
    fn GetQuality(&self) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IDirectSoundFXI3DL2Reverb {}
impl IDirectSoundFXI3DL2Reverb_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXI3DL2Reverb_Vtbl
    where
        Identity: IDirectSoundFXI3DL2Reverb_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXI3DL2Reverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXI3DL2Reverb_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxi3dl2reverb)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXI3DL2Reverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXI3DL2Reverb_Impl::GetAllParameters(this, core::mem::transmute_copy(&pdsfxi3dl2reverb)).into()
        }
        unsafe extern "system" fn SetPreset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpreset: u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXI3DL2Reverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXI3DL2Reverb_Impl::SetPreset(this, core::mem::transmute_copy(&dwpreset)).into()
        }
        unsafe extern "system" fn GetPreset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpreset: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXI3DL2Reverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundFXI3DL2Reverb_Impl::GetPreset(this) {
                Ok(ok__) => {
                    pdwpreset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lquality: i32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXI3DL2Reverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXI3DL2Reverb_Impl::SetQuality(this, core::mem::transmute_copy(&lquality)).into()
        }
        unsafe extern "system" fn GetQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plquality: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXI3DL2Reverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundFXI3DL2Reverb_Impl::GetQuality(this) {
                Ok(ok__) => {
                    plquality.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
            SetPreset: SetPreset::<Identity, OFFSET>,
            GetPreset: GetPreset::<Identity, OFFSET>,
            SetQuality: SetQuality::<Identity, OFFSET>,
            GetQuality: GetQuality::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXI3DL2Reverb as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXParamEq_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXParamEq>;
}
impl windows_core::RuntimeName for IDirectSoundFXParamEq {}
impl IDirectSoundFXParamEq_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXParamEq_Vtbl
    where
        Identity: IDirectSoundFXParamEq_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXParamEq_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXParamEq_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxparameq)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXParamEq_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundFXParamEq_Impl::GetAllParameters(this) {
                Ok(ok__) => {
                    pdsfxparameq.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXParamEq as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFXWavesReverb_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> windows_core::Result<()>;
    fn GetAllParameters(&self) -> windows_core::Result<DSFXWavesReverb>;
}
impl windows_core::RuntimeName for IDirectSoundFXWavesReverb {}
impl IDirectSoundFXWavesReverb_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFXWavesReverb_Vtbl
    where
        Identity: IDirectSoundFXWavesReverb_Impl,
    {
        unsafe extern "system" fn SetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXWavesReverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFXWavesReverb_Impl::SetAllParameters(this, core::mem::transmute_copy(&pcdsfxwavesreverb)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFXWavesReverb_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectSoundFXWavesReverb_Impl::GetAllParameters(this) {
                Ok(ok__) => {
                    pdsfxwavesreverb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXWavesReverb as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundFullDuplex_Impl: Sized {
    fn Initialize(&self, pcaptureguid: *const windows_core::GUID, prenderguid: *const windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut Option<IDirectSoundBuffer8>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundFullDuplex {}
impl IDirectSoundFullDuplex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundFullDuplex_Vtbl
    where
        Identity: IDirectSoundFullDuplex_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaptureguid: *const windows_core::GUID, prenderguid: *const windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut *mut core::ffi::c_void, lplpdirectsoundbuffer8: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDirectSoundFullDuplex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundFullDuplex_Impl::Initialize(this, core::mem::transmute_copy(&pcaptureguid), core::mem::transmute_copy(&prenderguid), core::mem::transmute_copy(&lpdscbufferdesc), core::mem::transmute_copy(&lpdsbufferdesc), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwlevel), core::mem::transmute_copy(&lplpdirectsoundcapturebuffer8), core::mem::transmute_copy(&lplpdirectsoundbuffer8)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundFullDuplex as windows_core::Interface>::IID
    }
}
pub trait IDirectSoundNotify_Impl: Sized {
    fn SetNotificationPositions(&self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectSoundNotify {}
impl IDirectSoundNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDirectSoundNotify_Vtbl
    where
        Identity: IDirectSoundNotify_Impl,
    {
        unsafe extern "system" fn SetNotificationPositions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> windows_core::HRESULT
        where
            Identity: IDirectSoundNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectSoundNotify_Impl::SetNotificationPositions(this, core::mem::transmute_copy(&dwpositionnotifies), core::mem::transmute_copy(&pcpositionnotifies)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetNotificationPositions: SetNotificationPositions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectSoundNotify as windows_core::Interface>::IID
    }
}
