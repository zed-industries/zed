pub trait IAcousticEchoCancellationControl_Impl: Sized {
    fn SetEchoCancellationRenderEndpoint(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAcousticEchoCancellationControl {}
impl IAcousticEchoCancellationControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAcousticEchoCancellationControl_Vtbl
    where
        Identity: IAcousticEchoCancellationControl_Impl,
    {
        unsafe extern "system" fn SetEchoCancellationRenderEndpoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAcousticEchoCancellationControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAcousticEchoCancellationControl_Impl::SetEchoCancellationRenderEndpoint(this, core::mem::transmute(&endpointid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetEchoCancellationRenderEndpoint: SetEchoCancellationRenderEndpoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAcousticEchoCancellationControl as windows_core::Interface>::IID
    }
}
pub trait IActivateAudioInterfaceAsyncOperation_Impl: Sized {
    fn GetActivateResult(&self, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IActivateAudioInterfaceAsyncOperation {}
impl IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActivateAudioInterfaceAsyncOperation_Vtbl
    where
        Identity: IActivateAudioInterfaceAsyncOperation_Impl,
    {
        unsafe extern "system" fn GetActivateResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActivateAudioInterfaceAsyncOperation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActivateAudioInterfaceAsyncOperation_Impl::GetActivateResult(this, core::mem::transmute_copy(&activateresult), core::mem::transmute_copy(&activatedinterface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivateResult: GetActivateResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceAsyncOperation as windows_core::Interface>::IID
    }
}
pub trait IActivateAudioInterfaceCompletionHandler_Impl: Sized {
    fn ActivateCompleted(&self, activateoperation: Option<&IActivateAudioInterfaceAsyncOperation>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IActivateAudioInterfaceCompletionHandler {}
impl IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IActivateAudioInterfaceCompletionHandler_Vtbl
    where
        Identity: IActivateAudioInterfaceCompletionHandler_Impl,
    {
        unsafe extern "system" fn ActivateCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activateoperation: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IActivateAudioInterfaceCompletionHandler_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IActivateAudioInterfaceCompletionHandler_Impl::ActivateCompleted(this, windows_core::from_raw_borrowed(&activateoperation)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ActivateCompleted: ActivateCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceCompletionHandler as windows_core::Interface>::IID
    }
}
pub trait IAudioAmbisonicsControl_Impl: Sized {
    fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::Result<()>;
    fn SetHeadTracking(&self, benableheadtracking: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetHeadTracking(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioAmbisonicsControl {}
impl IAudioAmbisonicsControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioAmbisonicsControl_Vtbl
    where
        Identity: IAudioAmbisonicsControl_Impl,
    {
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::HRESULT
        where
            Identity: IAudioAmbisonicsControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioAmbisonicsControl_Impl::SetData(this, core::mem::transmute_copy(&pambisonicsparams), core::mem::transmute_copy(&cbambisonicsparams)).into()
        }
        unsafe extern "system" fn SetHeadTracking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioAmbisonicsControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioAmbisonicsControl_Impl::SetHeadTracking(this, core::mem::transmute_copy(&benableheadtracking)).into()
        }
        unsafe extern "system" fn GetHeadTracking<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioAmbisonicsControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioAmbisonicsControl_Impl::GetHeadTracking(this) {
                Ok(ok__) => {
                    pbenableheadtracking.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> windows_core::HRESULT
        where
            Identity: IAudioAmbisonicsControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioAmbisonicsControl_Impl::SetRotation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&w)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetData: SetData::<Identity, OFFSET>,
            SetHeadTracking: SetHeadTracking::<Identity, OFFSET>,
            GetHeadTracking: GetHeadTracking::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioAmbisonicsControl as windows_core::Interface>::IID
    }
}
pub trait IAudioAutoGainControl_Impl: Sized {
    fn GetEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, benable: super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioAutoGainControl {}
impl IAudioAutoGainControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioAutoGainControl_Vtbl
    where
        Identity: IAudioAutoGainControl_Impl,
    {
        unsafe extern "system" fn GetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioAutoGainControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioAutoGainControl_Impl::GetEnabled(this) {
                Ok(ok__) => {
                    pbenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioAutoGainControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioAutoGainControl_Impl::SetEnabled(this, core::mem::transmute_copy(&benable), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnabled: GetEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioAutoGainControl as windows_core::Interface>::IID
    }
}
pub trait IAudioBass_Impl: Sized + IPerChannelDbLevel_Impl {}
impl windows_core::RuntimeName for IAudioBass {}
impl IAudioBass_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioBass_Vtbl
    where
        Identity: IAudioBass_Impl,
    {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioBass as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
pub trait IAudioCaptureClient_Impl: Sized {
    fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> windows_core::Result<()>;
    fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::Result<()>;
    fn GetNextPacketSize(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioCaptureClient {}
impl IAudioCaptureClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioCaptureClient_Vtbl
    where
        Identity: IAudioCaptureClient_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAudioCaptureClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioCaptureClient_Impl::GetBuffer(this, core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pnumframestoread), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pu64deviceposition), core::mem::transmute_copy(&pu64qpcposition)).into()
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframesread: u32) -> windows_core::HRESULT
        where
            Identity: IAudioCaptureClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioCaptureClient_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&numframesread)).into()
        }
        unsafe extern "system" fn GetNextPacketSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioCaptureClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioCaptureClient_Impl::GetNextPacketSize(this) {
                Ok(ok__) => {
                    pnumframesinnextpacket.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, OFFSET>,
            GetNextPacketSize: GetNextPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioCaptureClient as windows_core::Interface>::IID
    }
}
pub trait IAudioChannelConfig_Impl: Sized {
    fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelConfig(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioChannelConfig {}
impl IAudioChannelConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioChannelConfig_Vtbl
    where
        Identity: IAudioChannelConfig_Impl,
    {
        unsafe extern "system" fn SetChannelConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfig: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioChannelConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioChannelConfig_Impl::SetChannelConfig(this, core::mem::transmute_copy(&dwconfig), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetChannelConfig<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconfig: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioChannelConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioChannelConfig_Impl::GetChannelConfig(this) {
                Ok(ok__) => {
                    pdwconfig.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetChannelConfig: SetChannelConfig::<Identity, OFFSET>,
            GetChannelConfig: GetChannelConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioChannelConfig as windows_core::Interface>::IID
    }
}
pub trait IAudioClient_Impl: Sized {
    fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetBufferSize(&self) -> windows_core::Result<u32>;
    fn GetStreamLatency(&self) -> windows_core::Result<i64>;
    fn GetCurrentPadding(&self) -> windows_core::Result<u32>;
    fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT;
    fn GetMixFormat(&self) -> windows_core::Result<*mut WAVEFORMATEX>;
    fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetEventHandle(&self, eventhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn GetService(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioClient {}
impl IAudioClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClient_Vtbl
    where
        Identity: IAudioClient_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::Initialize(this, core::mem::transmute_copy(&sharemode), core::mem::transmute_copy(&streamflags), core::mem::transmute_copy(&hnsbufferduration), core::mem::transmute_copy(&hnsperiodicity), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&audiosessionguid)).into()
        }
        unsafe extern "system" fn GetBufferSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumbufferframes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClient_Impl::GetBufferSize(this) {
                Ok(ok__) => {
                    pnumbufferframes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamLatency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnslatency: *mut i64) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClient_Impl::GetStreamLatency(this) {
                Ok(ok__) => {
                    phnslatency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPadding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpaddingframes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClient_Impl::GetCurrentPadding(this) {
                Ok(ok__) => {
                    pnumpaddingframes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFormatSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::IsFormatSupported(this, core::mem::transmute_copy(&sharemode), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&ppclosestmatch))
        }
        unsafe extern "system" fn GetMixFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClient_Impl::GetMixFormat(this) {
                Ok(ok__) => {
                    ppdeviceformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::GetDevicePeriod(this, core::mem::transmute_copy(&phnsdefaultdeviceperiod), core::mem::transmute_copy(&phnsminimumdeviceperiod)).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::Reset(this).into()
        }
        unsafe extern "system" fn SetEventHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::SetEventHandle(this, core::mem::transmute_copy(&eventhandle)).into()
        }
        unsafe extern "system" fn GetService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient_Impl::GetService(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, OFFSET>,
            GetStreamLatency: GetStreamLatency::<Identity, OFFSET>,
            GetCurrentPadding: GetCurrentPadding::<Identity, OFFSET>,
            IsFormatSupported: IsFormatSupported::<Identity, OFFSET>,
            GetMixFormat: GetMixFormat::<Identity, OFFSET>,
            GetDevicePeriod: GetDevicePeriod::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClient as windows_core::Interface>::IID
    }
}
pub trait IAudioClient2_Impl: Sized + IAudioClient_Impl {
    fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> windows_core::Result<()>;
    fn GetBufferSizeLimits(&self, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioClient2 {}
impl IAudioClient2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClient2_Vtbl
    where
        Identity: IAudioClient2_Impl,
    {
        unsafe extern "system" fn IsOffloadCapable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClient2_Impl::IsOffloadCapable(this, core::mem::transmute_copy(&category)) {
                Ok(ok__) => {
                    pboffloadcapable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperties: *const AudioClientProperties) -> windows_core::HRESULT
        where
            Identity: IAudioClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient2_Impl::SetClientProperties(this, core::mem::transmute_copy(&pproperties)).into()
        }
        unsafe extern "system" fn GetBufferSizeLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> windows_core::HRESULT
        where
            Identity: IAudioClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient2_Impl::GetBufferSizeLimits(this, core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&beventdriven), core::mem::transmute_copy(&phnsminbufferduration), core::mem::transmute_copy(&phnsmaxbufferduration)).into()
        }
        Self {
            base__: IAudioClient_Vtbl::new::<Identity, OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Identity, OFFSET>,
            SetClientProperties: SetClientProperties::<Identity, OFFSET>,
            GetBufferSizeLimits: GetBufferSizeLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClient2 as windows_core::Interface>::IID || iid == &<IAudioClient as windows_core::Interface>::IID
    }
}
pub trait IAudioClient3_Impl: Sized + IAudioClient2_Impl {
    fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::Result<()>;
    fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioClient3 {}
impl IAudioClient3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClient3_Vtbl
    where
        Identity: IAudioClient3_Impl,
    {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioClient3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient3_Impl::GetSharedModeEnginePeriod(this, core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pdefaultperiodinframes), core::mem::transmute_copy(&pfundamentalperiodinframes), core::mem::transmute_copy(&pminperiodinframes), core::mem::transmute_copy(&pmaxperiodinframes)).into()
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioClient3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient3_Impl::GetCurrentSharedModeEnginePeriod(this, core::mem::transmute_copy(&ppformat), core::mem::transmute_copy(&pcurrentperiodinframes)).into()
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioClient3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClient3_Impl::InitializeSharedAudioStream(this, core::mem::transmute_copy(&streamflags), core::mem::transmute_copy(&periodinframes), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&audiosessionguid)).into()
        }
        Self {
            base__: IAudioClient2_Vtbl::new::<Identity, OFFSET>(),
            GetSharedModeEnginePeriod: GetSharedModeEnginePeriod::<Identity, OFFSET>,
            GetCurrentSharedModeEnginePeriod: GetCurrentSharedModeEnginePeriod::<Identity, OFFSET>,
            InitializeSharedAudioStream: InitializeSharedAudioStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClient3 as windows_core::Interface>::IID || iid == &<IAudioClient as windows_core::Interface>::IID || iid == &<IAudioClient2 as windows_core::Interface>::IID
    }
}
pub trait IAudioClientDuckingControl_Impl: Sized {
    fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioClientDuckingControl {}
impl IAudioClientDuckingControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClientDuckingControl_Vtbl
    where
        Identity: IAudioClientDuckingControl_Impl,
    {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT
        where
            Identity: IAudioClientDuckingControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClientDuckingControl_Impl::SetDuckingOptionsForCurrentStream(this, core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDuckingOptionsForCurrentStream: SetDuckingOptionsForCurrentStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClientDuckingControl as windows_core::Interface>::IID
    }
}
pub trait IAudioClock_Impl: Sized {
    fn GetFrequency(&self) -> windows_core::Result<u64>;
    fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> windows_core::Result<()>;
    fn GetCharacteristics(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IAudioClock {}
impl IAudioClock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClock_Vtbl
    where
        Identity: IAudioClock_Impl,
    {
        unsafe extern "system" fn GetFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu64frequency: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAudioClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClock_Impl::GetFrequency(this) {
                Ok(ok__) => {
                    pu64frequency.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAudioClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClock_Impl::GetPosition(this, core::mem::transmute_copy(&pu64position), core::mem::transmute_copy(&pu64qpcposition)).into()
        }
        unsafe extern "system" fn GetCharacteristics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioClock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioClock_Impl::GetCharacteristics(this) {
                Ok(ok__) => {
                    pdwcharacteristics.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrequency: GetFrequency::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClock as windows_core::Interface>::IID
    }
}
pub trait IAudioClock2_Impl: Sized {
    fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioClock2 {}
impl IAudioClock2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClock2_Vtbl
    where
        Identity: IAudioClock2_Impl,
    {
        unsafe extern "system" fn GetDevicePosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> windows_core::HRESULT
        where
            Identity: IAudioClock2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClock2_Impl::GetDevicePosition(this, core::mem::transmute_copy(&deviceposition), core::mem::transmute_copy(&qpcposition)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDevicePosition: GetDevicePosition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClock2 as windows_core::Interface>::IID
    }
}
pub trait IAudioClockAdjustment_Impl: Sized {
    fn SetSampleRate(&self, flsamplerate: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioClockAdjustment {}
impl IAudioClockAdjustment_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioClockAdjustment_Vtbl
    where
        Identity: IAudioClockAdjustment_Impl,
    {
        unsafe extern "system" fn SetSampleRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flsamplerate: f32) -> windows_core::HRESULT
        where
            Identity: IAudioClockAdjustment_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioClockAdjustment_Impl::SetSampleRate(this, core::mem::transmute_copy(&flsamplerate)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSampleRate: SetSampleRate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClockAdjustment as windows_core::Interface>::IID
    }
}
pub trait IAudioEffectsChangedNotificationClient_Impl: Sized {
    fn OnAudioEffectsChanged(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEffectsChangedNotificationClient {}
impl IAudioEffectsChangedNotificationClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEffectsChangedNotificationClient_Vtbl
    where
        Identity: IAudioEffectsChangedNotificationClient_Impl,
    {
        unsafe extern "system" fn OnAudioEffectsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEffectsChangedNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEffectsChangedNotificationClient_Impl::OnAudioEffectsChanged(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAudioEffectsChanged: OnAudioEffectsChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEffectsChangedNotificationClient as windows_core::Interface>::IID
    }
}
pub trait IAudioEffectsManager_Impl: Sized {
    fn RegisterAudioEffectsChangedNotificationCallback(&self, client: Option<&IAudioEffectsChangedNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterAudioEffectsChangedNotificationCallback(&self, client: Option<&IAudioEffectsChangedNotificationClient>) -> windows_core::Result<()>;
    fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::Result<()>;
    fn SetAudioEffectState(&self, effectid: &windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioEffectsManager {}
impl IAudioEffectsManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioEffectsManager_Vtbl
    where
        Identity: IAudioEffectsManager_Impl,
    {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, client: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEffectsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEffectsManager_Impl::RegisterAudioEffectsChangedNotificationCallback(this, windows_core::from_raw_borrowed(&client)).into()
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, client: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioEffectsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEffectsManager_Impl::UnregisterAudioEffectsChangedNotificationCallback(this, windows_core::from_raw_borrowed(&client)).into()
        }
        unsafe extern "system" fn GetAudioEffects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioEffectsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEffectsManager_Impl::GetAudioEffects(this, core::mem::transmute_copy(&effects), core::mem::transmute_copy(&numeffects)).into()
        }
        unsafe extern "system" fn SetAudioEffectState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::HRESULT
        where
            Identity: IAudioEffectsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioEffectsManager_Impl::SetAudioEffectState(this, core::mem::transmute(&effectid), core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterAudioEffectsChangedNotificationCallback: RegisterAudioEffectsChangedNotificationCallback::<Identity, OFFSET>,
            UnregisterAudioEffectsChangedNotificationCallback: UnregisterAudioEffectsChangedNotificationCallback::<Identity, OFFSET>,
            GetAudioEffects: GetAudioEffects::<Identity, OFFSET>,
            SetAudioEffectState: SetAudioEffectState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEffectsManager as windows_core::Interface>::IID
    }
}
pub trait IAudioFormatEnumerator_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetFormat(&self, index: u32) -> windows_core::Result<*mut WAVEFORMATEX>;
}
impl windows_core::RuntimeName for IAudioFormatEnumerator {}
impl IAudioFormatEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioFormatEnumerator_Vtbl
    where
        Identity: IAudioFormatEnumerator_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioFormatEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioFormatEnumerator_Impl::GetCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: IAudioFormatEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioFormatEnumerator_Impl::GetFormat(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetFormat: GetFormat::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFormatEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAudioInputSelector_Impl: Sized {
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioInputSelector {}
impl IAudioInputSelector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioInputSelector_Vtbl
    where
        Identity: IAudioInputSelector_Impl,
    {
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioInputSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioInputSelector_Impl::GetSelection(this) {
                Ok(ok__) => {
                    pnidselected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioInputSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioInputSelector_Impl::SetSelection(this, core::mem::transmute_copy(&nidselect), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioInputSelector as windows_core::Interface>::IID
    }
}
pub trait IAudioLoudness_Impl: Sized {
    fn GetEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, benable: super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioLoudness {}
impl IAudioLoudness_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioLoudness_Vtbl
    where
        Identity: IAudioLoudness_Impl,
    {
        unsafe extern "system" fn GetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioLoudness_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioLoudness_Impl::GetEnabled(this) {
                Ok(ok__) => {
                    pbenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioLoudness_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioLoudness_Impl::SetEnabled(this, core::mem::transmute_copy(&benable), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnabled: GetEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioLoudness as windows_core::Interface>::IID
    }
}
pub trait IAudioMidrange_Impl: Sized + IPerChannelDbLevel_Impl {}
impl windows_core::RuntimeName for IAudioMidrange {}
impl IAudioMidrange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioMidrange_Vtbl
    where
        Identity: IAudioMidrange_Impl,
    {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMidrange as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
pub trait IAudioMute_Impl: Sized {
    fn SetMute(&self, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IAudioMute {}
impl IAudioMute_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioMute_Vtbl
    where
        Identity: IAudioMute_Impl,
    {
        unsafe extern "system" fn SetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioMute_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioMute_Impl::SetMute(this, core::mem::transmute_copy(&bmuted), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioMute_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioMute_Impl::GetMute(this) {
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
        iid == &<IAudioMute as windows_core::Interface>::IID
    }
}
pub trait IAudioOutputSelector_Impl: Sized {
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioOutputSelector {}
impl IAudioOutputSelector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioOutputSelector_Vtbl
    where
        Identity: IAudioOutputSelector_Impl,
    {
        unsafe extern "system" fn GetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioOutputSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioOutputSelector_Impl::GetSelection(this) {
                Ok(ok__) => {
                    pnidselected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioOutputSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioOutputSelector_Impl::SetSelection(this, core::mem::transmute_copy(&nidselect), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioOutputSelector as windows_core::Interface>::IID
    }
}
pub trait IAudioPeakMeter_Impl: Sized {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
}
impl windows_core::RuntimeName for IAudioPeakMeter {}
impl IAudioPeakMeter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioPeakMeter_Vtbl
    where
        Identity: IAudioPeakMeter_Impl,
    {
        unsafe extern "system" fn GetChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioPeakMeter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioPeakMeter_Impl::GetChannelCount(this) {
                Ok(ok__) => {
                    pcchannels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioPeakMeter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioPeakMeter_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel)) {
                Ok(ok__) => {
                    pflevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            GetLevel: GetLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioPeakMeter as windows_core::Interface>::IID
    }
}
pub trait IAudioRenderClient_Impl: Sized {
    fn GetBuffer(&self, numframesrequested: u32) -> windows_core::Result<*mut u8>;
    fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioRenderClient {}
impl IAudioRenderClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioRenderClient_Vtbl
    where
        Identity: IAudioRenderClient_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IAudioRenderClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioRenderClient_Impl::GetBuffer(this, core::mem::transmute_copy(&numframesrequested)) {
                Ok(ok__) => {
                    ppdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IAudioRenderClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioRenderClient_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&numframeswritten), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioRenderClient as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionControl_Impl: Sized {
    fn GetState(&self) -> windows_core::Result<AudioSessionState>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDisplayName(&self, value: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIconPath(&self, value: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGroupingParam(&self, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterAudioSessionNotification(&self, newnotifications: Option<&IAudioSessionEvents>) -> windows_core::Result<()>;
    fn UnregisterAudioSessionNotification(&self, newnotifications: Option<&IAudioSessionEvents>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSessionControl {}
impl IAudioSessionControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionControl_Vtbl
    where
        Identity: IAudioSessionControl_Impl,
    {
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut AudioSessionState) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl_Impl::GetState(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl_Impl::GetDisplayName(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl_Impl::SetDisplayName(this, core::mem::transmute(&value), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetIconPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl_Impl::GetIconPath(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl_Impl::SetIconPath(this, core::mem::transmute(&value), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetGroupingParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl_Impl::GetGroupingParam(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupingParam<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl_Impl::SetGroupingParam(this, core::mem::transmute_copy(&r#override), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newnotifications: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl_Impl::RegisterAudioSessionNotification(this, windows_core::from_raw_borrowed(&newnotifications)).into()
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newnotifications: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl_Impl::UnregisterAudioSessionNotification(this, windows_core::from_raw_borrowed(&newnotifications)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetIconPath: GetIconPath::<Identity, OFFSET>,
            SetIconPath: SetIconPath::<Identity, OFFSET>,
            GetGroupingParam: GetGroupingParam::<Identity, OFFSET>,
            SetGroupingParam: SetGroupingParam::<Identity, OFFSET>,
            RegisterAudioSessionNotification: RegisterAudioSessionNotification::<Identity, OFFSET>,
            UnregisterAudioSessionNotification: UnregisterAudioSessionNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionControl as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionControl2_Impl: Sized + IAudioSessionControl_Impl {
    fn GetSessionIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSessionInstanceIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProcessId(&self) -> windows_core::Result<u32>;
    fn IsSystemSoundsSession(&self) -> windows_core::HRESULT;
    fn SetDuckingPreference(&self, optout: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSessionControl2 {}
impl IAudioSessionControl2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionControl2_Vtbl
    where
        Identity: IAudioSessionControl2_Impl,
    {
        unsafe extern "system" fn GetSessionIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl2_Impl::GetSessionIdentifier(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl2_Impl::GetSessionInstanceIdentifier(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionControl2_Impl::GetProcessId(this) {
                Ok(ok__) => {
                    pretval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSystemSoundsSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl2_Impl::IsSystemSoundsSession(this)
        }
        unsafe extern "system" fn SetDuckingPreference<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, optout: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IAudioSessionControl2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionControl2_Impl::SetDuckingPreference(this, core::mem::transmute_copy(&optout)).into()
        }
        Self {
            base__: IAudioSessionControl_Vtbl::new::<Identity, OFFSET>(),
            GetSessionIdentifier: GetSessionIdentifier::<Identity, OFFSET>,
            GetSessionInstanceIdentifier: GetSessionInstanceIdentifier::<Identity, OFFSET>,
            GetProcessId: GetProcessId::<Identity, OFFSET>,
            IsSystemSoundsSession: IsSystemSoundsSession::<Identity, OFFSET>,
            SetDuckingPreference: SetDuckingPreference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionControl2 as windows_core::Interface>::IID || iid == &<IAudioSessionControl as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionEnumerator_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetSession(&self, sessioncount: i32) -> windows_core::Result<IAudioSessionControl>;
}
impl windows_core::RuntimeName for IAudioSessionEnumerator {}
impl IAudioSessionEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionEnumerator_Vtbl
    where
        Identity: IAudioSessionEnumerator_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessioncount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionEnumerator_Impl::GetCount(this) {
                Ok(ok__) => {
                    sessioncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessioncount: i32, session: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionEnumerator_Impl::GetSession(this, core::mem::transmute_copy(&sessioncount)) {
                Ok(ok__) => {
                    session.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetSession: GetSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionEnumerator as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionEvents_Impl: Sized {
    fn OnDisplayNameChanged(&self, newdisplayname: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnIconPathChanged(&self, newiconpath: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnSimpleVolumeChanged(&self, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnChannelVolumeChanged(&self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnGroupingParamChanged(&self, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnStateChanged(&self, newstate: AudioSessionState) -> windows_core::Result<()>;
    fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSessionEvents {}
impl IAudioSessionEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionEvents_Vtbl
    where
        Identity: IAudioSessionEvents_Impl,
    {
        unsafe extern "system" fn OnDisplayNameChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newdisplayname: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnDisplayNameChanged(this, core::mem::transmute(&newdisplayname), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnIconPathChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newiconpath: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnIconPathChanged(this, core::mem::transmute(&newiconpath), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnSimpleVolumeChanged(this, core::mem::transmute_copy(&newvolume), core::mem::transmute_copy(&newmute), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnChannelVolumeChanged(this, core::mem::transmute_copy(&channelcount), core::mem::transmute_copy(&newchannelvolumearray), core::mem::transmute_copy(&changedchannel), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnGroupingParamChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnGroupingParamChanged(this, core::mem::transmute_copy(&newgroupingparam), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnStateChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: AudioSessionState) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnStateChanged(this, core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn OnSessionDisconnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> windows_core::HRESULT
        where
            Identity: IAudioSessionEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionEvents_Impl::OnSessionDisconnected(this, core::mem::transmute_copy(&disconnectreason)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDisplayNameChanged: OnDisplayNameChanged::<Identity, OFFSET>,
            OnIconPathChanged: OnIconPathChanged::<Identity, OFFSET>,
            OnSimpleVolumeChanged: OnSimpleVolumeChanged::<Identity, OFFSET>,
            OnChannelVolumeChanged: OnChannelVolumeChanged::<Identity, OFFSET>,
            OnGroupingParamChanged: OnGroupingParamChanged::<Identity, OFFSET>,
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnSessionDisconnected: OnSessionDisconnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionEvents as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionManager_Impl: Sized {
    fn GetAudioSessionControl(&self, audiosessionguid: *const windows_core::GUID, streamflags: u32) -> windows_core::Result<IAudioSessionControl>;
    fn GetSimpleAudioVolume(&self, audiosessionguid: *const windows_core::GUID, streamflags: u32) -> windows_core::Result<ISimpleAudioVolume>;
}
impl windows_core::RuntimeName for IAudioSessionManager {}
impl IAudioSessionManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionManager_Vtbl
    where
        Identity: IAudioSessionManager_Impl,
    {
        unsafe extern "system" fn GetAudioSessionControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *const windows_core::GUID, streamflags: u32, sessioncontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionManager_Impl::GetAudioSessionControl(this, core::mem::transmute_copy(&audiosessionguid), core::mem::transmute_copy(&streamflags)) {
                Ok(ok__) => {
                    sessioncontrol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *const windows_core::GUID, streamflags: u32, audiovolume: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionManager_Impl::GetSimpleAudioVolume(this, core::mem::transmute_copy(&audiosessionguid), core::mem::transmute_copy(&streamflags)) {
                Ok(ok__) => {
                    audiovolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAudioSessionControl: GetAudioSessionControl::<Identity, OFFSET>,
            GetSimpleAudioVolume: GetSimpleAudioVolume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionManager as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionManager2_Impl: Sized + IAudioSessionManager_Impl {
    fn GetSessionEnumerator(&self) -> windows_core::Result<IAudioSessionEnumerator>;
    fn RegisterSessionNotification(&self, sessionnotification: Option<&IAudioSessionNotification>) -> windows_core::Result<()>;
    fn UnregisterSessionNotification(&self, sessionnotification: Option<&IAudioSessionNotification>) -> windows_core::Result<()>;
    fn RegisterDuckNotification(&self, sessionid: &windows_core::PCWSTR, ducknotification: Option<&IAudioVolumeDuckNotification>) -> windows_core::Result<()>;
    fn UnregisterDuckNotification(&self, ducknotification: Option<&IAudioVolumeDuckNotification>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSessionManager2 {}
impl IAudioSessionManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionManager2_Vtbl
    where
        Identity: IAudioSessionManager2_Impl,
    {
        unsafe extern "system" fn GetSessionEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSessionManager2_Impl::GetSessionEnumerator(this) {
                Ok(ok__) => {
                    sessionenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterSessionNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionnotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionManager2_Impl::RegisterSessionNotification(this, windows_core::from_raw_borrowed(&sessionnotification)).into()
        }
        unsafe extern "system" fn UnregisterSessionNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionnotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionManager2_Impl::UnregisterSessionNotification(this, windows_core::from_raw_borrowed(&sessionnotification)).into()
        }
        unsafe extern "system" fn RegisterDuckNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, ducknotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionManager2_Impl::RegisterDuckNotification(this, core::mem::transmute(&sessionid), windows_core::from_raw_borrowed(&ducknotification)).into()
        }
        unsafe extern "system" fn UnregisterDuckNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ducknotification: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionManager2_Impl::UnregisterDuckNotification(this, windows_core::from_raw_borrowed(&ducknotification)).into()
        }
        Self {
            base__: IAudioSessionManager_Vtbl::new::<Identity, OFFSET>(),
            GetSessionEnumerator: GetSessionEnumerator::<Identity, OFFSET>,
            RegisterSessionNotification: RegisterSessionNotification::<Identity, OFFSET>,
            UnregisterSessionNotification: UnregisterSessionNotification::<Identity, OFFSET>,
            RegisterDuckNotification: RegisterDuckNotification::<Identity, OFFSET>,
            UnregisterDuckNotification: UnregisterDuckNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionManager2 as windows_core::Interface>::IID || iid == &<IAudioSessionManager as windows_core::Interface>::IID
    }
}
pub trait IAudioSessionNotification_Impl: Sized {
    fn OnSessionCreated(&self, newsession: Option<&IAudioSessionControl>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioSessionNotification {}
impl IAudioSessionNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSessionNotification_Vtbl
    where
        Identity: IAudioSessionNotification_Impl,
    {
        unsafe extern "system" fn OnSessionCreated<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newsession: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSessionNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSessionNotification_Impl::OnSessionCreated(this, windows_core::from_raw_borrowed(&newsession)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSessionCreated: OnSessionCreated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionNotification as windows_core::Interface>::IID
    }
}
pub trait IAudioStateMonitor_Impl: Sized {
    fn RegisterCallback(&self, callback: PAudioStateMonitorCallback, context: *const core::ffi::c_void) -> windows_core::Result<i64>;
    fn UnregisterCallback(&self, registration: i64);
    fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel;
}
impl windows_core::RuntimeName for IAudioStateMonitor {}
impl IAudioStateMonitor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioStateMonitor_Vtbl
    where
        Identity: IAudioStateMonitor_Impl,
    {
        unsafe extern "system" fn RegisterCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: PAudioStateMonitorCallback, context: *const core::ffi::c_void, registration: *mut i64) -> windows_core::HRESULT
        where
            Identity: IAudioStateMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioStateMonitor_Impl::RegisterCallback(this, core::mem::transmute_copy(&callback), core::mem::transmute_copy(&context)) {
                Ok(ok__) => {
                    registration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, registration: i64)
        where
            Identity: IAudioStateMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioStateMonitor_Impl::UnregisterCallback(this, core::mem::transmute_copy(&registration))
        }
        unsafe extern "system" fn GetSoundLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> AudioStateMonitorSoundLevel
        where
            Identity: IAudioStateMonitor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioStateMonitor_Impl::GetSoundLevel(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterCallback: RegisterCallback::<Identity, OFFSET>,
            UnregisterCallback: UnregisterCallback::<Identity, OFFSET>,
            GetSoundLevel: GetSoundLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioStateMonitor as windows_core::Interface>::IID
    }
}
pub trait IAudioStreamVolume_Impl: Sized {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioStreamVolume {}
impl IAudioStreamVolume_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioStreamVolume_Vtbl
    where
        Identity: IAudioStreamVolume_Impl,
    {
        unsafe extern "system" fn GetChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAudioStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioStreamVolume_Impl::GetChannelCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32) -> windows_core::HRESULT
        where
            Identity: IAudioStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioStreamVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel)).into()
        }
        unsafe extern "system" fn GetChannelVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioStreamVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pflevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> windows_core::HRESULT
        where
            Identity: IAudioStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioStreamVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
        }
        unsafe extern "system" fn GetAllVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT
        where
            Identity: IAudioStreamVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioStreamVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioStreamVolume as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Impl: Sized {
    fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAudioSystemEffectsPropertyChangeNotificationClient {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl
    where
        Identity: IAudioSystemEffectsPropertyChangeNotificationClient_Impl,
    {
        unsafe extern "system" fn OnPropertyChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyChangeNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSystemEffectsPropertyChangeNotificationClient_Impl::OnPropertyChanged(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&key)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPropertyChanged: OnPropertyChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyChangeNotificationClient as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyStore_Impl: Sized {
    fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenUserPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn ResetUserPropertyStore(&self) -> windows_core::Result<()>;
    fn ResetVolatilePropertyStore(&self) -> windows_core::Result<()>;
    fn RegisterPropertyChangeNotification(&self, callback: Option<&IAudioSystemEffectsPropertyChangeNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterPropertyChangeNotification(&self, callback: Option<&IAudioSystemEffectsPropertyChangeNotificationClient>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAudioSystemEffectsPropertyStore {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyStore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioSystemEffectsPropertyStore_Vtbl
    where
        Identity: IAudioSystemEffectsPropertyStore_Impl,
    {
        unsafe extern "system" fn OpenDefaultPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSystemEffectsPropertyStore_Impl::OpenDefaultPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                Ok(ok__) => {
                    propstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenUserPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSystemEffectsPropertyStore_Impl::OpenUserPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                Ok(ok__) => {
                    propstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAudioSystemEffectsPropertyStore_Impl::OpenVolatilePropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                Ok(ok__) => {
                    propstore.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetUserPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSystemEffectsPropertyStore_Impl::ResetUserPropertyStore(this).into()
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSystemEffectsPropertyStore_Impl::ResetVolatilePropertyStore(this).into()
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSystemEffectsPropertyStore_Impl::RegisterPropertyChangeNotification(this, windows_core::from_raw_borrowed(&callback)).into()
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IAudioSystemEffectsPropertyStore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioSystemEffectsPropertyStore_Impl::UnregisterPropertyChangeNotification(this, windows_core::from_raw_borrowed(&callback)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenDefaultPropertyStore: OpenDefaultPropertyStore::<Identity, OFFSET>,
            OpenUserPropertyStore: OpenUserPropertyStore::<Identity, OFFSET>,
            OpenVolatilePropertyStore: OpenVolatilePropertyStore::<Identity, OFFSET>,
            ResetUserPropertyStore: ResetUserPropertyStore::<Identity, OFFSET>,
            ResetVolatilePropertyStore: ResetVolatilePropertyStore::<Identity, OFFSET>,
            RegisterPropertyChangeNotification: RegisterPropertyChangeNotification::<Identity, OFFSET>,
            UnregisterPropertyChangeNotification: UnregisterPropertyChangeNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyStore as windows_core::Interface>::IID
    }
}
pub trait IAudioTreble_Impl: Sized + IPerChannelDbLevel_Impl {}
impl windows_core::RuntimeName for IAudioTreble {}
impl IAudioTreble_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioTreble_Vtbl
    where
        Identity: IAudioTreble_Impl,
    {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioTreble as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
pub trait IAudioViewManagerService_Impl: Sized {
    fn SetAudioStreamWindow(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioViewManagerService {}
impl IAudioViewManagerService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioViewManagerService_Vtbl
    where
        Identity: IAudioViewManagerService_Impl,
    {
        unsafe extern "system" fn SetAudioStreamWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IAudioViewManagerService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioViewManagerService_Impl::SetAudioStreamWindow(this, core::mem::transmute_copy(&hwnd)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetAudioStreamWindow: SetAudioStreamWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioViewManagerService as windows_core::Interface>::IID
    }
}
pub trait IAudioVolumeDuckNotification_Impl: Sized {
    fn OnVolumeDuckNotification(&self, sessionid: &windows_core::PCWSTR, countcommunicationsessions: u32) -> windows_core::Result<()>;
    fn OnVolumeUnduckNotification(&self, sessionid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAudioVolumeDuckNotification {}
impl IAudioVolumeDuckNotification_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioVolumeDuckNotification_Vtbl
    where
        Identity: IAudioVolumeDuckNotification_Impl,
    {
        unsafe extern "system" fn OnVolumeDuckNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, countcommunicationsessions: u32) -> windows_core::HRESULT
        where
            Identity: IAudioVolumeDuckNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioVolumeDuckNotification_Impl::OnVolumeDuckNotification(this, core::mem::transmute(&sessionid), core::mem::transmute_copy(&countcommunicationsessions)).into()
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IAudioVolumeDuckNotification_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAudioVolumeDuckNotification_Impl::OnVolumeUnduckNotification(this, core::mem::transmute(&sessionid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnVolumeDuckNotification: OnVolumeDuckNotification::<Identity, OFFSET>,
            OnVolumeUnduckNotification: OnVolumeUnduckNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioVolumeDuckNotification as windows_core::Interface>::IID
    }
}
pub trait IAudioVolumeLevel_Impl: Sized + IPerChannelDbLevel_Impl {}
impl windows_core::RuntimeName for IAudioVolumeLevel {}
impl IAudioVolumeLevel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAudioVolumeLevel_Vtbl
    where
        Identity: IAudioVolumeLevel_Impl,
    {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioVolumeLevel as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
pub trait IChannelAudioVolume_Impl: Sized {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChannelAudioVolume {}
impl IChannelAudioVolume_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IChannelAudioVolume_Vtbl
    where
        Identity: IChannelAudioVolume_Impl,
    {
        unsafe extern "system" fn GetChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChannelAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChannelAudioVolume_Impl::GetChannelCount(this) {
                Ok(ok__) => {
                    pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IChannelAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelAudioVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetChannelVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: IChannelAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IChannelAudioVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pflevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IChannelAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelAudioVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetAllVolumes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT
        where
            Identity: IChannelAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChannelAudioVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelAudioVolume as windows_core::Interface>::IID
    }
}
pub trait IConnector_Impl: Sized {
    fn GetType(&self) -> windows_core::Result<ConnectorType>;
    fn GetDataFlow(&self) -> windows_core::Result<DataFlow>;
    fn ConnectTo(&self, pconnectto: Option<&IConnector>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetConnectedTo(&self) -> windows_core::Result<IConnector>;
    fn GetConnectorIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDeviceIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IConnector {}
impl IConnector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConnector_Vtbl
    where
        Identity: IConnector_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut ConnectorType) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnector_Impl::GetType(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataFlow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflow: *mut DataFlow) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnector_Impl::GetDataFlow(this) {
                Ok(ok__) => {
                    pflow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectto: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnector_Impl::ConnectTo(this, windows_core::from_raw_borrowed(&pconnectto)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConnector_Impl::Disconnect(this).into()
        }
        unsafe extern "system" fn IsConnected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnector_Impl::IsConnected(this) {
                Ok(ok__) => {
                    pbconnected.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnector_Impl::GetConnectedTo(this) {
                Ok(ok__) => {
                    ppconto.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrconnectorid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnector_Impl::GetConnectorIdConnectedTo(this) {
                Ok(ok__) => {
                    ppwstrconnectorid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IConnector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConnector_Impl::GetDeviceIdConnectedTo(this) {
                Ok(ok__) => {
                    ppwstrdeviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDataFlow: GetDataFlow::<Identity, OFFSET>,
            ConnectTo: ConnectTo::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetConnectedTo: GetConnectedTo::<Identity, OFFSET>,
            GetConnectorIdConnectedTo: GetConnectorIdConnectedTo::<Identity, OFFSET>,
            GetDeviceIdConnectedTo: GetDeviceIdConnectedTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnector as windows_core::Interface>::IID
    }
}
pub trait IControlChangeNotify_Impl: Sized {
    fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IControlChangeNotify {}
impl IControlChangeNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IControlChangeNotify_Vtbl
    where
        Identity: IControlChangeNotify_Impl,
    {
        unsafe extern "system" fn OnNotify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IControlChangeNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IControlChangeNotify_Impl::OnNotify(this, core::mem::transmute_copy(&dwsenderprocessid), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlChangeNotify as windows_core::Interface>::IID
    }
}
pub trait IControlInterface_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetIID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl windows_core::RuntimeName for IControlInterface {}
impl IControlInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IControlInterface_Vtbl
    where
        Identity: IControlInterface_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IControlInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IControlInterface_Impl::GetName(this) {
                Ok(ok__) => {
                    ppwstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IControlInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IControlInterface_Impl::GetIID(this) {
                Ok(ok__) => {
                    piid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetIID: GetIID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlInterface as windows_core::Interface>::IID
    }
}
pub trait IDeviceSpecificProperty_Impl: Sized {
    fn GetType(&self) -> windows_core::Result<u16>;
    fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::Result<()>;
    fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDeviceSpecificProperty {}
impl IDeviceSpecificProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeviceSpecificProperty_Vtbl
    where
        Identity: IDeviceSpecificProperty_Impl,
    {
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *mut u16) -> windows_core::HRESULT
        where
            Identity: IDeviceSpecificProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceSpecificProperty_Impl::GetType(this) {
                Ok(ok__) => {
                    pvtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDeviceSpecificProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceSpecificProperty_Impl::GetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&pcbvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDeviceSpecificProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceSpecificProperty_Impl::SetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&cbvalue), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn Get4BRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDeviceSpecificProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceSpecificProperty_Impl::Get4BRange(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plstepping)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Get4BRange: Get4BRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceSpecificProperty as windows_core::Interface>::IID
    }
}
pub trait IDeviceTopology_Impl: Sized {
    fn GetConnectorCount(&self) -> windows_core::Result<u32>;
    fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector>;
    fn GetSubunitCount(&self) -> windows_core::Result<u32>;
    fn GetSubunit(&self, nindex: u32) -> windows_core::Result<ISubunit>;
    fn GetPartById(&self, nid: u32) -> windows_core::Result<IPart>;
    fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignalPath(&self, pipartfrom: Option<&IPart>, pipartto: Option<&IPart>, brejectmixedpaths: super::super::Foundation::BOOL) -> windows_core::Result<IPartsList>;
}
impl windows_core::RuntimeName for IDeviceTopology {}
impl IDeviceTopology_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDeviceTopology_Vtbl
    where
        Identity: IDeviceTopology_Impl,
    {
        unsafe extern "system" fn GetConnectorCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetConnectorCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnector<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetConnector(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppconnector.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunitCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetSubunitCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppsubunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetSubunit(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppsubunit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartById<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nid: u32, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetPartById(this, core::mem::transmute_copy(&nid)) {
                Ok(ok__) => {
                    pppart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetDeviceId(this) {
                Ok(ok__) => {
                    ppwstrdeviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipartfrom: *mut core::ffi::c_void, pipartto: *mut core::ffi::c_void, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDeviceTopology_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceTopology_Impl::GetSignalPath(this, windows_core::from_raw_borrowed(&pipartfrom), windows_core::from_raw_borrowed(&pipartto), core::mem::transmute_copy(&brejectmixedpaths)) {
                Ok(ok__) => {
                    ppparts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectorCount: GetConnectorCount::<Identity, OFFSET>,
            GetConnector: GetConnector::<Identity, OFFSET>,
            GetSubunitCount: GetSubunitCount::<Identity, OFFSET>,
            GetSubunit: GetSubunit::<Identity, OFFSET>,
            GetPartById: GetPartById::<Identity, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, OFFSET>,
            GetSignalPath: GetSignalPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceTopology as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMMDevice_Impl: Sized {
    fn Activate(&self, iid: *const windows_core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const windows_core::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::STGM) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetState(&self) -> windows_core::Result<DEVICE_STATE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IMMDevice {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMMDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMMDevice_Vtbl
    where
        Identity: IMMDevice_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMDevice_Impl::Activate(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&dwclsctx), core::mem::transmute_copy(&pactivationparams), core::mem::transmute_copy(&ppinterface)).into()
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: super::super::System::Com::STGM, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDevice_Impl::OpenPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstrid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IMMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDevice_Impl::GetId(this) {
                Ok(ok__) => {
                    ppstrid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut DEVICE_STATE) -> windows_core::HRESULT
        where
            Identity: IMMDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDevice_Impl::GetState(this) {
                Ok(ok__) => {
                    pdwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDevice as windows_core::Interface>::IID
    }
}
pub trait IMMDeviceActivator_Impl: Sized {
    fn Activate(&self, iid: *const windows_core::GUID, pdevice: Option<&IMMDevice>, pactivationparams: *const windows_core::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMMDeviceActivator {}
impl IMMDeviceActivator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMMDeviceActivator_Vtbl
    where
        Identity: IMMDeviceActivator_Impl,
    {
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, pdevice: *mut core::ffi::c_void, pactivationparams: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceActivator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMDeviceActivator_Impl::Activate(this, core::mem::transmute_copy(&iid), windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pactivationparams), core::mem::transmute_copy(&ppinterface)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceActivator as windows_core::Interface>::IID
    }
}
pub trait IMMDeviceCollection_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice>;
}
impl windows_core::RuntimeName for IMMDeviceCollection {}
impl IMMDeviceCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMMDeviceCollection_Vtbl
    where
        Identity: IMMDeviceCollection_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdevices: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMMDeviceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDeviceCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcdevices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ndevice: u32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDeviceCollection_Impl::Item(this, core::mem::transmute_copy(&ndevice)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, Item: Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceCollection as windows_core::Interface>::IID
    }
}
pub trait IMMDeviceEnumerator_Impl: Sized {
    fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: DEVICE_STATE) -> windows_core::Result<IMMDeviceCollection>;
    fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice>;
    fn GetDevice(&self, pwstrid: &windows_core::PCWSTR) -> windows_core::Result<IMMDevice>;
    fn RegisterEndpointNotificationCallback(&self, pclient: Option<&IMMNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterEndpointNotificationCallback(&self, pclient: Option<&IMMNotificationClient>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMMDeviceEnumerator {}
impl IMMDeviceEnumerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMMDeviceEnumerator_Vtbl
    where
        Identity: IMMDeviceEnumerator_Impl,
    {
        unsafe extern "system" fn EnumAudioEndpoints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataflow: EDataFlow, dwstatemask: DEVICE_STATE, ppdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDeviceEnumerator_Impl::EnumAudioEndpoints(this, core::mem::transmute_copy(&dataflow), core::mem::transmute_copy(&dwstatemask)) {
                Ok(ok__) => {
                    ppdevices.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDeviceEnumerator_Impl::GetDefaultAudioEndpoint(this, core::mem::transmute_copy(&dataflow), core::mem::transmute_copy(&role)) {
                Ok(ok__) => {
                    ppendpoint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrid: windows_core::PCWSTR, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMDeviceEnumerator_Impl::GetDevice(this, core::mem::transmute(&pwstrid)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclient: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMDeviceEnumerator_Impl::RegisterEndpointNotificationCallback(this, windows_core::from_raw_borrowed(&pclient)).into()
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclient: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMMDeviceEnumerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMDeviceEnumerator_Impl::UnregisterEndpointNotificationCallback(this, windows_core::from_raw_borrowed(&pclient)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumAudioEndpoints: EnumAudioEndpoints::<Identity, OFFSET>,
            GetDefaultAudioEndpoint: GetDefaultAudioEndpoint::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            RegisterEndpointNotificationCallback: RegisterEndpointNotificationCallback::<Identity, OFFSET>,
            UnregisterEndpointNotificationCallback: UnregisterEndpointNotificationCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceEnumerator as windows_core::Interface>::IID
    }
}
pub trait IMMEndpoint_Impl: Sized {
    fn GetDataFlow(&self) -> windows_core::Result<EDataFlow>;
}
impl windows_core::RuntimeName for IMMEndpoint {}
impl IMMEndpoint_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMMEndpoint_Vtbl
    where
        Identity: IMMEndpoint_Impl,
    {
        unsafe extern "system" fn GetDataFlow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataflow: *mut EDataFlow) -> windows_core::HRESULT
        where
            Identity: IMMEndpoint_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMMEndpoint_Impl::GetDataFlow(this) {
                Ok(ok__) => {
                    pdataflow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDataFlow: GetDataFlow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMEndpoint as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IMMNotificationClient_Impl: Sized {
    fn OnDeviceStateChanged(&self, pwstrdeviceid: &windows_core::PCWSTR, dwnewstate: DEVICE_STATE) -> windows_core::Result<()>;
    fn OnDeviceAdded(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDeviceRemoved(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDefaultDeviceChanged(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnPropertyValueChanged(&self, pwstrdeviceid: &windows_core::PCWSTR, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IMMNotificationClient {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IMMNotificationClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMMNotificationClient_Vtbl
    where
        Identity: IMMNotificationClient_Impl,
    {
        unsafe extern "system" fn OnDeviceStateChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR, dwnewstate: DEVICE_STATE) -> windows_core::HRESULT
        where
            Identity: IMMNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMNotificationClient_Impl::OnDeviceStateChanged(this, core::mem::transmute(&pwstrdeviceid), core::mem::transmute_copy(&dwnewstate)).into()
        }
        unsafe extern "system" fn OnDeviceAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMMNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMNotificationClient_Impl::OnDeviceAdded(this, core::mem::transmute(&pwstrdeviceid)).into()
        }
        unsafe extern "system" fn OnDeviceRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMMNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMNotificationClient_Impl::OnDeviceRemoved(this, core::mem::transmute(&pwstrdeviceid)).into()
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IMMNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMNotificationClient_Impl::OnDefaultDeviceChanged(this, core::mem::transmute_copy(&flow), core::mem::transmute_copy(&role), core::mem::transmute(&pwstrdefaultdeviceid)).into()
        }
        unsafe extern "system" fn OnPropertyValueChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT
        where
            Identity: IMMNotificationClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMMNotificationClient_Impl::OnPropertyValueChanged(this, core::mem::transmute(&pwstrdeviceid), core::mem::transmute(&key)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDeviceStateChanged: OnDeviceStateChanged::<Identity, OFFSET>,
            OnDeviceAdded: OnDeviceAdded::<Identity, OFFSET>,
            OnDeviceRemoved: OnDeviceRemoved::<Identity, OFFSET>,
            OnDefaultDeviceChanged: OnDefaultDeviceChanged::<Identity, OFFSET>,
            OnPropertyValueChanged: OnPropertyValueChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMNotificationClient as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageFilter_Impl: Sized {
    fn HandleInComingCall(&self, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32;
    fn RetryRejectedCall(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32;
    fn MessagePending(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMessageFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IMessageFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMessageFilter_Vtbl
    where
        Identity: IMessageFilter_Impl,
    {
        unsafe extern "system" fn HandleInComingCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32
        where
            Identity: IMessageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageFilter_Impl::HandleInComingCall(this, core::mem::transmute_copy(&dwcalltype), core::mem::transmute_copy(&htaskcaller), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&lpinterfaceinfo))
        }
        unsafe extern "system" fn RetryRejectedCall<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32
        where
            Identity: IMessageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageFilter_Impl::RetryRejectedCall(this, core::mem::transmute_copy(&htaskcallee), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&dwrejecttype))
        }
        unsafe extern "system" fn MessagePending<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32
        where
            Identity: IMessageFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMessageFilter_Impl::MessagePending(this, core::mem::transmute_copy(&htaskcallee), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&dwpendingtype))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleInComingCall: HandleInComingCall::<Identity, OFFSET>,
            RetryRejectedCall: RetryRejectedCall::<Identity, OFFSET>,
            MessagePending: MessagePending::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageFilter as windows_core::Interface>::IID
    }
}
pub trait IPart_Impl: Sized {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLocalId(&self) -> windows_core::Result<u32>;
    fn GetGlobalId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPartType(&self) -> windows_core::Result<PartType>;
    fn GetSubType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetControlInterfaceCount(&self) -> windows_core::Result<u32>;
    fn GetControlInterface(&self, nindex: u32) -> windows_core::Result<IControlInterface>;
    fn EnumPartsIncoming(&self) -> windows_core::Result<IPartsList>;
    fn EnumPartsOutgoing(&self) -> windows_core::Result<IPartsList>;
    fn GetTopologyObject(&self) -> windows_core::Result<IDeviceTopology>;
    fn Activate(&self, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RegisterControlChangeCallback(&self, riid: *const windows_core::GUID, pnotify: Option<&IControlChangeNotify>) -> windows_core::Result<()>;
    fn UnregisterControlChangeCallback(&self, pnotify: Option<&IControlChangeNotify>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPart {}
impl IPart_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPart_Vtbl
    where
        Identity: IPart_Impl,
    {
        unsafe extern "system" fn GetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetName(this) {
                Ok(ok__) => {
                    ppwstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetLocalId(this) {
                Ok(ok__) => {
                    pnid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrglobalid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetGlobalId(this) {
                Ok(ok__) => {
                    ppwstrglobalid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparttype: *mut PartType) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetPartType(this) {
                Ok(ok__) => {
                    pparttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubtype: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetSubType(this) {
                Ok(ok__) => {
                    psubtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetControlInterfaceCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterface<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetControlInterface(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    ppinterfacedesc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::EnumPartsIncoming(this) {
                Ok(ok__) => {
                    ppparts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::EnumPartsOutgoing(this) {
                Ok(ok__) => {
                    ppparts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTopologyObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPart_Impl::GetTopologyObject(this) {
                Ok(ok__) => {
                    pptopology.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPart_Impl::Activate(this, core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPart_Impl::RegisterControlChangeCallback(this, core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&pnotify)).into()
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPart_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPart_Impl::UnregisterControlChangeCallback(this, windows_core::from_raw_borrowed(&pnotify)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetLocalId: GetLocalId::<Identity, OFFSET>,
            GetGlobalId: GetGlobalId::<Identity, OFFSET>,
            GetPartType: GetPartType::<Identity, OFFSET>,
            GetSubType: GetSubType::<Identity, OFFSET>,
            GetControlInterfaceCount: GetControlInterfaceCount::<Identity, OFFSET>,
            GetControlInterface: GetControlInterface::<Identity, OFFSET>,
            EnumPartsIncoming: EnumPartsIncoming::<Identity, OFFSET>,
            EnumPartsOutgoing: EnumPartsOutgoing::<Identity, OFFSET>,
            GetTopologyObject: GetTopologyObject::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            RegisterControlChangeCallback: RegisterControlChangeCallback::<Identity, OFFSET>,
            UnregisterControlChangeCallback: UnregisterControlChangeCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPart as windows_core::Interface>::IID
    }
}
pub trait IPartsList_Impl: Sized {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetPart(&self, nindex: u32) -> windows_core::Result<IPart>;
}
impl windows_core::RuntimeName for IPartsList {}
impl IPartsList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPartsList_Vtbl
    where
        Identity: IPartsList_Impl,
    {
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPartsList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartsList_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPartsList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPartsList_Impl::GetPart(this, core::mem::transmute_copy(&nindex)) {
                Ok(ok__) => {
                    pppart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetPart: GetPart::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartsList as windows_core::Interface>::IID
    }
}
pub trait IPerChannelDbLevel_Impl: Sized {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::Result<()>;
    fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPerChannelDbLevel {}
impl IPerChannelDbLevel_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPerChannelDbLevel_Vtbl
    where
        Identity: IPerChannelDbLevel_Impl,
    {
        unsafe extern "system" fn GetChannelCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut u32) -> windows_core::HRESULT
        where
            Identity: IPerChannelDbLevel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerChannelDbLevel_Impl::GetChannelCount(this) {
                Ok(ok__) => {
                    pcchannels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::HRESULT
        where
            Identity: IPerChannelDbLevel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerChannelDbLevel_Impl::GetLevelRange(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&pfminleveldb), core::mem::transmute_copy(&pfmaxleveldb), core::mem::transmute_copy(&pfstepping)).into()
        }
        unsafe extern "system" fn GetLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> windows_core::HRESULT
        where
            Identity: IPerChannelDbLevel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPerChannelDbLevel_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel)) {
                Ok(ok__) => {
                    pfleveldb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPerChannelDbLevel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerChannelDbLevel_Impl::SetLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetLevelUniform<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPerChannelDbLevel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerChannelDbLevel_Impl::SetLevelUniform(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetLevelAllChannels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IPerChannelDbLevel_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPerChannelDbLevel_Impl::SetLevelAllChannels(this, core::mem::transmute_copy(&alevelsdb), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            GetLevelRange: GetLevelRange::<Identity, OFFSET>,
            GetLevel: GetLevel::<Identity, OFFSET>,
            SetLevel: SetLevel::<Identity, OFFSET>,
            SetLevelUniform: SetLevelUniform::<Identity, OFFSET>,
            SetLevelAllChannels: SetLevelAllChannels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
pub trait ISimpleAudioVolume_Impl: Sized {
    fn SetMasterVolume(&self, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMasterVolume(&self) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: super::super::Foundation::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ISimpleAudioVolume {}
impl ISimpleAudioVolume_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISimpleAudioVolume_Vtbl
    where
        Identity: ISimpleAudioVolume_Impl,
    {
        unsafe extern "system" fn SetMasterVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISimpleAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleAudioVolume_Impl::SetMasterVolume(this, core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetMasterVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT
        where
            Identity: ISimpleAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISimpleAudioVolume_Impl::GetMasterVolume(this) {
                Ok(ok__) => {
                    pflevel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISimpleAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleAudioVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute), core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISimpleAudioVolume_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISimpleAudioVolume_Impl::GetMute(this) {
                Ok(ok__) => {
                    pbmute.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMasterVolume: SetMasterVolume::<Identity, OFFSET>,
            GetMasterVolume: GetMasterVolume::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimpleAudioVolume as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioClient_Impl: Sized {
    fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::Result<()>;
    fn GetNativeStaticObjectTypeMask(&self) -> windows_core::Result<AudioObjectType>;
    fn GetMaxDynamicObjectCount(&self) -> windows_core::Result<u32>;
    fn GetSupportedAudioObjectFormatEnumerator(&self) -> windows_core::Result<IAudioFormatEnumerator>;
    fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32>;
    fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<()>;
    fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const windows_core::GUID, auxiliaryinfo: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn ActivateSpatialAudioStream(&self, activationparams: *const windows_core::PROPVARIANT, riid: *const windows_core::GUID, stream: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioClient {}
impl ISpatialAudioClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioClient_Vtbl
    where
        Identity: ISpatialAudioClient_Impl,
    {
        unsafe extern "system" fn GetStaticObjectPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioClient_Impl::GetStaticObjectPosition(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: *mut AudioObjectType) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioClient_Impl::GetNativeStaticObjectTypeMask(this) {
                Ok(ok__) => {
                    mask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioClient_Impl::GetMaxDynamicObjectCount(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioClient_Impl::GetSupportedAudioObjectFormatEnumerator(this) {
                Ok(ok__) => {
                    enumerator.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioClient_Impl::GetMaxFrameCount(this, core::mem::transmute_copy(&objectformat)) {
                Ok(ok__) => {
                    framecountperbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioClient_Impl::IsAudioObjectFormatSupported(this, core::mem::transmute_copy(&objectformat)).into()
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamuuid: *const windows_core::GUID, auxiliaryinfo: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioClient_Impl::IsSpatialAudioStreamAvailable(this, core::mem::transmute_copy(&streamuuid), core::mem::transmute_copy(&auxiliaryinfo)).into()
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, activationparams: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, riid: *const windows_core::GUID, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioClient_Impl::ActivateSpatialAudioStream(this, core::mem::transmute_copy(&activationparams), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&stream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStaticObjectPosition: GetStaticObjectPosition::<Identity, OFFSET>,
            GetNativeStaticObjectTypeMask: GetNativeStaticObjectTypeMask::<Identity, OFFSET>,
            GetMaxDynamicObjectCount: GetMaxDynamicObjectCount::<Identity, OFFSET>,
            GetSupportedAudioObjectFormatEnumerator: GetSupportedAudioObjectFormatEnumerator::<Identity, OFFSET>,
            GetMaxFrameCount: GetMaxFrameCount::<Identity, OFFSET>,
            IsAudioObjectFormatSupported: IsAudioObjectFormatSupported::<Identity, OFFSET>,
            IsSpatialAudioStreamAvailable: IsSpatialAudioStreamAvailable::<Identity, OFFSET>,
            ActivateSpatialAudioStream: ActivateSpatialAudioStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioClient as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioClient2_Impl: Sized + ISpatialAudioClient_Impl {
    fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetMaxFrameCountForCategory(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISpatialAudioClient2 {}
impl ISpatialAudioClient2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioClient2_Vtbl
    where
        Identity: ISpatialAudioClient2_Impl,
    {
        unsafe extern "system" fn IsOffloadCapable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioClient2_Impl::IsOffloadCapable(this, core::mem::transmute_copy(&category)) {
                Ok(ok__) => {
                    isoffloadcapable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioClient2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioClient2_Impl::GetMaxFrameCountForCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&offloadenabled), core::mem::transmute_copy(&objectformat)) {
                Ok(ok__) => {
                    framecountperbuffer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioClient_Vtbl::new::<Identity, OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Identity, OFFSET>,
            GetMaxFrameCountForCategory: GetMaxFrameCountForCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioClient2 as windows_core::Interface>::IID || iid == &<ISpatialAudioClient as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataClient_Impl: Sized {
    fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut Option<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> windows_core::Result<u32>;
    fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> windows_core::Result<ISpatialAudioMetadataWriter>;
    fn ActivateSpatialAudioMetadataCopier(&self) -> windows_core::Result<ISpatialAudioMetadataCopier>;
    fn ActivateSpatialAudioMetadataReader(&self) -> windows_core::Result<ISpatialAudioMetadataReader>;
}
impl windows_core::RuntimeName for ISpatialAudioMetadataClient {}
impl ISpatialAudioMetadataClient_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioMetadataClient_Vtbl
    where
        Identity: ISpatialAudioMetadataClient_Impl,
    {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut *mut core::ffi::c_void, metadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataItems(this, core::mem::transmute_copy(&maxitemcount), core::mem::transmute_copy(&framecount), core::mem::transmute_copy(&metadataitemsbuffer), core::mem::transmute_copy(&metadataitems)).into()
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataClient_Impl::GetSpatialAudioMetadataItemsBufferLength(this, core::mem::transmute_copy(&maxitemcount)) {
                Ok(ok__) => {
                    bufferlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataWriter(this, core::mem::transmute_copy(&overflowmode)) {
                Ok(ok__) => {
                    metadatawriter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatacopier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataCopier(this) {
                Ok(ok__) => {
                    metadatacopier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatareader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataClient_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataReader(this) {
                Ok(ok__) => {
                    metadatareader.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioMetadataItems: ActivateSpatialAudioMetadataItems::<Identity, OFFSET>,
            GetSpatialAudioMetadataItemsBufferLength: GetSpatialAudioMetadataItemsBufferLength::<Identity, OFFSET>,
            ActivateSpatialAudioMetadataWriter: ActivateSpatialAudioMetadataWriter::<Identity, OFFSET>,
            ActivateSpatialAudioMetadataCopier: ActivateSpatialAudioMetadataCopier::<Identity, OFFSET>,
            ActivateSpatialAudioMetadataReader: ActivateSpatialAudioMetadataReader::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataClient as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataCopier_Impl: Sized {
    fn Open(&self, metadataitems: Option<&ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn CopyMetadataForFrames(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: Option<&ISpatialAudioMetadataItems>) -> windows_core::Result<u16>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioMetadataCopier {}
impl ISpatialAudioMetadataCopier_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioMetadataCopier_Vtbl
    where
        Identity: ISpatialAudioMetadataCopier_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataCopier_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataCopier_Impl::Open(this, windows_core::from_raw_borrowed(&metadataitems)).into()
        }
        unsafe extern "system" fn CopyMetadataForFrames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: *mut core::ffi::c_void, itemscopied: *mut u16) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataCopier_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataCopier_Impl::CopyMetadataForFrames(this, core::mem::transmute_copy(&copyframecount), core::mem::transmute_copy(&copymode), windows_core::from_raw_borrowed(&dstmetadataitems)) {
                Ok(ok__) => {
                    itemscopied.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataCopier_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataCopier_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            CopyMetadataForFrames: CopyMetadataForFrames::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataCopier as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataItems_Impl: Sized {
    fn GetFrameCount(&self) -> windows_core::Result<u16>;
    fn GetItemCount(&self) -> windows_core::Result<u16>;
    fn GetMaxItemCount(&self) -> windows_core::Result<u16>;
    fn GetMaxValueBufferLength(&self) -> windows_core::Result<u32>;
    fn GetInfo(&self) -> windows_core::Result<SpatialAudioMetadataItemsInfo>;
}
impl windows_core::RuntimeName for ISpatialAudioMetadataItems {}
impl ISpatialAudioMetadataItems_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioMetadataItems_Vtbl
    where
        Identity: ISpatialAudioMetadataItems_Impl,
    {
        unsafe extern "system" fn GetFrameCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, framecount: *mut u16) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataItems_Impl::GetFrameCount(this) {
                Ok(ok__) => {
                    framecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemcount: *mut u16) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataItems_Impl::GetItemCount(this) {
                Ok(ok__) => {
                    itemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: *mut u16) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataItems_Impl::GetMaxItemCount(this) {
                Ok(ok__) => {
                    maxitemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxvaluebufferlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataItems_Impl::GetMaxValueBufferLength(this) {
                Ok(ok__) => {
                    maxvaluebufferlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioMetadataItems_Impl::GetInfo(this) {
                Ok(ok__) => {
                    info.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameCount: GetFrameCount::<Identity, OFFSET>,
            GetItemCount: GetItemCount::<Identity, OFFSET>,
            GetMaxItemCount: GetMaxItemCount::<Identity, OFFSET>,
            GetMaxValueBufferLength: GetMaxValueBufferLength::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItems as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataItemsBuffer_Impl: Sized {
    fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> windows_core::Result<()>;
    fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> windows_core::Result<()>;
    fn DetachBuffer(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioMetadataItemsBuffer {}
impl ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioMetadataItemsBuffer_Vtbl
    where
        Identity: ISpatialAudioMetadataItemsBuffer_Impl,
    {
        unsafe extern "system" fn AttachToBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItemsBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataItemsBuffer_Impl::AttachToBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItemsBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataItemsBuffer_Impl::AttachToPopulatedBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn DetachBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataItemsBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataItemsBuffer_Impl::DetachBuffer(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AttachToBuffer: AttachToBuffer::<Identity, OFFSET>,
            AttachToPopulatedBuffer: AttachToPopulatedBuffer::<Identity, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItemsBuffer as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataReader_Impl: Sized {
    fn Open(&self, metadataitems: Option<&ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::Result<()>;
    fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioMetadataReader {}
impl ISpatialAudioMetadataReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioMetadataReader_Vtbl
    where
        Identity: ISpatialAudioMetadataReader_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataReader_Impl::Open(this, windows_core::from_raw_borrowed(&metadataitems)).into()
        }
        unsafe extern "system" fn ReadNextItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataReader_Impl::ReadNextItem(this, core::mem::transmute_copy(&commandcount), core::mem::transmute_copy(&frameoffset)).into()
        }
        unsafe extern "system" fn ReadNextItemCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataReader_Impl::ReadNextItemCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&maxvaluebufferlength), core::mem::transmute_copy(&valuebufferlength)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataReader_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            ReadNextItem: ReadNextItem::<Identity, OFFSET>,
            ReadNextItemCommand: ReadNextItemCommand::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataReader as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataWriter_Impl: Sized {
    fn Open(&self, metadataitems: Option<&ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn WriteNextItem(&self, frameoffset: u16) -> windows_core::Result<()>;
    fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioMetadataWriter {}
impl ISpatialAudioMetadataWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioMetadataWriter_Vtbl
    where
        Identity: ISpatialAudioMetadataWriter_Impl,
    {
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataWriter_Impl::Open(this, windows_core::from_raw_borrowed(&metadataitems)).into()
        }
        unsafe extern "system" fn WriteNextItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameoffset: u16) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataWriter_Impl::WriteNextItem(this, core::mem::transmute_copy(&frameoffset)).into()
        }
        unsafe extern "system" fn WriteNextItemCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataWriter_Impl::WriteNextItemCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&valuebufferlength)).into()
        }
        unsafe extern "system" fn Close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioMetadataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioMetadataWriter_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            WriteNextItem: WriteNextItem::<Identity, OFFSET>,
            WriteNextItemCommand: WriteNextItemCommand::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataWriter as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObject_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()>;
    fn SetVolume(&self, volume: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioObject {}
impl ISpatialAudioObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObject_Vtbl
    where
        Identity: ISpatialAudioObject_Impl,
    {
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObject_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: f32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObject_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(),
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObject as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectBase_Impl: Sized {
    fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::Result<()>;
    fn SetEndOfStream(&self, framecount: u32) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAudioObjectType(&self) -> windows_core::Result<AudioObjectType>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectBase {}
impl ISpatialAudioObjectBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectBase_Vtbl
    where
        Identity: ISpatialAudioObjectBase_Impl,
    {
        unsafe extern "system" fn GetBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectBase_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn SetEndOfStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, framecount: u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectBase_Impl::SetEndOfStream(this, core::mem::transmute_copy(&framecount)).into()
        }
        unsafe extern "system" fn IsActive<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectBase_Impl::IsActive(this) {
                Ok(ok__) => {
                    isactive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioObjectType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectBase_Impl::GetAudioObjectType(this) {
                Ok(ok__) => {
                    audioobjecttype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SetEndOfStream: SetEndOfStream::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            GetAudioObjectType: GetAudioObjectType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectForHrtf_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()>;
    fn SetGain(&self, gain: f32) -> windows_core::Result<()>;
    fn SetOrientation(&self, orientation: *const *const f32) -> windows_core::Result<()>;
    fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> windows_core::Result<()>;
    fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> windows_core::Result<()>;
    fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectForHrtf {}
impl ISpatialAudioObjectForHrtf_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectForHrtf_Vtbl
    where
        Identity: ISpatialAudioObjectForHrtf_Impl,
    {
        unsafe extern "system" fn SetPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForHrtf_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn SetGain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, gain: f32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForHrtf_Impl::SetGain(this, core::mem::transmute_copy(&gain)).into()
        }
        unsafe extern "system" fn SetOrientation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, orientation: *const *const f32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForHrtf_Impl::SetOrientation(this, core::mem::transmute_copy(&orientation)).into()
        }
        unsafe extern "system" fn SetEnvironment<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForHrtf_Impl::SetEnvironment(this, core::mem::transmute_copy(&environment)).into()
        }
        unsafe extern "system" fn SetDistanceDecay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForHrtf_Impl::SetDistanceDecay(this, core::mem::transmute_copy(&distancedecay)).into()
        }
        unsafe extern "system" fn SetDirectivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForHrtf_Impl::SetDirectivity(this, core::mem::transmute_copy(&directivity)).into()
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(),
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetGain: SetGain::<Identity, OFFSET>,
            SetOrientation: SetOrientation::<Identity, OFFSET>,
            SetEnvironment: SetEnvironment::<Identity, OFFSET>,
            SetDistanceDecay: SetDistanceDecay::<Identity, OFFSET>,
            SetDirectivity: SetDirectivity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForHrtf as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectForMetadataCommands_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectForMetadataCommands {}
impl ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectForMetadataCommands_Vtbl
    where
        Identity: ISpatialAudioObjectForMetadataCommands_Impl,
    {
        unsafe extern "system" fn WriteNextMetadataCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForMetadataCommands_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectForMetadataCommands_Impl::WriteNextMetadataCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&valuebufferlength)).into()
        }
        Self { base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(), WriteNextMetadataCommand: WriteNextMetadataCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataCommands as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectForMetadataItems_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn GetSpatialAudioMetadataItems(&self) -> windows_core::Result<ISpatialAudioMetadataItems>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectForMetadataItems {}
impl ISpatialAudioObjectForMetadataItems_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectForMetadataItems_Vtbl
    where
        Identity: ISpatialAudioObjectForMetadataItems_Impl,
    {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectForMetadataItems_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectForMetadataItems_Impl::GetSpatialAudioMetadataItems(this) {
                Ok(ok__) => {
                    metadataitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(), GetSpatialAudioMetadataItems: GetSpatialAudioMetadataItems::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataItems as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStream_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObject>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStream {}
impl ISpatialAudioObjectRenderStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStream_Vtbl
    where
        Identity: ISpatialAudioObjectRenderStream_Impl,
    {
        unsafe extern "system" fn ActivateSpatialAudioObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectRenderStream_Impl::ActivateSpatialAudioObject(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    audioobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioObject: ActivateSpatialAudioObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStream as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamBase_Impl: Sized {
    fn GetAvailableDynamicObjectCount(&self) -> windows_core::Result<u32>;
    fn GetService(&self, riid: *const windows_core::GUID, service: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::Result<()>;
    fn EndUpdatingAudioObjects(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamBase {}
impl ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamBase_Vtbl
    where
        Identity: ISpatialAudioObjectRenderStreamBase_Impl,
    {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectRenderStreamBase_Impl::GetAvailableDynamicObjectCount(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamBase_Impl::GetService(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&service)).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamBase_Impl::Start(this).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamBase_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamBase_Impl::Reset(this).into()
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamBase_Impl::BeginUpdatingAudioObjects(this, core::mem::transmute_copy(&availabledynamicobjectcount), core::mem::transmute_copy(&framecountperbuffer)).into()
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamBase_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamBase_Impl::EndUpdatingAudioObjects(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableDynamicObjectCount: GetAvailableDynamicObjectCount::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            BeginUpdatingAudioObjects: BeginUpdatingAudioObjects::<Identity, OFFSET>,
            EndUpdatingAudioObjects: EndUpdatingAudioObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamForHrtf_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForHrtf>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamForHrtf {}
impl ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForHrtf_Vtbl
    where
        Identity: ISpatialAudioObjectRenderStreamForHrtf_Impl,
    {
        unsafe extern "system" fn ActivateSpatialAudioObjectForHrtf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamForHrtf_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectRenderStreamForHrtf_Impl::ActivateSpatialAudioObjectForHrtf(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    audioobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioObjectForHrtf: ActivateSpatialAudioObjectForHrtf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForHrtf as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamForMetadata_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataCommands>;
    fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataItems>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamForMetadata {}
impl ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForMetadata_Vtbl
    where
        Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl,
    {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectRenderStreamForMetadata_Impl::ActivateSpatialAudioObjectForMetadataCommands(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    audioobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpatialAudioObjectRenderStreamForMetadata_Impl::ActivateSpatialAudioObjectForMetadataItems(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    audioobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioObjectForMetadataCommands: ActivateSpatialAudioObjectForMetadataCommands::<Identity, OFFSET>,
            ActivateSpatialAudioObjectForMetadataItems: ActivateSpatialAudioObjectForMetadataItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForMetadata as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamNotify_Impl: Sized {
    fn OnAvailableDynamicObjectCountChange(&self, sender: Option<&ISpatialAudioObjectRenderStreamBase>, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamNotify {}
impl ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamNotify_Vtbl
    where
        Identity: ISpatialAudioObjectRenderStreamNotify_Impl,
    {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::HRESULT
        where
            Identity: ISpatialAudioObjectRenderStreamNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpatialAudioObjectRenderStreamNotify_Impl::OnAvailableDynamicObjectCountChange(this, windows_core::from_raw_borrowed(&sender), core::mem::transmute_copy(&hnscompliancedeadlinetime), core::mem::transmute_copy(&availabledynamicobjectcountchange)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAvailableDynamicObjectCountChange: OnAvailableDynamicObjectCountChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamNotify as windows_core::Interface>::IID
    }
}
pub trait ISubunit_Impl: Sized {}
impl windows_core::RuntimeName for ISubunit {}
impl ISubunit_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISubunit_Vtbl
    where
        Identity: ISubunit_Impl,
    {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISubunit as windows_core::Interface>::IID
    }
}
