pub trait IXAPO_Impl: Sized {
    fn GetRegistrationProperties(&self) -> windows_core::Result<*mut XAPO_REGISTRATION_PROPERTIES>;
    fn IsInputFormatSupported(&self, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn IsOutputFormatSupported(&self, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn Initialize(&self, pdata: *const core::ffi::c_void, databytesize: u32) -> windows_core::Result<()>;
    fn Reset(&self);
    fn LockForProcess(&self, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> windows_core::Result<()>;
    fn UnlockForProcess(&self);
    fn Process(&self, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL);
    fn CalcInputFrames(&self, outputframecount: u32) -> u32;
    fn CalcOutputFrames(&self, inputframecount: u32) -> u32;
}
impl windows_core::RuntimeName for IXAPO {}
impl IXAPO_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>() -> IXAPO_Vtbl {
        unsafe extern "system" fn GetRegistrationProperties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IXAPO_Impl::GetRegistrationProperties(this) {
                Ok(ok__) => {
                    core::ptr::write(ppregistrationproperties, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::IsInputFormatSupported(this, core::mem::transmute_copy(&poutputformat), core::mem::transmute_copy(&prequestedinputformat), core::mem::transmute_copy(&ppsupportedinputformat)).into()
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::IsOutputFormatSupported(this, core::mem::transmute_copy(&pinputformat), core::mem::transmute_copy(&prequestedoutputformat), core::mem::transmute_copy(&ppsupportedoutputformat)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, databytesize: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::Initialize(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&databytesize)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::Reset(this)
        }
        unsafe extern "system" fn LockForProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::LockForProcess(this, core::mem::transmute_copy(&inputlockedparametercount), core::mem::transmute_copy(&pinputlockedparameters), core::mem::transmute_copy(&outputlockedparametercount), core::mem::transmute_copy(&poutputlockedparameters)).into()
        }
        unsafe extern "system" fn UnlockForProcess<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::UnlockForProcess(this)
        }
        unsafe extern "system" fn Process<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: super::super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::Process(this, core::mem::transmute_copy(&inputprocessparametercount), core::mem::transmute_copy(&pinputprocessparameters), core::mem::transmute_copy(&outputprocessparametercount), core::mem::transmute_copy(&poutputprocessparameters), core::mem::transmute_copy(&isenabled))
        }
        unsafe extern "system" fn CalcInputFrames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputframecount: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::CalcInputFrames(this, core::mem::transmute_copy(&outputframecount))
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputframecount: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPO_Impl::CalcOutputFrames(this, core::mem::transmute_copy(&inputframecount))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRegistrationProperties: GetRegistrationProperties::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            LockForProcess: LockForProcess::<Identity, Impl, OFFSET>,
            UnlockForProcess: UnlockForProcess::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
            CalcInputFrames: CalcInputFrames::<Identity, Impl, OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAPO as windows_core::Interface>::IID
    }
}
pub trait IXAPOHrtfParameters_Impl: Sized {
    fn SetSourcePosition(&self, position: *const HrtfPosition) -> windows_core::Result<()>;
    fn SetSourceOrientation(&self, orientation: *const HrtfOrientation) -> windows_core::Result<()>;
    fn SetSourceGain(&self, gain: f32) -> windows_core::Result<()>;
    fn SetEnvironment(&self, environment: HrtfEnvironment) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXAPOHrtfParameters {}
impl IXAPOHrtfParameters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>() -> IXAPOHrtfParameters_Vtbl {
        unsafe extern "system" fn SetSourcePosition<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: *const HrtfPosition) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPOHrtfParameters_Impl::SetSourcePosition(this, core::mem::transmute_copy(&position)).into()
        }
        unsafe extern "system" fn SetSourceOrientation<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, orientation: *const HrtfOrientation) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPOHrtfParameters_Impl::SetSourceOrientation(this, core::mem::transmute_copy(&orientation)).into()
        }
        unsafe extern "system" fn SetSourceGain<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gain: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPOHrtfParameters_Impl::SetSourceGain(this, core::mem::transmute_copy(&gain)).into()
        }
        unsafe extern "system" fn SetEnvironment<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: HrtfEnvironment) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPOHrtfParameters_Impl::SetEnvironment(this, core::mem::transmute_copy(&environment)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSourcePosition: SetSourcePosition::<Identity, Impl, OFFSET>,
            SetSourceOrientation: SetSourceOrientation::<Identity, Impl, OFFSET>,
            SetSourceGain: SetSourceGain::<Identity, Impl, OFFSET>,
            SetEnvironment: SetEnvironment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAPOHrtfParameters as windows_core::Interface>::IID
    }
}
pub trait IXAPOParameters_Impl: Sized {
    fn SetParameters(&self, pparameters: *const core::ffi::c_void, parameterbytesize: u32);
    fn GetParameters(&self, pparameters: *mut core::ffi::c_void, parameterbytesize: u32);
}
impl windows_core::RuntimeName for IXAPOParameters {}
impl IXAPOParameters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOParameters_Impl, const OFFSET: isize>() -> IXAPOParameters_Vtbl {
        unsafe extern "system" fn SetParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *const core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPOParameters_Impl::SetParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parameterbytesize))
        }
        unsafe extern "system" fn GetParameters<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, parameterbytesize: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAPOParameters_Impl::GetParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parameterbytesize))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, Impl, OFFSET>,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAPOParameters as windows_core::Interface>::IID
    }
}
pub trait IXAudio2_Impl: Sized {
    fn RegisterForCallbacks(&self, pcallback: Option<&IXAudio2EngineCallback>) -> windows_core::Result<()>;
    fn UnregisterForCallbacks(&self, pcallback: Option<&IXAudio2EngineCallback>);
    fn CreateSourceVoice(&self, ppsourcevoice: *mut Option<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: Option<&IXAudio2VoiceCallback>, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn CreateSubmixVoice(&self, ppsubmixvoice: *mut Option<IXAudio2SubmixVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn CreateMasteringVoice(&self, ppmasteringvoice: *mut Option<IXAudio2MasteringVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: &windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::Result<()>;
    fn StartEngine(&self) -> windows_core::Result<()>;
    fn StopEngine(&self);
    fn CommitChanges(&self, operationset: u32) -> windows_core::Result<()>;
    fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA);
    fn SetDebugConfiguration(&self, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *const core::ffi::c_void);
}
impl windows_core::RuntimeName for IXAudio2 {}
impl IXAudio2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>() -> IXAudio2_Vtbl {
        unsafe extern "system" fn RegisterForCallbacks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::RegisterForCallbacks(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn UnregisterForCallbacks<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::UnregisterForCallbacks(this, windows_core::from_raw_borrowed(&pcallback))
        }
        unsafe extern "system" fn CreateSourceVoice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsourcevoice: *mut *mut core::ffi::c_void, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: *mut core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::CreateSourceVoice(this, core::mem::transmute_copy(&ppsourcevoice), core::mem::transmute_copy(&psourceformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&maxfrequencyratio), windows_core::from_raw_borrowed(&pcallback), core::mem::transmute_copy(&psendlist), core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn CreateSubmixVoice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubmixvoice: *mut *mut core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::CreateSubmixVoice(this, core::mem::transmute_copy(&ppsubmixvoice), core::mem::transmute_copy(&inputchannels), core::mem::transmute_copy(&inputsamplerate), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&processingstage), core::mem::transmute_copy(&psendlist), core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn CreateMasteringVoice<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmasteringvoice: *mut *mut core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::CreateMasteringVoice(this, core::mem::transmute_copy(&ppmasteringvoice), core::mem::transmute_copy(&inputchannels), core::mem::transmute_copy(&inputsamplerate), core::mem::transmute_copy(&flags), core::mem::transmute(&szdeviceid), core::mem::transmute_copy(&peffectchain), core::mem::transmute_copy(&streamcategory)).into()
        }
        unsafe extern "system" fn StartEngine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::StartEngine(this).into()
        }
        unsafe extern "system" fn StopEngine<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::StopEngine(this)
        }
        unsafe extern "system" fn CommitChanges<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::CommitChanges(this, core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetPerformanceData<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::GetPerformanceData(this, core::mem::transmute_copy(&pperfdata))
        }
        unsafe extern "system" fn SetDebugConfiguration<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *const core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2_Impl::SetDebugConfiguration(this, core::mem::transmute_copy(&pdebugconfiguration), core::mem::transmute_copy(&preserved))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForCallbacks: RegisterForCallbacks::<Identity, Impl, OFFSET>,
            UnregisterForCallbacks: UnregisterForCallbacks::<Identity, Impl, OFFSET>,
            CreateSourceVoice: CreateSourceVoice::<Identity, Impl, OFFSET>,
            CreateSubmixVoice: CreateSubmixVoice::<Identity, Impl, OFFSET>,
            CreateMasteringVoice: CreateMasteringVoice::<Identity, Impl, OFFSET>,
            StartEngine: StartEngine::<Identity, Impl, OFFSET>,
            StopEngine: StopEngine::<Identity, Impl, OFFSET>,
            CommitChanges: CommitChanges::<Identity, Impl, OFFSET>,
            GetPerformanceData: GetPerformanceData::<Identity, Impl, OFFSET>,
            SetDebugConfiguration: SetDebugConfiguration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAudio2 as windows_core::Interface>::IID
    }
}
pub trait IXAudio2EngineCallback_Impl: Sized {
    fn OnProcessingPassStart(&self);
    fn OnProcessingPassEnd(&self);
    fn OnCriticalError(&self, error: windows_core::HRESULT);
}
impl IXAudio2EngineCallback_Vtbl {
    pub const fn new<Impl: IXAudio2EngineCallback_Impl>() -> IXAudio2EngineCallback_Vtbl {
        unsafe extern "system" fn OnProcessingPassStart<Impl: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2EngineCallback_Impl::OnProcessingPassStart(this)
        }
        unsafe extern "system" fn OnProcessingPassEnd<Impl: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2EngineCallback_Impl::OnProcessingPassEnd(this)
        }
        unsafe extern "system" fn OnCriticalError<Impl: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void, error: windows_core::HRESULT) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2EngineCallback_Impl::OnCriticalError(this, core::mem::transmute_copy(&error))
        }
        Self { OnProcessingPassStart: OnProcessingPassStart::<Impl>, OnProcessingPassEnd: OnProcessingPassEnd::<Impl>, OnCriticalError: OnCriticalError::<Impl> }
    }
}
#[doc(hidden)]
struct IXAudio2EngineCallback_ImplVtbl<T: IXAudio2EngineCallback_Impl>(std::marker::PhantomData<T>);
impl<T: IXAudio2EngineCallback_Impl> IXAudio2EngineCallback_ImplVtbl<T> {
    const VTABLE: IXAudio2EngineCallback_Vtbl = IXAudio2EngineCallback_Vtbl::new::<T>();
}
impl IXAudio2EngineCallback {
    pub fn new<'a, T: IXAudio2EngineCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2EngineCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IXAudio2Extension_Impl: Sized {
    fn GetProcessingQuantum(&self, quantumnumerator: *mut u32, quantumdenominator: *mut u32);
    fn GetProcessor(&self, processor: *mut u32);
}
impl windows_core::RuntimeName for IXAudio2Extension {}
impl IXAudio2Extension_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2Extension_Impl, const OFFSET: isize>() -> IXAudio2Extension_Vtbl {
        unsafe extern "system" fn GetProcessingQuantum<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2Extension_Impl::GetProcessingQuantum(this, core::mem::transmute_copy(&quantumnumerator), core::mem::transmute_copy(&quantumdenominator))
        }
        unsafe extern "system" fn GetProcessor<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processor: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IXAudio2Extension_Impl::GetProcessor(this, core::mem::transmute_copy(&processor))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProcessingQuantum: GetProcessingQuantum::<Identity, Impl, OFFSET>,
            GetProcessor: GetProcessor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAudio2Extension as windows_core::Interface>::IID
    }
}
pub trait IXAudio2MasteringVoice_Impl: Sized + IXAudio2Voice_Impl {
    fn GetChannelMask(&self) -> windows_core::Result<u32>;
}
impl IXAudio2MasteringVoice_Vtbl {
    pub const fn new<Impl: IXAudio2MasteringVoice_Impl>() -> IXAudio2MasteringVoice_Vtbl {
        unsafe extern "system" fn GetChannelMask<Impl: IXAudio2MasteringVoice_Impl>(this: *mut core::ffi::c_void, pchannelmask: *mut u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            match IXAudio2MasteringVoice_Impl::GetChannelMask(this) {
                Ok(ok__) => {
                    core::ptr::write(pchannelmask, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IXAudio2Voice_Vtbl::new::<Impl>(), GetChannelMask: GetChannelMask::<Impl> }
    }
}
#[doc(hidden)]
struct IXAudio2MasteringVoice_ImplVtbl<T: IXAudio2MasteringVoice_Impl>(std::marker::PhantomData<T>);
impl<T: IXAudio2MasteringVoice_Impl> IXAudio2MasteringVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2MasteringVoice_Vtbl = IXAudio2MasteringVoice_Vtbl::new::<T>();
}
impl IXAudio2MasteringVoice {
    pub fn new<'a, T: IXAudio2MasteringVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2MasteringVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IXAudio2SourceVoice_Impl: Sized + IXAudio2Voice_Impl {
    fn Start(&self, flags: u32, operationset: u32) -> windows_core::Result<()>;
    fn Stop(&self, flags: u32, operationset: u32) -> windows_core::Result<()>;
    fn SubmitSourceBuffer(&self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> windows_core::Result<()>;
    fn FlushSourceBuffers(&self) -> windows_core::Result<()>;
    fn Discontinuity(&self) -> windows_core::Result<()>;
    fn ExitLoop(&self, operationset: u32) -> windows_core::Result<()>;
    fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32);
    fn SetFrequencyRatio(&self, ratio: f32, operationset: u32) -> windows_core::Result<()>;
    fn GetFrequencyRatio(&self, pratio: *mut f32);
    fn SetSourceSampleRate(&self, newsourcesamplerate: u32) -> windows_core::Result<()>;
}
impl IXAudio2SourceVoice_Vtbl {
    pub const fn new<Impl: IXAudio2SourceVoice_Impl>() -> IXAudio2SourceVoice_Vtbl {
        unsafe extern "system" fn Start<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, flags: u32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::Start(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn Stop<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, flags: u32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::Stop(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn SubmitSourceBuffer<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::SubmitSourceBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pbufferwma)).into()
        }
        unsafe extern "system" fn FlushSourceBuffers<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::FlushSourceBuffers(this).into()
        }
        unsafe extern "system" fn Discontinuity<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::Discontinuity(this).into()
        }
        unsafe extern "system" fn ExitLoop<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::ExitLoop(this, core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetState<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::GetState(this, core::mem::transmute_copy(&pvoicestate), core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetFrequencyRatio<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, ratio: f32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::SetFrequencyRatio(this, core::mem::transmute_copy(&ratio), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetFrequencyRatio<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pratio: *mut f32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::GetFrequencyRatio(this, core::mem::transmute_copy(&pratio))
        }
        unsafe extern "system" fn SetSourceSampleRate<Impl: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, newsourcesamplerate: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2SourceVoice_Impl::SetSourceSampleRate(this, core::mem::transmute_copy(&newsourcesamplerate)).into()
        }
        Self {
            base__: IXAudio2Voice_Vtbl::new::<Impl>(),
            Start: Start::<Impl>,
            Stop: Stop::<Impl>,
            SubmitSourceBuffer: SubmitSourceBuffer::<Impl>,
            FlushSourceBuffers: FlushSourceBuffers::<Impl>,
            Discontinuity: Discontinuity::<Impl>,
            ExitLoop: ExitLoop::<Impl>,
            GetState: GetState::<Impl>,
            SetFrequencyRatio: SetFrequencyRatio::<Impl>,
            GetFrequencyRatio: GetFrequencyRatio::<Impl>,
            SetSourceSampleRate: SetSourceSampleRate::<Impl>,
        }
    }
}
#[doc(hidden)]
struct IXAudio2SourceVoice_ImplVtbl<T: IXAudio2SourceVoice_Impl>(std::marker::PhantomData<T>);
impl<T: IXAudio2SourceVoice_Impl> IXAudio2SourceVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2SourceVoice_Vtbl = IXAudio2SourceVoice_Vtbl::new::<T>();
}
impl IXAudio2SourceVoice {
    pub fn new<'a, T: IXAudio2SourceVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2SourceVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IXAudio2SubmixVoice_Impl: Sized + IXAudio2Voice_Impl {}
impl IXAudio2SubmixVoice_Vtbl {
    pub const fn new<Impl: IXAudio2SubmixVoice_Impl>() -> IXAudio2SubmixVoice_Vtbl {
        Self { base__: IXAudio2Voice_Vtbl::new::<Impl>() }
    }
}
#[doc(hidden)]
struct IXAudio2SubmixVoice_ImplVtbl<T: IXAudio2SubmixVoice_Impl>(std::marker::PhantomData<T>);
impl<T: IXAudio2SubmixVoice_Impl> IXAudio2SubmixVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2SubmixVoice_Vtbl = IXAudio2SubmixVoice_Vtbl::new::<T>();
}
impl IXAudio2SubmixVoice {
    pub fn new<'a, T: IXAudio2SubmixVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2SubmixVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IXAudio2Voice_Impl: Sized {
    fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS);
    fn SetOutputVoices(&self, psendlist: *const XAUDIO2_VOICE_SENDS) -> windows_core::Result<()>;
    fn SetEffectChain(&self, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn EnableEffect(&self, effectindex: u32, operationset: u32) -> windows_core::Result<()>;
    fn DisableEffect(&self, effectindex: u32, operationset: u32) -> windows_core::Result<()>;
    fn GetEffectState(&self, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL);
    fn SetEffectParameters(&self, effectindex: u32, pparameters: *const core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> windows_core::Result<()>;
    fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut core::ffi::c_void, parametersbytesize: u32) -> windows_core::Result<()>;
    fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::Result<()>;
    fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetOutputFilterParameters(&self, pdestinationvoice: Option<&IXAudio2Voice>, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::Result<()>;
    fn GetOutputFilterParameters(&self, pdestinationvoice: Option<&IXAudio2Voice>, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetVolume(&self, volume: f32, operationset: u32) -> windows_core::Result<()>;
    fn GetVolume(&self, pvolume: *mut f32);
    fn SetChannelVolumes(&self, channels: u32, pvolumes: *const f32, operationset: u32) -> windows_core::Result<()>;
    fn GetChannelVolumes(&self, channels: u32, pvolumes: *mut f32);
    fn SetOutputMatrix(&self, pdestinationvoice: Option<&IXAudio2Voice>, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> windows_core::Result<()>;
    fn GetOutputMatrix(&self, pdestinationvoice: Option<&IXAudio2Voice>, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32);
    fn DestroyVoice(&self);
}
impl IXAudio2Voice_Vtbl {
    pub const fn new<Impl: IXAudio2Voice_Impl>() -> IXAudio2Voice_Vtbl {
        unsafe extern "system" fn GetVoiceDetails<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetVoiceDetails(this, core::mem::transmute_copy(&pvoicedetails))
        }
        unsafe extern "system" fn SetOutputVoices<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetOutputVoices(this, core::mem::transmute_copy(&psendlist)).into()
        }
        unsafe extern "system" fn SetEffectChain<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetEffectChain(this, core::mem::transmute_copy(&peffectchain)).into()
        }
        unsafe extern "system" fn EnableEffect<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::EnableEffect(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn DisableEffect<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::DisableEffect(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetEffectState<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, penabled: *mut super::super::super::Foundation::BOOL) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetEffectState(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&penabled))
        }
        unsafe extern "system" fn SetEffectParameters<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, pparameters: *const core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetEffectParameters(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parametersbytesize), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetEffectParameters<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, pparameters: *mut core::ffi::c_void, parametersbytesize: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetEffectParameters(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parametersbytesize)).into()
        }
        unsafe extern "system" fn SetFilterParameters<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetFilterParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetFilterParameters<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetFilterParameters(this, core::mem::transmute_copy(&pparameters))
        }
        unsafe extern "system" fn SetOutputFilterParameters<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetOutputFilterParameters(this, windows_core::from_raw_borrowed(&pdestinationvoice), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetOutputFilterParameters<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetOutputFilterParameters(this, windows_core::from_raw_borrowed(&pdestinationvoice), core::mem::transmute_copy(&pparameters))
        }
        unsafe extern "system" fn SetVolume<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, volume: f32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetVolume(this, core::mem::transmute_copy(&volume), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetVolume<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pvolume: *mut f32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetVolume(this, core::mem::transmute_copy(&pvolume))
        }
        unsafe extern "system" fn SetChannelVolumes<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, channels: u32, pvolumes: *const f32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetChannelVolumes(this, core::mem::transmute_copy(&channels), core::mem::transmute_copy(&pvolumes), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetChannelVolumes<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, channels: u32, pvolumes: *mut f32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetChannelVolumes(this, core::mem::transmute_copy(&channels), core::mem::transmute_copy(&pvolumes))
        }
        unsafe extern "system" fn SetOutputMatrix<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::SetOutputMatrix(this, windows_core::from_raw_borrowed(&pdestinationvoice), core::mem::transmute_copy(&sourcechannels), core::mem::transmute_copy(&destinationchannels), core::mem::transmute_copy(&plevelmatrix), core::mem::transmute_copy(&operationset)).into()
        }
        unsafe extern "system" fn GetOutputMatrix<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::GetOutputMatrix(this, windows_core::from_raw_borrowed(&pdestinationvoice), core::mem::transmute_copy(&sourcechannels), core::mem::transmute_copy(&destinationchannels), core::mem::transmute_copy(&plevelmatrix))
        }
        unsafe extern "system" fn DestroyVoice<Impl: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2Voice_Impl::DestroyVoice(this)
        }
        Self {
            GetVoiceDetails: GetVoiceDetails::<Impl>,
            SetOutputVoices: SetOutputVoices::<Impl>,
            SetEffectChain: SetEffectChain::<Impl>,
            EnableEffect: EnableEffect::<Impl>,
            DisableEffect: DisableEffect::<Impl>,
            GetEffectState: GetEffectState::<Impl>,
            SetEffectParameters: SetEffectParameters::<Impl>,
            GetEffectParameters: GetEffectParameters::<Impl>,
            SetFilterParameters: SetFilterParameters::<Impl>,
            GetFilterParameters: GetFilterParameters::<Impl>,
            SetOutputFilterParameters: SetOutputFilterParameters::<Impl>,
            GetOutputFilterParameters: GetOutputFilterParameters::<Impl>,
            SetVolume: SetVolume::<Impl>,
            GetVolume: GetVolume::<Impl>,
            SetChannelVolumes: SetChannelVolumes::<Impl>,
            GetChannelVolumes: GetChannelVolumes::<Impl>,
            SetOutputMatrix: SetOutputMatrix::<Impl>,
            GetOutputMatrix: GetOutputMatrix::<Impl>,
            DestroyVoice: DestroyVoice::<Impl>,
        }
    }
}
#[doc(hidden)]
struct IXAudio2Voice_ImplVtbl<T: IXAudio2Voice_Impl>(std::marker::PhantomData<T>);
impl<T: IXAudio2Voice_Impl> IXAudio2Voice_ImplVtbl<T> {
    const VTABLE: IXAudio2Voice_Vtbl = IXAudio2Voice_Vtbl::new::<T>();
}
impl IXAudio2Voice {
    pub fn new<'a, T: IXAudio2Voice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2Voice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IXAudio2VoiceCallback_Impl: Sized {
    fn OnVoiceProcessingPassStart(&self, bytesrequired: u32);
    fn OnVoiceProcessingPassEnd(&self);
    fn OnStreamEnd(&self);
    fn OnBufferStart(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnBufferEnd(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnLoopEnd(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnVoiceError(&self, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT);
}
impl IXAudio2VoiceCallback_Vtbl {
    pub const fn new<Impl: IXAudio2VoiceCallback_Impl>() -> IXAudio2VoiceCallback_Vtbl {
        unsafe extern "system" fn OnVoiceProcessingPassStart<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, bytesrequired: u32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnVoiceProcessingPassStart(this, core::mem::transmute_copy(&bytesrequired))
        }
        unsafe extern "system" fn OnVoiceProcessingPassEnd<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnVoiceProcessingPassEnd(this)
        }
        unsafe extern "system" fn OnStreamEnd<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnStreamEnd(this)
        }
        unsafe extern "system" fn OnBufferStart<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnBufferStart(this, core::mem::transmute_copy(&pbuffercontext))
        }
        unsafe extern "system" fn OnBufferEnd<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnBufferEnd(this, core::mem::transmute_copy(&pbuffercontext))
        }
        unsafe extern "system" fn OnLoopEnd<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnLoopEnd(this, core::mem::transmute_copy(&pbuffercontext))
        }
        unsafe extern "system" fn OnVoiceError<Impl: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IXAudio2VoiceCallback_Impl::OnVoiceError(this, core::mem::transmute_copy(&pbuffercontext), core::mem::transmute_copy(&error))
        }
        Self {
            OnVoiceProcessingPassStart: OnVoiceProcessingPassStart::<Impl>,
            OnVoiceProcessingPassEnd: OnVoiceProcessingPassEnd::<Impl>,
            OnStreamEnd: OnStreamEnd::<Impl>,
            OnBufferStart: OnBufferStart::<Impl>,
            OnBufferEnd: OnBufferEnd::<Impl>,
            OnLoopEnd: OnLoopEnd::<Impl>,
            OnVoiceError: OnVoiceError::<Impl>,
        }
    }
}
#[doc(hidden)]
struct IXAudio2VoiceCallback_ImplVtbl<T: IXAudio2VoiceCallback_Impl>(std::marker::PhantomData<T>);
impl<T: IXAudio2VoiceCallback_Impl> IXAudio2VoiceCallback_ImplVtbl<T> {
    const VTABLE: IXAudio2VoiceCallback_Vtbl = IXAudio2VoiceCallback_Vtbl::new::<T>();
}
impl IXAudio2VoiceCallback {
    pub fn new<'a, T: IXAudio2VoiceCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2VoiceCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
