pub trait IAudioEndpointFormatControl_Impl: Sized {
    fn ResetToDefault(&self, resetflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEndpointFormatControl {}
impl IAudioEndpointFormatControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointFormatControl_Vtbl
    where
        Identity: IAudioEndpointFormatControl_Impl,
    {
        unsafe extern "system" fn ResetToDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, resetflags: u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointFormatControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointFormatControl_Impl::ResetToDefault(this, core::mem::transmute_copy(&resetflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ResetToDefault: ResetToDefault::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointFormatControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioEndpointLastBufferControl_Impl: Sized {
    fn IsLastBufferControlSupported(&self) -> super::super::super::Foundation::BOOL;
    fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl windows_core::RuntimeName for IAudioEndpointLastBufferControl {}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioEndpointLastBufferControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointLastBufferControl_Vtbl
    where
        Identity: IAudioEndpointLastBufferControl_Impl,
    {
        unsafe extern "system" fn IsLastBufferControlSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL
        where
            Identity: IAudioEndpointLastBufferControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointLastBufferControl_Impl::IsLastBufferControlSupported(this)
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY)
        where
            Identity: IAudioEndpointLastBufferControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointLastBufferControl_Impl::ReleaseOutputDataPointerForLastBuffer(this, core::mem::transmute_copy(&pconnectionproperty))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsLastBufferControlSupported: IsLastBufferControlSupported::<Identity, OFFSET>,
            ReleaseOutputDataPointerForLastBuffer: ReleaseOutputDataPointerForLastBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointLastBufferControl as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMeter_Impl: Sized {
    fn GetMeterChannelCount(&self) -> windows_core::Result<u32>;
    fn GetMeteringData(&self, u32channelcount: u32) -> windows_core::Result<f32>;
}
impl windows_core::RuntimeName for IAudioEndpointOffloadStreamMeter {}
impl IAudioEndpointOffloadStreamMeter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamMeter_Vtbl
    where
        Identity: IAudioEndpointOffloadStreamMeter_Impl,
    {
        unsafe extern "system" fn GetMeterChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32channelcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamMeter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointOffloadStreamMeter_Impl::GetMeterChannelCount(this) {
                Ok(ok__) => {
                    pu32channelcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamMeter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointOffloadStreamMeter_Impl::GetMeteringData(this, core::mem::transmute_copy(&u32channelcount)) {
                Ok(ok__) => {
                    pf32peakvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMeterChannelCount: GetMeterChannelCount::<Identity, OFFSET>,
            GetMeteringData: GetMeteringData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMeter as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMute_Impl: Sized {
    fn SetMute(&self, bmuted: u8) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<u8>;
}
impl windows_core::RuntimeName for IAudioEndpointOffloadStreamMute {}
impl IAudioEndpointOffloadStreamMute_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamMute_Vtbl
    where
        Identity: IAudioEndpointOffloadStreamMute_Impl,
    {
        unsafe extern "system" fn SetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmuted: u8) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamMute_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointOffloadStreamMute_Impl::SetMute(this, core::mem::transmute_copy(&bmuted)).into()
        }
        unsafe extern "system" fn GetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmuted: *mut u8) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamMute_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointOffloadStreamMute_Impl::GetMute(this) {
                Ok(ok__) => {
                    pbmuted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMute: SetMute::<Identity, OFFSET>, GetMute: GetMute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub trait IAudioEndpointOffloadStreamVolume_Impl: Sized {
    fn GetVolumeChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> windows_core::Result<()>;
    fn GetChannelVolumes(&self, u32channelcount: u32) -> windows_core::Result<f32>;
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl windows_core::RuntimeName for IAudioEndpointOffloadStreamVolume {}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl IAudioEndpointOffloadStreamVolume_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamVolume_Vtbl
    where
        Identity: IAudioEndpointOffloadStreamVolume_Impl,
    {
        unsafe extern "system" fn GetVolumeChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32channelcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointOffloadStreamVolume_Impl::GetVolumeChannelCount(this) {
                Ok(ok__) => {
                    pu32channelcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointOffloadStreamVolume_Impl::SetChannelVolumes(this, core::mem::transmute_copy(&u32channelcount), core::mem::transmute_copy(&pf32volumes), core::mem::transmute_copy(&u32curvetype), core::mem::transmute_copy(&pcurveduration)).into()
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointOffloadStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointOffloadStreamVolume_Impl::GetChannelVolumes(this, core::mem::transmute_copy(&u32channelcount)) {
                Ok(ok__) => {
                    pf32volumes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVolumeChannelCount: GetVolumeChannelCount::<Identity, OFFSET>,
            SetChannelVolumes: SetChannelVolumes::<Identity, OFFSET>,
            GetChannelVolumes: GetChannelVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamVolume as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointVolume_Impl: Sized {
    fn RegisterControlChangeNotify(&self, pnotify: Option<&IAudioEndpointVolumeCallback>) -> windows_core::Result<()>;
    fn UnregisterControlChangeNotify(&self, pnotify: Option<&IAudioEndpointVolumeCallback>) -> windows_core::Result<()>;
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMasterVolumeLevel(&self) -> windows_core::Result<f32>;
    fn GetMasterVolumeLevelScalar(&self) -> windows_core::Result<f32>;
    fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelVolumeLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::Result<()>;
    fn VolumeStepUp(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn VolumeStepDown(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn QueryHardwareSupport(&self) -> windows_core::Result<u32>;
    fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEndpointVolume {}
impl IAudioEndpointVolume_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointVolume_Vtbl
    where
        Identity: IAudioEndpointVolume_Impl,
    {
        unsafe extern "system" fn RegisterControlChangeNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::RegisterControlChangeNotify(this, windows_core::from_raw_borrowed(&pnotify)).into()
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::UnregisterControlChangeNotify(this, windows_core::from_raw_borrowed(&pnotify)).into()
        }
        unsafe extern "system" fn GetChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnchannelcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::GetChannelCount(this) {
                Ok(ok__) => {
                    pnchannelcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::SetMasterVolumeLevel(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::SetMasterVolumeLevelScalar(this, core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfleveldb: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::GetMasterVolumeLevel(this) {
                Ok(ok__) => {
                    pfleveldb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::GetMasterVolumeLevelScalar(this) {
                Ok(ok__) => {
                    pflevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::SetChannelVolumeLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::SetChannelVolumeLevelScalar(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::GetChannelVolumeLevel(this, core::mem::transmute_copy(&nchannel)) {
                Ok(ok__) => {
                    pfleveldb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::GetChannelVolumeLevelScalar(this, core::mem::transmute_copy(&nchannel)) {
                Ok(ok__) => {
                    pflevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::GetMute(this) {
                Ok(ok__) => {
                    pbmute.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeStepInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::GetVolumeStepInfo(this, core::mem::transmute_copy(&pnstep), core::mem::transmute_copy(&pnstepcount)).into()
        }
        unsafe extern "system" fn VolumeStepUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::VolumeStepUp(this, core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn VolumeStepDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::VolumeStepDown(this, core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioEndpointVolume_Impl::QueryHardwareSupport(this) {
                Ok(ok__) => {
                    pdwhardwaresupportmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolume_Impl::GetVolumeRange(this, core::mem::transmute_copy(&pflvolumemindb), core::mem::transmute_copy(&pflvolumemaxdb), core::mem::transmute_copy(&pflvolumeincrementdb)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterControlChangeNotify: RegisterControlChangeNotify::<Identity, OFFSET>,
            UnregisterControlChangeNotify: UnregisterControlChangeNotify::<Identity, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetMasterVolumeLevel: SetMasterVolumeLevel::<Identity, OFFSET>,
            SetMasterVolumeLevelScalar: SetMasterVolumeLevelScalar::<Identity, OFFSET>,
            GetMasterVolumeLevel: GetMasterVolumeLevel::<Identity, OFFSET>,
            GetMasterVolumeLevelScalar: GetMasterVolumeLevelScalar::<Identity, OFFSET>,
            SetChannelVolumeLevel: SetChannelVolumeLevel::<Identity, OFFSET>,
            SetChannelVolumeLevelScalar: SetChannelVolumeLevelScalar::<Identity, OFFSET>,
            GetChannelVolumeLevel: GetChannelVolumeLevel::<Identity, OFFSET>,
            GetChannelVolumeLevelScalar: GetChannelVolumeLevelScalar::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
            GetVolumeStepInfo: GetVolumeStepInfo::<Identity, OFFSET>,
            VolumeStepUp: VolumeStepUp::<Identity, OFFSET>,
            VolumeStepDown: VolumeStepDown::<Identity, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, OFFSET>,
            GetVolumeRange: GetVolumeRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolume as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointVolumeCallback_Impl: Sized {
    fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEndpointVolumeCallback {}
impl IAudioEndpointVolumeCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointVolumeCallback_Vtbl
    where
        Identity: IAudioEndpointVolumeCallback_Impl,
    {
        unsafe extern "system" fn OnNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolumeCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolumeCallback_Impl::OnNotify(this, core::mem::transmute_copy(&pnotify)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeCallback as windows_core::Interface>::IID
    }
}
pub trait IAudioEndpointVolumeEx_Impl: Sized + IAudioEndpointVolume_Impl {
    fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEndpointVolumeEx {}
impl IAudioEndpointVolumeEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEndpointVolumeEx_Vtbl
    where
        Identity: IAudioEndpointVolumeEx_Impl,
    {
        unsafe extern "system" fn GetVolumeRangeChannel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioEndpointVolumeEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEndpointVolumeEx_Impl::GetVolumeRangeChannel(this, core::mem::transmute_copy(&ichannel), core::mem::transmute_copy(&pflvolumemindb), core::mem::transmute_copy(&pflvolumemaxdb), core::mem::transmute_copy(&pflvolumeincrementdb)).into()
        }
        Self { base__: IAudioEndpointVolume_Vtbl::new::<Identity, OFFSET>(), GetVolumeRangeChannel: GetVolumeRangeChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeEx as windows_core::Interface>::IID || iid == &<IAudioEndpointVolume as windows_core::Interface>::IID
    }
}
pub trait IAudioLfxControl_Impl: Sized {
    fn SetLocalEffectsState(&self, benabled: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetLocalEffectsState(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAudioLfxControl {}
impl IAudioLfxControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioLfxControl_Vtbl
    where
        Identity: IAudioLfxControl_Impl,
    {
        unsafe extern "system" fn SetLocalEffectsState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioLfxControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioLfxControl_Impl::SetLocalEffectsState(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn GetLocalEffectsState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioLfxControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioLfxControl_Impl::GetLocalEffectsState(this) {
                Ok(ok__) => {
                    pbenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLocalEffectsState: SetLocalEffectsState::<Identity, OFFSET>,
            GetLocalEffectsState: GetLocalEffectsState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioLfxControl as windows_core::Interface>::IID
    }
}
pub trait IAudioMeterInformation_Impl: Sized {
    fn GetPeakValue(&self) -> windows_core::Result<f32>;
    fn GetMeteringChannelCount(&self) -> windows_core::Result<u32>;
    fn GetChannelsPeakValues(&self, u32channelcount: u32, afpeakvalues: *mut f32) -> windows_core::Result<()>;
    fn QueryHardwareSupport(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioMeterInformation {}
impl IAudioMeterInformation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioMeterInformation_Vtbl
    where
        Identity: IAudioMeterInformation_Impl,
    {
        unsafe extern "system" fn GetPeakValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpeak: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioMeterInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioMeterInformation_Impl::GetPeakValue(this) {
                Ok(ok__) => {
                    pfpeak.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnchannelcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioMeterInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioMeterInformation_Impl::GetMeteringChannelCount(this) {
                Ok(ok__) => {
                    pnchannelcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelsPeakValues<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioMeterInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioMeterInformation_Impl::GetChannelsPeakValues(this, core::mem::transmute_copy(&u32channelcount), core::mem::transmute_copy(&afpeakvalues)).into()
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioMeterInformation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioMeterInformation_Impl::QueryHardwareSupport(this) {
                Ok(ok__) => {
                    pdwhardwaresupportmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeakValue: GetPeakValue::<Identity, OFFSET>,
            GetMeteringChannelCount: GetMeteringChannelCount::<Identity, OFFSET>,
            GetChannelsPeakValues: GetChannelsPeakValues::<Identity, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMeterInformation as windows_core::Interface>::IID
    }
}
pub trait IHardwareAudioEngineBase_Impl: Sized {
    fn GetAvailableOffloadConnectorCount(&self, _pwstrdeviceid: &windows_core::PCWSTR, _uconnectorid: u32) -> windows_core::Result<u32>;
    fn GetEngineFormat(&self, pdevice: Option<&super::IMMDevice>, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetEngineDeviceFormat(&self, pdevice: Option<&super::IMMDevice>, _pwfxformat: *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn SetGfxState(&self, pdevice: Option<&super::IMMDevice>, _benable: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetGfxState(&self, pdevice: Option<&super::IMMDevice>) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IHardwareAudioEngineBase {}
impl IHardwareAudioEngineBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IHardwareAudioEngineBase_Vtbl
    where
        Identity: IHardwareAudioEngineBase_Impl,
    {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, _pwstrdeviceid: windows_core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IHardwareAudioEngineBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHardwareAudioEngineBase_Impl::GetAvailableOffloadConnectorCount(this, core::mem::transmute(&_pwstrdeviceid), core::mem::transmute_copy(&_uconnectorid)) {
                Ok(ok__) => {
                    _pavailableconnectorinstancecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEngineFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IHardwareAudioEngineBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHardwareAudioEngineBase_Impl::GetEngineFormat(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&_brequestdeviceformat), core::mem::transmute_copy(&_ppwfxformat)).into()
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _pwfxformat: *mut super::WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IHardwareAudioEngineBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHardwareAudioEngineBase_Impl::SetEngineDeviceFormat(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&_pwfxformat)).into()
        }
        unsafe extern "system" fn SetGfxState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _benable: super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHardwareAudioEngineBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IHardwareAudioEngineBase_Impl::SetGfxState(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&_benable)).into()
        }
        unsafe extern "system" fn GetGfxState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, _pbenable: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IHardwareAudioEngineBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IHardwareAudioEngineBase_Impl::GetGfxState(this, windows_core::from_raw_borrowed(&pdevice)) {
                Ok(ok__) => {
                    _pbenable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableOffloadConnectorCount: GetAvailableOffloadConnectorCount::<Identity, OFFSET>,
            GetEngineFormat: GetEngineFormat::<Identity, OFFSET>,
            SetEngineDeviceFormat: SetEngineDeviceFormat::<Identity, OFFSET>,
            SetGfxState: SetGfxState::<Identity, OFFSET>,
            GetGfxState: GetGfxState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHardwareAudioEngineBase as windows_core::Interface>::IID
    }
}
