#[inline]
pub unsafe fn CreateAudioReverb() -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("xaudio2_8.dll" "system" fn CreateAudioReverb(ppapo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAudioReverb(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateAudioVolumeMeter() -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("xaudio2_8.dll" "system" fn CreateAudioVolumeMeter(ppapo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAudioVolumeMeter(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateFX(clsid: *const windows_core::GUID, peffect: *mut Option<windows_core::IUnknown>, pinitdat: Option<*const core::ffi::c_void>, initdatabytesize: u32) -> windows_core::Result<()> {
    windows_core::link!("xaudio2_8.dll" "C" fn CreateFX(clsid : *const windows_core::GUID, peffect : *mut * mut core::ffi::c_void, pinitdat : *const core::ffi::c_void, initdatabytesize : u32) -> windows_core::HRESULT);
    unsafe { CreateFX(clsid, core::mem::transmute(peffect), pinitdat.unwrap_or(core::mem::zeroed()) as _, initdatabytesize).ok() }
}
#[inline]
pub unsafe fn CreateHrtfApo(init: *const HrtfApoInit) -> windows_core::Result<IXAPO> {
    windows_core::link!("hrtfapo.dll" "system" fn CreateHrtfApo(init : *const HrtfApoInit, xapo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateHrtfApo(init, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn XAudio2CreateWithVersionInfo(ppxaudio2: *mut Option<IXAudio2>, flags: u32, xaudio2processor: u32, ntddiversion: u32) -> windows_core::Result<()> {
    windows_core::link!("xaudio2_8.dll" "system" fn XAudio2CreateWithVersionInfo(ppxaudio2 : *mut * mut core::ffi::c_void, flags : u32, xaudio2processor : u32, ntddiversion : u32) -> windows_core::HRESULT);
    unsafe { XAudio2CreateWithVersionInfo(core::mem::transmute(ppxaudio2), flags, xaudio2processor, ntddiversion).ok() }
}
pub const AudioReverb: windows_core::GUID = windows_core::GUID::from_u128(0xc2633b16_471b_4498_b8c5_4f0959e2ec09);
pub const AudioVolumeMeter: windows_core::GUID = windows_core::GUID::from_u128(0x4fc3b166_972a_40cf_bc37_7db03db2fba3);
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(1i32);
pub const Cardioid: HrtfDirectivityType = HrtfDirectivityType(1i32);
pub const Cone: HrtfDirectivityType = HrtfDirectivityType(2i32);
pub const CustomDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(1i32);
pub const FACILITY_XAPO: u32 = 2199u32;
pub const FACILITY_XAUDIO2: u32 = 2198u32;
pub const FXECHO_DEFAULT_DELAY: f32 = 500f32;
pub const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5f32;
pub const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5f32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct FXECHO_INITDATA {
    pub MaxDelay: f32,
}
pub const FXECHO_MAX_DELAY: f32 = 2000f32;
pub const FXECHO_MAX_FEEDBACK: f32 = 1f32;
pub const FXECHO_MAX_WETDRYMIX: f32 = 1f32;
pub const FXECHO_MIN_DELAY: f32 = 1f32;
pub const FXECHO_MIN_FEEDBACK: f32 = 0f32;
pub const FXECHO_MIN_WETDRYMIX: f32 = 0f32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct FXECHO_PARAMETERS {
    pub WetDryMix: f32,
    pub Feedback: f32,
    pub Delay: f32,
}
pub const FXEQ: windows_core::GUID = windows_core::GUID::from_u128(0xf5e01117_d6c4_485a_a3f5_695196f3dbfa);
pub const FXEQ_DEFAULT_BANDWIDTH: f32 = 1f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_0: f32 = 100f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_1: f32 = 800f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_2: f32 = 2000f32;
pub const FXEQ_DEFAULT_FREQUENCY_CENTER_3: f32 = 10000f32;
pub const FXEQ_DEFAULT_GAIN: f32 = 1f32;
pub const FXEQ_MAX_BANDWIDTH: f32 = 2f32;
pub const FXEQ_MAX_FRAMERATE: u32 = 48000u32;
pub const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20000f32;
pub const FXEQ_MAX_GAIN: f32 = 7.94f32;
pub const FXEQ_MIN_BANDWIDTH: f32 = 0.1f32;
pub const FXEQ_MIN_FRAMERATE: u32 = 22000u32;
pub const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20f32;
pub const FXEQ_MIN_GAIN: f32 = 0.126f32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct FXEQ_PARAMETERS {
    pub FrequencyCenter0: f32,
    pub Gain0: f32,
    pub Bandwidth0: f32,
    pub FrequencyCenter1: f32,
    pub Gain1: f32,
    pub Bandwidth1: f32,
    pub FrequencyCenter2: f32,
    pub Gain2: f32,
    pub Bandwidth2: f32,
    pub FrequencyCenter3: f32,
    pub Gain3: f32,
    pub Bandwidth3: f32,
}
pub const FXEcho: windows_core::GUID = windows_core::GUID::from_u128(0x5039d740_f736_449a_84d3_a56202557b87);
pub const FXLOUDNESS_DEFAULT_MOMENTARY_MS: u32 = 400u32;
pub const FXLOUDNESS_DEFAULT_SHORTTERM_MS: u32 = 3000u32;
pub const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000u32;
pub const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6u32;
pub const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800u32;
pub const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20u32;
pub const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1u32;
pub const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct FXMASTERINGLIMITER_PARAMETERS {
    pub Release: u32,
    pub Loudness: u32,
}
pub const FXMasteringLimiter: windows_core::GUID = windows_core::GUID::from_u128(0xc4137916_2be1_46fd_8599_441536f49856);
pub const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9f32;
pub const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6f32;
pub const FXREVERB_MAX_DIFFUSION: f32 = 1f32;
pub const FXREVERB_MAX_ROOMSIZE: f32 = 1f32;
pub const FXREVERB_MIN_DIFFUSION: f32 = 0f32;
pub const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001f32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct FXREVERB_PARAMETERS {
    pub Diffusion: f32,
    pub RoomSize: f32,
}
pub const FXReverb: windows_core::GUID = windows_core::GUID::from_u128(0x7d9aca56_cb68_4807_b632_b137352e8596);
pub const HRTF_DEFAULT_UNITY_GAIN_DISTANCE: f32 = 1f32;
pub const HRTF_MAX_GAIN_LIMIT: f32 = 12f32;
pub const HRTF_MIN_GAIN_LIMIT: f32 = -96f32;
pub const HRTF_MIN_UNITY_GAIN_DISTANCE: f32 = 0.05f32;
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(2i32);
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HrtfApoInit {
    pub distanceDecay: *mut HrtfDistanceDecay,
    pub directivity: *mut HrtfDirectivity,
}
impl Default for HrtfApoInit {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HrtfDirectivity {
    pub r#type: HrtfDirectivityType,
    pub scaling: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HrtfDirectivityCardioid {
    pub directivity: HrtfDirectivity,
    pub order: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HrtfDirectivityCone {
    pub directivity: HrtfDirectivity,
    pub innerAngle: f32,
    pub outerAngle: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HrtfDirectivityType(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HrtfDistanceDecay {
    pub r#type: HrtfDistanceDecayType,
    pub maxGain: f32,
    pub minGain: f32,
    pub unityGainDistance: f32,
    pub cutoffDistance: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HrtfDistanceDecayType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HrtfEnvironment(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HrtfOrientation {
    pub element: [f32; 9],
}
impl Default for HrtfOrientation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HrtfPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
windows_core::imp::define_interface!(IXAPO, IXAPO_Vtbl, 0xa410b984_9839_4819_a0be_2856ae6b3adb);
windows_core::imp::interface_hierarchy!(IXAPO, windows_core::IUnknown);
impl IXAPO {
    pub unsafe fn GetRegistrationProperties(&self) -> windows_core::Result<*mut XAPO_REGISTRATION_PROPERTIES> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegistrationProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsInputFormatSupported(&self, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: Option<*mut *mut super::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsInputFormatSupported)(windows_core::Interface::as_raw(self), poutputformat, prequestedinputformat, ppsupportedinputformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn IsOutputFormatSupported(&self, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: Option<*mut *mut super::WAVEFORMATEX>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsOutputFormatSupported)(windows_core::Interface::as_raw(self), pinputformat, prequestedoutputformat, ppsupportedoutputformat.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Initialize(&self, pdata: Option<*const core::ffi::c_void>, databytesize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pdata.unwrap_or(core::mem::zeroed()) as _, databytesize).ok() }
    }
    pub unsafe fn Reset(&self) {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LockForProcess(&self, pinputlockedparameters: Option<&[XAPO_LOCKFORPROCESS_PARAMETERS]>, poutputlockedparameters: Option<&[XAPO_LOCKFORPROCESS_PARAMETERS]>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockForProcess)(windows_core::Interface::as_raw(self), pinputlockedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pinputlockedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), poutputlockedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(poutputlockedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
    }
    pub unsafe fn UnlockForProcess(&self) {
        unsafe { (windows_core::Interface::vtable(self).UnlockForProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Process(&self, pinputprocessparameters: Option<&[XAPO_PROCESS_BUFFER_PARAMETERS]>, poutputprocessparameters: Option<&mut [XAPO_PROCESS_BUFFER_PARAMETERS]>, isenabled: bool) {
        unsafe { (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), pinputprocessparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pinputprocessparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), poutputprocessparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(poutputprocessparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), isenabled.into()) }
    }
    pub unsafe fn CalcInputFrames(&self, outputframecount: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).CalcInputFrames)(windows_core::Interface::as_raw(self), outputframecount) }
    }
    pub unsafe fn CalcOutputFrames(&self, inputframecount: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).CalcOutputFrames)(windows_core::Interface::as_raw(self), inputframecount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRegistrationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut XAPO_REGISTRATION_PROPERTIES) -> windows_core::HRESULT,
    pub IsInputFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WAVEFORMATEX, *const super::WAVEFORMATEX, *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub IsOutputFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::WAVEFORMATEX, *const super::WAVEFORMATEX, *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub LockForProcess: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XAPO_LOCKFORPROCESS_PARAMETERS, u32, *const XAPO_LOCKFORPROCESS_PARAMETERS) -> windows_core::HRESULT,
    pub UnlockForProcess: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const XAPO_PROCESS_BUFFER_PARAMETERS, u32, *mut XAPO_PROCESS_BUFFER_PARAMETERS, windows_core::BOOL),
    pub CalcInputFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub CalcOutputFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
}
pub trait IXAPO_Impl: windows_core::IUnknownImpl {
    fn GetRegistrationProperties(&self) -> windows_core::Result<*mut XAPO_REGISTRATION_PROPERTIES>;
    fn IsInputFormatSupported(&self, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn IsOutputFormatSupported(&self, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::Result<()>;
    fn Initialize(&self, pdata: *const core::ffi::c_void, databytesize: u32) -> windows_core::Result<()>;
    fn Reset(&self);
    fn LockForProcess(&self, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> windows_core::Result<()>;
    fn UnlockForProcess(&self);
    fn Process(&self, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: windows_core::BOOL);
    fn CalcInputFrames(&self, outputframecount: u32) -> u32;
    fn CalcOutputFrames(&self, inputframecount: u32) -> u32;
}
impl IXAPO_Vtbl {
    pub const fn new<Identity: IXAPO_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRegistrationProperties<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrationproperties: *mut *mut XAPO_REGISTRATION_PROPERTIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXAPO_Impl::GetRegistrationProperties(this) {
                    Ok(ok__) => {
                        ppregistrationproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputformat: *const super::WAVEFORMATEX, prequestedinputformat: *const super::WAVEFORMATEX, ppsupportedinputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::IsInputFormatSupported(this, core::mem::transmute_copy(&poutputformat), core::mem::transmute_copy(&prequestedinputformat), core::mem::transmute_copy(&ppsupportedinputformat)).into()
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputformat: *const super::WAVEFORMATEX, prequestedoutputformat: *const super::WAVEFORMATEX, ppsupportedoutputformat: *mut *mut super::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::IsOutputFormatSupported(this, core::mem::transmute_copy(&pinputformat), core::mem::transmute_copy(&prequestedoutputformat), core::mem::transmute_copy(&ppsupportedoutputformat)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const core::ffi::c_void, databytesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::Initialize(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&databytesize)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::Reset(this)
            }
        }
        unsafe extern "system" fn LockForProcess<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputlockedparametercount: u32, pinputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS, outputlockedparametercount: u32, poutputlockedparameters: *const XAPO_LOCKFORPROCESS_PARAMETERS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::LockForProcess(this, core::mem::transmute_copy(&inputlockedparametercount), core::mem::transmute_copy(&pinputlockedparameters), core::mem::transmute_copy(&outputlockedparametercount), core::mem::transmute_copy(&poutputlockedparameters)).into()
            }
        }
        unsafe extern "system" fn UnlockForProcess<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::UnlockForProcess(this)
            }
        }
        unsafe extern "system" fn Process<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputprocessparametercount: u32, pinputprocessparameters: *const XAPO_PROCESS_BUFFER_PARAMETERS, outputprocessparametercount: u32, poutputprocessparameters: *mut XAPO_PROCESS_BUFFER_PARAMETERS, isenabled: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::Process(this, core::mem::transmute_copy(&inputprocessparametercount), core::mem::transmute_copy(&pinputprocessparameters), core::mem::transmute_copy(&outputprocessparametercount), core::mem::transmute_copy(&poutputprocessparameters), core::mem::transmute_copy(&isenabled))
            }
        }
        unsafe extern "system" fn CalcInputFrames<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputframecount: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::CalcInputFrames(this, core::mem::transmute_copy(&outputframecount))
            }
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: IXAPO_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputframecount: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPO_Impl::CalcOutputFrames(this, core::mem::transmute_copy(&inputframecount))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRegistrationProperties: GetRegistrationProperties::<Identity, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            LockForProcess: LockForProcess::<Identity, OFFSET>,
            UnlockForProcess: UnlockForProcess::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
            CalcInputFrames: CalcInputFrames::<Identity, OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAPO as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAPO {}
windows_core::imp::define_interface!(IXAPOHrtfParameters, IXAPOHrtfParameters_Vtbl, 0x15b3cd66_e9de_4464_b6e6_2bc3cf63d455);
windows_core::imp::interface_hierarchy!(IXAPOHrtfParameters, windows_core::IUnknown);
impl IXAPOHrtfParameters {
    pub unsafe fn SetSourcePosition(&self, position: *const HrtfPosition) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourcePosition)(windows_core::Interface::as_raw(self), position).ok() }
    }
    pub unsafe fn SetSourceOrientation(&self, orientation: *const HrtfOrientation) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceOrientation)(windows_core::Interface::as_raw(self), orientation).ok() }
    }
    pub unsafe fn SetSourceGain(&self, gain: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceGain)(windows_core::Interface::as_raw(self), gain).ok() }
    }
    pub unsafe fn SetEnvironment(&self, environment: HrtfEnvironment) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironment)(windows_core::Interface::as_raw(self), environment).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPOHrtfParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSourcePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const HrtfPosition) -> windows_core::HRESULT,
    pub SetSourceOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *const HrtfOrientation) -> windows_core::HRESULT,
    pub SetSourceGain: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, HrtfEnvironment) -> windows_core::HRESULT,
}
pub trait IXAPOHrtfParameters_Impl: windows_core::IUnknownImpl {
    fn SetSourcePosition(&self, position: *const HrtfPosition) -> windows_core::Result<()>;
    fn SetSourceOrientation(&self, orientation: *const HrtfOrientation) -> windows_core::Result<()>;
    fn SetSourceGain(&self, gain: f32) -> windows_core::Result<()>;
    fn SetEnvironment(&self, environment: HrtfEnvironment) -> windows_core::Result<()>;
}
impl IXAPOHrtfParameters_Vtbl {
    pub const fn new<Identity: IXAPOHrtfParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourcePosition<Identity: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: *const HrtfPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPOHrtfParameters_Impl::SetSourcePosition(this, core::mem::transmute_copy(&position)).into()
            }
        }
        unsafe extern "system" fn SetSourceOrientation<Identity: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, orientation: *const HrtfOrientation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPOHrtfParameters_Impl::SetSourceOrientation(this, core::mem::transmute_copy(&orientation)).into()
            }
        }
        unsafe extern "system" fn SetSourceGain<Identity: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gain: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPOHrtfParameters_Impl::SetSourceGain(this, core::mem::transmute_copy(&gain)).into()
            }
        }
        unsafe extern "system" fn SetEnvironment<Identity: IXAPOHrtfParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: HrtfEnvironment) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPOHrtfParameters_Impl::SetEnvironment(this, core::mem::transmute_copy(&environment)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSourcePosition: SetSourcePosition::<Identity, OFFSET>,
            SetSourceOrientation: SetSourceOrientation::<Identity, OFFSET>,
            SetSourceGain: SetSourceGain::<Identity, OFFSET>,
            SetEnvironment: SetEnvironment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAPOHrtfParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAPOHrtfParameters {}
windows_core::imp::define_interface!(IXAPOParameters, IXAPOParameters_Vtbl, 0x26d95c66_80f2_499a_ad54_5ae7f01c6d98);
windows_core::imp::interface_hierarchy!(IXAPOParameters, windows_core::IUnknown);
impl IXAPOParameters {
    pub unsafe fn SetParameters(&self, pparameters: *const core::ffi::c_void, parameterbytesize: u32) {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), pparameters, parameterbytesize) }
    }
    pub unsafe fn GetParameters(&self, pparameters: *mut core::ffi::c_void, parameterbytesize: u32) {
        unsafe { (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), pparameters as _, parameterbytesize) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAPOParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32),
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
}
pub trait IXAPOParameters_Impl: windows_core::IUnknownImpl {
    fn SetParameters(&self, pparameters: *const core::ffi::c_void, parameterbytesize: u32);
    fn GetParameters(&self, pparameters: *mut core::ffi::c_void, parameterbytesize: u32);
}
impl IXAPOParameters_Vtbl {
    pub const fn new<Identity: IXAPOParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetParameters<Identity: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *const core::ffi::c_void, parameterbytesize: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPOParameters_Impl::SetParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parameterbytesize))
            }
        }
        unsafe extern "system" fn GetParameters<Identity: IXAPOParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparameters: *mut core::ffi::c_void, parameterbytesize: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAPOParameters_Impl::GetParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parameterbytesize))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetParameters: SetParameters::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAPOParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAPOParameters {}
windows_core::imp::define_interface!(IXAudio2, IXAudio2_Vtbl, 0x2b02e3cf_2e0b_4ec3_be45_1b2a3fe7210d);
windows_core::imp::interface_hierarchy!(IXAudio2, windows_core::IUnknown);
impl IXAudio2 {
    pub unsafe fn RegisterForCallbacks<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXAudio2EngineCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterForCallbacks)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn UnregisterForCallbacks<P0>(&self, pcallback: P0)
    where
        P0: windows_core::Param<IXAudio2EngineCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterForCallbacks)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    pub unsafe fn CreateSourceVoice<P4>(&self, ppsourcevoice: *mut Option<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: P4, psendlist: Option<*const XAUDIO2_VOICE_SENDS>, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>) -> windows_core::Result<()>
    where
        P4: windows_core::Param<IXAudio2VoiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSourceVoice)(windows_core::Interface::as_raw(self), core::mem::transmute(ppsourcevoice), psourceformat, flags, maxfrequencyratio, pcallback.param().abi(), psendlist.unwrap_or(core::mem::zeroed()) as _, peffectchain.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateSubmixVoice(&self, ppsubmixvoice: *mut Option<IXAudio2SubmixVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: Option<*const XAUDIO2_VOICE_SENDS>, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateSubmixVoice)(windows_core::Interface::as_raw(self), core::mem::transmute(ppsubmixvoice), inputchannels, inputsamplerate, flags, processingstage, psendlist.unwrap_or(core::mem::zeroed()) as _, peffectchain.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn CreateMasteringVoice<P4>(&self, ppmasteringvoice: *mut Option<IXAudio2MasteringVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: P4, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::Result<()>
    where
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMasteringVoice)(windows_core::Interface::as_raw(self), core::mem::transmute(ppmasteringvoice), inputchannels, inputsamplerate, flags, szdeviceid.param().abi(), peffectchain.unwrap_or(core::mem::zeroed()) as _, streamcategory).ok() }
    }
    pub unsafe fn StartEngine(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartEngine)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StopEngine(&self) {
        unsafe { (windows_core::Interface::vtable(self).StopEngine)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CommitChanges(&self, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommitChanges)(windows_core::Interface::as_raw(self), operationset).ok() }
    }
    pub unsafe fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
        unsafe { (windows_core::Interface::vtable(self).GetPerformanceData)(windows_core::Interface::as_raw(self), pperfdata as _) }
    }
    pub unsafe fn SetDebugConfiguration(&self, pdebugconfiguration: Option<*const XAUDIO2_DEBUG_CONFIGURATION>, preserved: Option<*const core::ffi::c_void>) {
        unsafe { (windows_core::Interface::vtable(self).SetDebugConfiguration)(windows_core::Interface::as_raw(self), pdebugconfiguration.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterForCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterForCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub CreateSourceVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *const super::WAVEFORMATEX, u32, f32, *mut core::ffi::c_void, *const XAUDIO2_VOICE_SENDS, *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT,
    pub CreateSubmixVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32, u32, u32, *const XAUDIO2_VOICE_SENDS, *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT,
    pub CreateMasteringVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, u32, u32, windows_core::PCWSTR, *const XAUDIO2_EFFECT_CHAIN, super::AUDIO_STREAM_CATEGORY) -> windows_core::HRESULT,
    pub StartEngine: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopEngine: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub CommitChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPerformanceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_PERFORMANCE_DATA),
    pub SetDebugConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_DEBUG_CONFIGURATION, *const core::ffi::c_void),
}
pub trait IXAudio2_Impl: windows_core::IUnknownImpl {
    fn RegisterForCallbacks(&self, pcallback: windows_core::Ref<IXAudio2EngineCallback>) -> windows_core::Result<()>;
    fn UnregisterForCallbacks(&self, pcallback: windows_core::Ref<IXAudio2EngineCallback>);
    fn CreateSourceVoice(&self, ppsourcevoice: windows_core::OutRef<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: windows_core::Ref<IXAudio2VoiceCallback>, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn CreateSubmixVoice(&self, ppsubmixvoice: windows_core::OutRef<IXAudio2SubmixVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn CreateMasteringVoice(&self, ppmasteringvoice: windows_core::OutRef<IXAudio2MasteringVoice>, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: &windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::Result<()>;
    fn StartEngine(&self) -> windows_core::Result<()>;
    fn StopEngine(&self);
    fn CommitChanges(&self, operationset: u32) -> windows_core::Result<()>;
    fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA);
    fn SetDebugConfiguration(&self, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *const core::ffi::c_void);
}
impl IXAudio2_Vtbl {
    pub const fn new<Identity: IXAudio2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterForCallbacks<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::RegisterForCallbacks(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn UnregisterForCallbacks<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::UnregisterForCallbacks(this, core::mem::transmute_copy(&pcallback))
            }
        }
        unsafe extern "system" fn CreateSourceVoice<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsourcevoice: *mut *mut core::ffi::c_void, psourceformat: *const super::WAVEFORMATEX, flags: u32, maxfrequencyratio: f32, pcallback: *mut core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CreateSourceVoice(this, core::mem::transmute_copy(&ppsourcevoice), core::mem::transmute_copy(&psourceformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&maxfrequencyratio), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&psendlist), core::mem::transmute_copy(&peffectchain)).into()
            }
        }
        unsafe extern "system" fn CreateSubmixVoice<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubmixvoice: *mut *mut core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, processingstage: u32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CreateSubmixVoice(this, core::mem::transmute_copy(&ppsubmixvoice), core::mem::transmute_copy(&inputchannels), core::mem::transmute_copy(&inputsamplerate), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&processingstage), core::mem::transmute_copy(&psendlist), core::mem::transmute_copy(&peffectchain)).into()
            }
        }
        unsafe extern "system" fn CreateMasteringVoice<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmasteringvoice: *mut *mut core::ffi::c_void, inputchannels: u32, inputsamplerate: u32, flags: u32, szdeviceid: windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CreateMasteringVoice(this, core::mem::transmute_copy(&ppmasteringvoice), core::mem::transmute_copy(&inputchannels), core::mem::transmute_copy(&inputsamplerate), core::mem::transmute_copy(&flags), core::mem::transmute(&szdeviceid), core::mem::transmute_copy(&peffectchain), core::mem::transmute_copy(&streamcategory)).into()
            }
        }
        unsafe extern "system" fn StartEngine<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::StartEngine(this).into()
            }
        }
        unsafe extern "system" fn StopEngine<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::StopEngine(this)
            }
        }
        unsafe extern "system" fn CommitChanges<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CommitChanges(this, core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetPerformanceData<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::GetPerformanceData(this, core::mem::transmute_copy(&pperfdata))
            }
        }
        unsafe extern "system" fn SetDebugConfiguration<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *const core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::SetDebugConfiguration(this, core::mem::transmute_copy(&pdebugconfiguration), core::mem::transmute_copy(&preserved))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForCallbacks: RegisterForCallbacks::<Identity, OFFSET>,
            UnregisterForCallbacks: UnregisterForCallbacks::<Identity, OFFSET>,
            CreateSourceVoice: CreateSourceVoice::<Identity, OFFSET>,
            CreateSubmixVoice: CreateSubmixVoice::<Identity, OFFSET>,
            CreateMasteringVoice: CreateMasteringVoice::<Identity, OFFSET>,
            StartEngine: StartEngine::<Identity, OFFSET>,
            StopEngine: StopEngine::<Identity, OFFSET>,
            CommitChanges: CommitChanges::<Identity, OFFSET>,
            GetPerformanceData: GetPerformanceData::<Identity, OFFSET>,
            SetDebugConfiguration: SetDebugConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAudio2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAudio2 {}
windows_core::imp::define_interface!(IXAudio2EngineCallback, IXAudio2EngineCallback_Vtbl);
impl IXAudio2EngineCallback {
    pub unsafe fn OnProcessingPassStart(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnProcessingPassStart)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnProcessingPassEnd(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnProcessingPassEnd)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnCriticalError(&self, error: windows_core::HRESULT) {
        unsafe { (windows_core::Interface::vtable(self).OnCriticalError)(windows_core::Interface::as_raw(self), error) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2EngineCallback_Vtbl {
    pub OnProcessingPassStart: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnProcessingPassEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnCriticalError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
}
pub trait IXAudio2EngineCallback_Impl {
    fn OnProcessingPassStart(&self);
    fn OnProcessingPassEnd(&self);
    fn OnCriticalError(&self, error: windows_core::HRESULT);
}
impl IXAudio2EngineCallback_Vtbl {
    pub const fn new<Identity: IXAudio2EngineCallback_Impl>() -> Self {
        unsafe extern "system" fn OnProcessingPassStart<Identity: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2EngineCallback_Impl::OnProcessingPassStart(this)
            }
        }
        unsafe extern "system" fn OnProcessingPassEnd<Identity: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2EngineCallback_Impl::OnProcessingPassEnd(this)
            }
        }
        unsafe extern "system" fn OnCriticalError<Identity: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void, error: windows_core::HRESULT) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2EngineCallback_Impl::OnCriticalError(this, core::mem::transmute_copy(&error))
            }
        }
        Self {
            OnProcessingPassStart: OnProcessingPassStart::<Identity>,
            OnProcessingPassEnd: OnProcessingPassEnd::<Identity>,
            OnCriticalError: OnCriticalError::<Identity>,
        }
    }
}
struct IXAudio2EngineCallback_ImplVtbl<T: IXAudio2EngineCallback_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2EngineCallback_Impl> IXAudio2EngineCallback_ImplVtbl<T> {
    const VTABLE: IXAudio2EngineCallback_Vtbl = IXAudio2EngineCallback_Vtbl::new::<T>();
}
impl IXAudio2EngineCallback {
    pub fn new<'a, T: IXAudio2EngineCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2EngineCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IXAudio2Extension, IXAudio2Extension_Vtbl, 0x84ac29bb_d619_44d2_b197_e4acf7df3ed6);
windows_core::imp::interface_hierarchy!(IXAudio2Extension, windows_core::IUnknown);
impl IXAudio2Extension {
    pub unsafe fn GetProcessingQuantum(&self, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
        unsafe { (windows_core::Interface::vtable(self).GetProcessingQuantum)(windows_core::Interface::as_raw(self), quantumnumerator as _, quantumdenominator as _) }
    }
    pub unsafe fn GetProcessor(&self, processor: *mut u32) {
        unsafe { (windows_core::Interface::vtable(self).GetProcessor)(windows_core::Interface::as_raw(self), processor as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Extension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProcessingQuantum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32),
    pub GetProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32),
}
pub trait IXAudio2Extension_Impl: windows_core::IUnknownImpl {
    fn GetProcessingQuantum(&self, quantumnumerator: *mut u32, quantumdenominator: *mut u32);
    fn GetProcessor(&self, processor: *mut u32);
}
impl IXAudio2Extension_Vtbl {
    pub const fn new<Identity: IXAudio2Extension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProcessingQuantum<Identity: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quantumnumerator: *mut u32, quantumdenominator: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2Extension_Impl::GetProcessingQuantum(this, core::mem::transmute_copy(&quantumnumerator), core::mem::transmute_copy(&quantumdenominator))
            }
        }
        unsafe extern "system" fn GetProcessor<Identity: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processor: *mut u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2Extension_Impl::GetProcessor(this, core::mem::transmute_copy(&processor))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProcessingQuantum: GetProcessingQuantum::<Identity, OFFSET>,
            GetProcessor: GetProcessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAudio2Extension as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IXAudio2Extension {}
windows_core::imp::define_interface!(IXAudio2MasteringVoice, IXAudio2MasteringVoice_Vtbl);
impl core::ops::Deref for IXAudio2MasteringVoice {
    type Target = IXAudio2Voice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAudio2MasteringVoice, IXAudio2Voice);
impl IXAudio2MasteringVoice {
    pub unsafe fn GetChannelMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2MasteringVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
    pub GetChannelMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IXAudio2MasteringVoice_Impl: IXAudio2Voice_Impl {
    fn GetChannelMask(&self) -> windows_core::Result<u32>;
}
impl IXAudio2MasteringVoice_Vtbl {
    pub const fn new<Identity: IXAudio2MasteringVoice_Impl>() -> Self {
        unsafe extern "system" fn GetChannelMask<Identity: IXAudio2MasteringVoice_Impl>(this: *mut core::ffi::c_void, pchannelmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IXAudio2MasteringVoice_Impl::GetChannelMask(this) {
                    Ok(ok__) => {
                        pchannelmask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IXAudio2Voice_Vtbl::new::<Identity>(), GetChannelMask: GetChannelMask::<Identity> }
    }
}
struct IXAudio2MasteringVoice_ImplVtbl<T: IXAudio2MasteringVoice_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2MasteringVoice_Impl> IXAudio2MasteringVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2MasteringVoice_Vtbl = IXAudio2MasteringVoice_Vtbl::new::<T>();
}
impl IXAudio2MasteringVoice {
    pub fn new<'a, T: IXAudio2MasteringVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2MasteringVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IXAudio2SourceVoice, IXAudio2SourceVoice_Vtbl);
impl core::ops::Deref for IXAudio2SourceVoice {
    type Target = IXAudio2Voice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAudio2SourceVoice, IXAudio2Voice);
impl IXAudio2SourceVoice {
    pub unsafe fn Start(&self, flags: u32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), flags, operationset).ok() }
    }
    pub unsafe fn Stop(&self, flags: u32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), flags, operationset).ok() }
    }
    pub unsafe fn SubmitSourceBuffer(&self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: Option<*const XAUDIO2_BUFFER_WMA>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SubmitSourceBuffer)(windows_core::Interface::as_raw(self), pbuffer, pbufferwma.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn FlushSourceBuffers(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FlushSourceBuffers)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Discontinuity(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Discontinuity)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExitLoop(&self, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExitLoop)(windows_core::Interface::as_raw(self), operationset).ok() }
    }
    pub unsafe fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
        unsafe { (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pvoicestate as _, flags) }
    }
    pub unsafe fn SetFrequencyRatio(&self, ratio: f32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFrequencyRatio)(windows_core::Interface::as_raw(self), ratio, operationset).ok() }
    }
    pub unsafe fn GetFrequencyRatio(&self) -> f32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrequencyRatio)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetSourceSampleRate(&self, newsourcesamplerate: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceSampleRate)(windows_core::Interface::as_raw(self), newsourcesamplerate).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SourceVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SubmitSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_BUFFER, *const XAUDIO2_BUFFER_WMA) -> windows_core::HRESULT,
    pub FlushSourceBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExitLoop: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_VOICE_STATE, u32),
    pub SetFrequencyRatio: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub GetFrequencyRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32),
    pub SetSourceSampleRate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IXAudio2SourceVoice_Impl: IXAudio2Voice_Impl {
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
    pub const fn new<Identity: IXAudio2SourceVoice_Impl>() -> Self {
        unsafe extern "system" fn Start<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, flags: u32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::Start(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, flags: u32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::Stop(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn SubmitSourceBuffer<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::SubmitSourceBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pbufferwma)).into()
            }
        }
        unsafe extern "system" fn FlushSourceBuffers<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::FlushSourceBuffers(this).into()
            }
        }
        unsafe extern "system" fn Discontinuity<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::Discontinuity(this).into()
            }
        }
        unsafe extern "system" fn ExitLoop<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::ExitLoop(this, core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: u32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::GetState(this, core::mem::transmute_copy(&pvoicestate), core::mem::transmute_copy(&flags))
            }
        }
        unsafe extern "system" fn SetFrequencyRatio<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, ratio: f32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::SetFrequencyRatio(this, core::mem::transmute_copy(&ratio), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetFrequencyRatio<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pratio: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::GetFrequencyRatio(this, core::mem::transmute_copy(&pratio))
            }
        }
        unsafe extern "system" fn SetSourceSampleRate<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, newsourcesamplerate: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::SetSourceSampleRate(this, core::mem::transmute_copy(&newsourcesamplerate)).into()
            }
        }
        Self {
            base__: IXAudio2Voice_Vtbl::new::<Identity>(),
            Start: Start::<Identity>,
            Stop: Stop::<Identity>,
            SubmitSourceBuffer: SubmitSourceBuffer::<Identity>,
            FlushSourceBuffers: FlushSourceBuffers::<Identity>,
            Discontinuity: Discontinuity::<Identity>,
            ExitLoop: ExitLoop::<Identity>,
            GetState: GetState::<Identity>,
            SetFrequencyRatio: SetFrequencyRatio::<Identity>,
            GetFrequencyRatio: GetFrequencyRatio::<Identity>,
            SetSourceSampleRate: SetSourceSampleRate::<Identity>,
        }
    }
}
struct IXAudio2SourceVoice_ImplVtbl<T: IXAudio2SourceVoice_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2SourceVoice_Impl> IXAudio2SourceVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2SourceVoice_Vtbl = IXAudio2SourceVoice_Vtbl::new::<T>();
}
impl IXAudio2SourceVoice {
    pub fn new<'a, T: IXAudio2SourceVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2SourceVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IXAudio2SubmixVoice, IXAudio2SubmixVoice_Vtbl);
impl core::ops::Deref for IXAudio2SubmixVoice {
    type Target = IXAudio2Voice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAudio2SubmixVoice, IXAudio2Voice);
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SubmixVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
}
pub trait IXAudio2SubmixVoice_Impl: IXAudio2Voice_Impl {}
impl IXAudio2SubmixVoice_Vtbl {
    pub const fn new<Identity: IXAudio2SubmixVoice_Impl>() -> Self {
        Self { base__: IXAudio2Voice_Vtbl::new::<Identity>() }
    }
}
struct IXAudio2SubmixVoice_ImplVtbl<T: IXAudio2SubmixVoice_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2SubmixVoice_Impl> IXAudio2SubmixVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2SubmixVoice_Vtbl = IXAudio2SubmixVoice_Vtbl::new::<T>();
}
impl IXAudio2SubmixVoice {
    pub fn new<'a, T: IXAudio2SubmixVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2SubmixVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IXAudio2Voice, IXAudio2Voice_Vtbl);
impl IXAudio2Voice {
    pub unsafe fn GetVoiceDetails(&self) -> XAUDIO2_VOICE_DETAILS {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVoiceDetails)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetOutputVoices(&self, psendlist: Option<*const XAUDIO2_VOICE_SENDS>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOutputVoices)(windows_core::Interface::as_raw(self), psendlist.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetEffectChain(&self, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffectChain)(windows_core::Interface::as_raw(self), peffectchain.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn EnableEffect(&self, effectindex: u32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableEffect)(windows_core::Interface::as_raw(self), effectindex, operationset).ok() }
    }
    pub unsafe fn DisableEffect(&self, effectindex: u32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableEffect)(windows_core::Interface::as_raw(self), effectindex, operationset).ok() }
    }
    pub unsafe fn GetEffectState(&self, effectindex: u32) -> windows_core::BOOL {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectState)(windows_core::Interface::as_raw(self), effectindex, &mut result__);
            result__
        }
    }
    pub unsafe fn SetEffectParameters(&self, effectindex: u32, pparameters: *const core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEffectParameters)(windows_core::Interface::as_raw(self), effectindex, pparameters, parametersbytesize, operationset).ok() }
    }
    pub unsafe fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut core::ffi::c_void, parametersbytesize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectParameters)(windows_core::Interface::as_raw(self), effectindex, pparameters as _, parametersbytesize).ok() }
    }
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFilterParameters)(windows_core::Interface::as_raw(self), pparameters, operationset).ok() }
    }
    pub unsafe fn GetFilterParameters(&self) -> XAUDIO2_FILTER_PARAMETERS {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilterParameters)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXAudio2Voice>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFilterParameters)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), pparameters, operationset).ok() }
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0) -> XAUDIO2_FILTER_PARAMETERS
    where
        P0: windows_core::Param<IXAudio2Voice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputFilterParameters)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), &mut result__);
            result__
        }
    }
    pub unsafe fn SetVolume(&self, volume: f32, operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume, operationset).ok() }
    }
    pub unsafe fn GetVolume(&self) -> f32 {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumes)(windows_core::Interface::as_raw(self), pvolumes.len().try_into().unwrap(), core::mem::transmute(pvolumes.as_ptr()), operationset).ok() }
    }
    pub unsafe fn GetChannelVolumes(&self, pvolumes: &mut [f32]) {
        unsafe { (windows_core::Interface::vtable(self).GetChannelVolumes)(windows_core::Interface::as_raw(self), pvolumes.len().try_into().unwrap(), core::mem::transmute(pvolumes.as_ptr())) }
    }
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IXAudio2Voice>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputMatrix)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset).ok() }
    }
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: u32, destinationchannels: u32) -> f32
    where
        P0: windows_core::Param<IXAudio2Voice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputMatrix)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), sourcechannels, destinationchannels, &mut result__);
            result__
        }
    }
    pub unsafe fn DestroyVoice(&self) {
        unsafe { (windows_core::Interface::vtable(self).DestroyVoice)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Voice_Vtbl {
    pub GetVoiceDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_VOICE_DETAILS),
    pub SetOutputVoices: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_VOICE_SENDS) -> windows_core::HRESULT,
    pub SetEffectChain: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT,
    pub EnableEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub DisableEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetEffectState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL),
    pub SetEffectParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetEffectParameters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_FILTER_PARAMETERS, u32) -> windows_core::HRESULT,
    pub GetFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_FILTER_PARAMETERS),
    pub SetOutputFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const XAUDIO2_FILTER_PARAMETERS, u32) -> windows_core::HRESULT,
    pub GetOutputFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut XAUDIO2_FILTER_PARAMETERS),
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32, u32) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32),
    pub SetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32),
    pub SetOutputMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const f32, u32) -> windows_core::HRESULT,
    pub GetOutputMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut f32),
    pub DestroyVoice: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IXAudio2Voice_Impl {
    fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS);
    fn SetOutputVoices(&self, psendlist: *const XAUDIO2_VOICE_SENDS) -> windows_core::Result<()>;
    fn SetEffectChain(&self, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn EnableEffect(&self, effectindex: u32, operationset: u32) -> windows_core::Result<()>;
    fn DisableEffect(&self, effectindex: u32, operationset: u32) -> windows_core::Result<()>;
    fn GetEffectState(&self, effectindex: u32, penabled: *mut windows_core::BOOL);
    fn SetEffectParameters(&self, effectindex: u32, pparameters: *const core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> windows_core::Result<()>;
    fn GetEffectParameters(&self, effectindex: u32, pparameters: *mut core::ffi::c_void, parametersbytesize: u32) -> windows_core::Result<()>;
    fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::Result<()>;
    fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetOutputFilterParameters(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::Result<()>;
    fn GetOutputFilterParameters(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetVolume(&self, volume: f32, operationset: u32) -> windows_core::Result<()>;
    fn GetVolume(&self, pvolume: *mut f32);
    fn SetChannelVolumes(&self, channels: u32, pvolumes: *const f32, operationset: u32) -> windows_core::Result<()>;
    fn GetChannelVolumes(&self, channels: u32, pvolumes: *mut f32);
    fn SetOutputMatrix(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> windows_core::Result<()>;
    fn GetOutputMatrix(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32);
    fn DestroyVoice(&self);
}
impl IXAudio2Voice_Vtbl {
    pub const fn new<Identity: IXAudio2Voice_Impl>() -> Self {
        unsafe extern "system" fn GetVoiceDetails<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetVoiceDetails(this, core::mem::transmute_copy(&pvoicedetails))
            }
        }
        unsafe extern "system" fn SetOutputVoices<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetOutputVoices(this, core::mem::transmute_copy(&psendlist)).into()
            }
        }
        unsafe extern "system" fn SetEffectChain<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetEffectChain(this, core::mem::transmute_copy(&peffectchain)).into()
            }
        }
        unsafe extern "system" fn EnableEffect<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::EnableEffect(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn DisableEffect<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::DisableEffect(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetEffectState<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, penabled: *mut windows_core::BOOL) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetEffectState(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&penabled))
            }
        }
        unsafe extern "system" fn SetEffectParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, pparameters: *const core::ffi::c_void, parametersbytesize: u32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetEffectParameters(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parametersbytesize), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetEffectParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: u32, pparameters: *mut core::ffi::c_void, parametersbytesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetEffectParameters(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parametersbytesize)).into()
            }
        }
        unsafe extern "system" fn SetFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetFilterParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetFilterParameters(this, core::mem::transmute_copy(&pparameters))
            }
        }
        unsafe extern "system" fn SetOutputFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetOutputFilterParameters(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetOutputFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetOutputFilterParameters(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&pparameters))
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, volume: f32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetVolume(this, core::mem::transmute_copy(&volume), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pvolume: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetVolume(this, core::mem::transmute_copy(&pvolume))
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, channels: u32, pvolumes: *const f32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetChannelVolumes(this, core::mem::transmute_copy(&channels), core::mem::transmute_copy(&pvolumes), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, channels: u32, pvolumes: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetChannelVolumes(this, core::mem::transmute_copy(&channels), core::mem::transmute_copy(&pvolumes))
            }
        }
        unsafe extern "system" fn SetOutputMatrix<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *const f32, operationset: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetOutputMatrix(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&sourcechannels), core::mem::transmute_copy(&destinationchannels), core::mem::transmute_copy(&plevelmatrix), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetOutputMatrix<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, sourcechannels: u32, destinationchannels: u32, plevelmatrix: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetOutputMatrix(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&sourcechannels), core::mem::transmute_copy(&destinationchannels), core::mem::transmute_copy(&plevelmatrix))
            }
        }
        unsafe extern "system" fn DestroyVoice<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::DestroyVoice(this)
            }
        }
        Self {
            GetVoiceDetails: GetVoiceDetails::<Identity>,
            SetOutputVoices: SetOutputVoices::<Identity>,
            SetEffectChain: SetEffectChain::<Identity>,
            EnableEffect: EnableEffect::<Identity>,
            DisableEffect: DisableEffect::<Identity>,
            GetEffectState: GetEffectState::<Identity>,
            SetEffectParameters: SetEffectParameters::<Identity>,
            GetEffectParameters: GetEffectParameters::<Identity>,
            SetFilterParameters: SetFilterParameters::<Identity>,
            GetFilterParameters: GetFilterParameters::<Identity>,
            SetOutputFilterParameters: SetOutputFilterParameters::<Identity>,
            GetOutputFilterParameters: GetOutputFilterParameters::<Identity>,
            SetVolume: SetVolume::<Identity>,
            GetVolume: GetVolume::<Identity>,
            SetChannelVolumes: SetChannelVolumes::<Identity>,
            GetChannelVolumes: GetChannelVolumes::<Identity>,
            SetOutputMatrix: SetOutputMatrix::<Identity>,
            GetOutputMatrix: GetOutputMatrix::<Identity>,
            DestroyVoice: DestroyVoice::<Identity>,
        }
    }
}
struct IXAudio2Voice_ImplVtbl<T: IXAudio2Voice_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2Voice_Impl> IXAudio2Voice_ImplVtbl<T> {
    const VTABLE: IXAudio2Voice_Vtbl = IXAudio2Voice_Vtbl::new::<T>();
}
impl IXAudio2Voice {
    pub fn new<'a, T: IXAudio2Voice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2Voice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IXAudio2VoiceCallback, IXAudio2VoiceCallback_Vtbl);
impl IXAudio2VoiceCallback {
    pub unsafe fn OnVoiceProcessingPassStart(&self, bytesrequired: u32) {
        unsafe { (windows_core::Interface::vtable(self).OnVoiceProcessingPassStart)(windows_core::Interface::as_raw(self), bytesrequired) }
    }
    pub unsafe fn OnVoiceProcessingPassEnd(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnVoiceProcessingPassEnd)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnStreamEnd(&self) {
        unsafe { (windows_core::Interface::vtable(self).OnStreamEnd)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnBufferStart(&self, pbuffercontext: *mut core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).OnBufferStart)(windows_core::Interface::as_raw(self), pbuffercontext as _) }
    }
    pub unsafe fn OnBufferEnd(&self, pbuffercontext: *mut core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).OnBufferEnd)(windows_core::Interface::as_raw(self), pbuffercontext as _) }
    }
    pub unsafe fn OnLoopEnd(&self, pbuffercontext: *mut core::ffi::c_void) {
        unsafe { (windows_core::Interface::vtable(self).OnLoopEnd)(windows_core::Interface::as_raw(self), pbuffercontext as _) }
    }
    pub unsafe fn OnVoiceError(&self, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT) {
        unsafe { (windows_core::Interface::vtable(self).OnVoiceError)(windows_core::Interface::as_raw(self), pbuffercontext as _, error) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2VoiceCallback_Vtbl {
    pub OnVoiceProcessingPassStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub OnVoiceProcessingPassEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnStreamEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnBufferStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnBufferEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnLoopEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnVoiceError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
}
pub trait IXAudio2VoiceCallback_Impl {
    fn OnVoiceProcessingPassStart(&self, bytesrequired: u32);
    fn OnVoiceProcessingPassEnd(&self);
    fn OnStreamEnd(&self);
    fn OnBufferStart(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnBufferEnd(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnLoopEnd(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnVoiceError(&self, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT);
}
impl IXAudio2VoiceCallback_Vtbl {
    pub const fn new<Identity: IXAudio2VoiceCallback_Impl>() -> Self {
        unsafe extern "system" fn OnVoiceProcessingPassStart<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, bytesrequired: u32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnVoiceProcessingPassStart(this, core::mem::transmute_copy(&bytesrequired))
            }
        }
        unsafe extern "system" fn OnVoiceProcessingPassEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnVoiceProcessingPassEnd(this)
            }
        }
        unsafe extern "system" fn OnStreamEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnStreamEnd(this)
            }
        }
        unsafe extern "system" fn OnBufferStart<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnBufferStart(this, core::mem::transmute_copy(&pbuffercontext))
            }
        }
        unsafe extern "system" fn OnBufferEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnBufferEnd(this, core::mem::transmute_copy(&pbuffercontext))
            }
        }
        unsafe extern "system" fn OnLoopEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnLoopEnd(this, core::mem::transmute_copy(&pbuffercontext))
            }
        }
        unsafe extern "system" fn OnVoiceError<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnVoiceError(this, core::mem::transmute_copy(&pbuffercontext), core::mem::transmute_copy(&error))
            }
        }
        Self {
            OnVoiceProcessingPassStart: OnVoiceProcessingPassStart::<Identity>,
            OnVoiceProcessingPassEnd: OnVoiceProcessingPassEnd::<Identity>,
            OnStreamEnd: OnStreamEnd::<Identity>,
            OnBufferStart: OnBufferStart::<Identity>,
            OnBufferEnd: OnBufferEnd::<Identity>,
            OnLoopEnd: OnLoopEnd::<Identity>,
            OnVoiceError: OnVoiceError::<Identity>,
        }
    }
}
struct IXAudio2VoiceCallback_ImplVtbl<T: IXAudio2VoiceCallback_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2VoiceCallback_Impl> IXAudio2VoiceCallback_ImplVtbl<T> {
    const VTABLE: IXAudio2VoiceCallback_Vtbl = IXAudio2VoiceCallback_Vtbl::new::<T>();
}
impl IXAudio2VoiceCallback {
    pub fn new<'a, T: IXAudio2VoiceCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2VoiceCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub const Large: HrtfEnvironment = HrtfEnvironment(2i32);
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(0i32);
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(4i32);
pub const Medium: HrtfEnvironment = HrtfEnvironment(1i32);
pub const NaturalDecay: HrtfDistanceDecayType = HrtfDistanceDecayType(0i32);
pub const NotchFilter: XAUDIO2_FILTER_TYPE = XAUDIO2_FILTER_TYPE(3i32);
pub const OmniDirectional: HrtfDirectivityType = HrtfDirectivityType(0i32);
pub const Outdoors: HrtfEnvironment = HrtfEnvironment(3i32);
pub const Processor1: u32 = 1u32;
pub const Processor10: u32 = 512u32;
pub const Processor11: u32 = 1024u32;
pub const Processor12: u32 = 2048u32;
pub const Processor13: u32 = 4096u32;
pub const Processor14: u32 = 8192u32;
pub const Processor15: u32 = 16384u32;
pub const Processor16: u32 = 32768u32;
pub const Processor17: u32 = 65536u32;
pub const Processor18: u32 = 131072u32;
pub const Processor19: u32 = 262144u32;
pub const Processor2: u32 = 2u32;
pub const Processor20: u32 = 524288u32;
pub const Processor21: u32 = 1048576u32;
pub const Processor22: u32 = 2097152u32;
pub const Processor23: u32 = 4194304u32;
pub const Processor24: u32 = 8388608u32;
pub const Processor25: u32 = 16777216u32;
pub const Processor26: u32 = 33554432u32;
pub const Processor27: u32 = 67108864u32;
pub const Processor28: u32 = 134217728u32;
pub const Processor29: u32 = 268435456u32;
pub const Processor3: u32 = 4u32;
pub const Processor30: u32 = 536870912u32;
pub const Processor31: u32 = 1073741824u32;
pub const Processor32: u32 = 2147483648u32;
pub const Processor4: u32 = 8u32;
pub const Processor5: u32 = 16u32;
pub const Processor6: u32 = 32u32;
pub const Processor7: u32 = 64u32;
pub const Processor8: u32 = 128u32;
pub const Processor9: u32 = 256u32;
pub const SPEAKER_MONO: u32 = 4u32;
pub const Small: HrtfEnvironment = HrtfEnvironment(0i32);
pub const X3DAUDIO_2PI: f32 = 6.2831855f32;
pub const X3DAUDIO_CALCULATE_DELAY: u32 = 2u32;
pub const X3DAUDIO_CALCULATE_DOPPLER: u32 = 32u32;
pub const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 64u32;
pub const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 4u32;
pub const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 8u32;
pub const X3DAUDIO_CALCULATE_MATRIX: u32 = 1u32;
pub const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 131072u32;
pub const X3DAUDIO_CALCULATE_REVERB: u32 = 16u32;
pub const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 65536u32;
pub const X3DAUDIO_HANDLE_BYTESIZE: u32 = 20u32;
pub const X3DAUDIO_PI: f32 = 3.1415927f32;
pub const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5f32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XAPO_BUFFER_FLAGS(pub i32);
pub const XAPO_BUFFER_SILENT: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(0i32);
pub const XAPO_BUFFER_VALID: XAPO_BUFFER_FLAGS = XAPO_BUFFER_FLAGS(1i32);
pub const XAPO_E_FORMAT_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88970001_u32 as _);
pub const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 4u32;
pub const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 8u32;
pub const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 1u32;
pub const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 2u32;
pub const XAPO_FLAG_INPLACE_REQUIRED: u32 = 32u32;
pub const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 16u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAPO_LOCKFORPROCESS_PARAMETERS {
    pub pFormat: *const super::WAVEFORMATEX,
    pub MaxFrameCount: u32,
}
impl Default for XAPO_LOCKFORPROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAPO_MAX_CHANNELS: u32 = 64u32;
pub const XAPO_MAX_FRAMERATE: u32 = 200000u32;
pub const XAPO_MIN_CHANNELS: u32 = 1u32;
pub const XAPO_MIN_FRAMERATE: u32 = 1000u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAPO_PROCESS_BUFFER_PARAMETERS {
    pub pBuffer: *mut core::ffi::c_void,
    pub BufferFlags: XAPO_BUFFER_FLAGS,
    pub ValidFrameCount: u32,
}
impl Default for XAPO_PROCESS_BUFFER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAPO_REGISTRATION_PROPERTIES {
    pub clsid: windows_core::GUID,
    pub FriendlyName: [u16; 256],
    pub CopyrightInfo: [u16; 256],
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub Flags: u32,
    pub MinInputBufferCount: u32,
    pub MaxInputBufferCount: u32,
    pub MinOutputBufferCount: u32,
    pub MaxOutputBufferCount: u32,
}
impl Default for XAPO_REGISTRATION_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAPO_REGISTRATION_STRING_LENGTH: u32 = 256u32;
pub const XAUDIO2D_DLL: windows_core::PCWSTR = windows_core::w!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_A: windows_core::PCSTR = windows_core::s!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_W: windows_core::PCWSTR = windows_core::w!("xaudio2_9d.dll");
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_DEFAULT_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_DECAY_TIME: f32 = 1f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_DISABLE_LATE_FIELD: u32 = 0u32;
pub const XAUDIO2FX_REVERB_DEFAULT_EARLY_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LATE_DIFFUSION: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_CUTOFF: u32 = 4u32;
pub const XAUDIO2FX_REVERB_DEFAULT_LOW_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION: u32 = 6u32;
pub const XAUDIO2FX_REVERB_DEFAULT_POSITION_MATRIX: u32 = 27u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REFLECTIONS_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_DEFAULT_REVERB_GAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_FREQ: f32 = 5000f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_DEFAULT_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_DEFAULT_WET_DRY_MIX: f32 = 100f32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2FX_REVERB_I3DL2_PARAMETERS {
    pub WetDryMix: f32,
    pub Room: i32,
    pub RoomHF: i32,
    pub RoomRolloffFactor: f32,
    pub DecayTime: f32,
    pub DecayHFRatio: f32,
    pub Reflections: i32,
    pub ReflectionsDelay: f32,
    pub Reverb: i32,
    pub ReverbDelay: f32,
    pub Diffusion: f32,
    pub Density: f32,
    pub HFReference: f32,
}
pub const XAUDIO2FX_REVERB_MAX_7POINT1_REAR_DELAY: u32 = 20u32;
pub const XAUDIO2FX_REVERB_MAX_7POINT1_SIDE_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_DENSITY: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_DIFFUSION: u32 = 15u32;
pub const XAUDIO2FX_REVERB_MAX_FRAMERATE: u32 = 48000u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_CUTOFF: u32 = 14u32;
pub const XAUDIO2FX_REVERB_MAX_HIGH_EQ_GAIN: u32 = 8u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_CUTOFF: u32 = 9u32;
pub const XAUDIO2FX_REVERB_MAX_LOW_EQ_GAIN: u32 = 12u32;
pub const XAUDIO2FX_REVERB_MAX_POSITION: u32 = 30u32;
pub const XAUDIO2FX_REVERB_MAX_REAR_DELAY: u32 = 5u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_DELAY: u32 = 300u32;
pub const XAUDIO2FX_REVERB_MAX_REFLECTIONS_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_DELAY: u32 = 85u32;
pub const XAUDIO2FX_REVERB_MAX_REVERB_GAIN: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_FREQ: f32 = 20000f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_HF: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_FILTER_MAIN: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MAX_ROOM_SIZE: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MAX_WET_DRY_MIX: f32 = 100f32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_7POINT1_SIDE_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_DECAY_TIME: f32 = 0.1f32;
pub const XAUDIO2FX_REVERB_MIN_DENSITY: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_DIFFUSION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_FRAMERATE: u32 = 20000u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_HIGH_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_CUTOFF: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_LOW_EQ_GAIN: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_POSITION: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REAR_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REFLECTIONS_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_DELAY: u32 = 0u32;
pub const XAUDIO2FX_REVERB_MIN_REVERB_GAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_FREQ: f32 = 20f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_HF: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_FILTER_MAIN: f32 = -100f32;
pub const XAUDIO2FX_REVERB_MIN_ROOM_SIZE: f32 = 0f32;
pub const XAUDIO2FX_REVERB_MIN_WET_DRY_MIX: f32 = 0f32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2FX_REVERB_PARAMETERS {
    pub WetDryMix: f32,
    pub ReflectionsDelay: u32,
    pub ReverbDelay: u8,
    pub RearDelay: u8,
    pub SideDelay: u8,
    pub PositionLeft: u8,
    pub PositionRight: u8,
    pub PositionMatrixLeft: u8,
    pub PositionMatrixRight: u8,
    pub EarlyDiffusion: u8,
    pub LateDiffusion: u8,
    pub LowEQGain: u8,
    pub LowEQCutoff: u8,
    pub HighEQGain: u8,
    pub HighEQCutoff: u8,
    pub RoomFilterFreq: f32,
    pub RoomFilterMain: f32,
    pub RoomFilterHF: f32,
    pub ReflectionsGain: f32,
    pub ReverbGain: f32,
    pub DecayTime: f32,
    pub Density: f32,
    pub RoomSize: f32,
    pub DisableLateField: windows_core::BOOL,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2FX_VOLUMEMETER_LEVELS {
    pub pPeakLevels: *mut f32,
    pub pRMSLevels: *mut f32,
    pub ChannelCount: u32,
}
impl Default for XAUDIO2FX_VOLUMEMETER_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_1024_QUANTUM: u32 = 32768u32;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_BUFFER {
    pub Flags: u32,
    pub AudioBytes: u32,
    pub pAudioData: *const u8,
    pub PlayBegin: u32,
    pub PlayLength: u32,
    pub LoopBegin: u32,
    pub LoopLength: u32,
    pub LoopCount: u32,
    pub pContext: *mut core::ffi::c_void,
}
impl Default for XAUDIO2_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *const u32,
    pub PacketCount: u32,
}
impl Default for XAUDIO2_BUFFER_WMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_COMMIT_ALL: u32 = 0u32;
pub const XAUDIO2_COMMIT_NOW: u32 = 0u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: u32,
    pub BreakMask: u32,
    pub LogThreadID: windows_core::BOOL,
    pub LogFileline: windows_core::BOOL,
    pub LogFunctionName: windows_core::BOOL,
    pub LogTiming: windows_core::BOOL,
}
pub const XAUDIO2_DEBUG_ENGINE: u32 = 1u32;
pub const XAUDIO2_DEFAULT_CHANNELS: u32 = 0u32;
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1f32;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2f32;
pub const XAUDIO2_DEFAULT_PROCESSOR: u32 = 1u32;
pub const XAUDIO2_DEFAULT_SAMPLERATE: u32 = 0u32;
pub const XAUDIO2_DLL: windows_core::PCWSTR = windows_core::w!("xaudio2_9.dll");
pub const XAUDIO2_DLL_A: windows_core::PCSTR = windows_core::s!("xaudio2_9.dll");
pub const XAUDIO2_DLL_W: windows_core::PCWSTR = windows_core::w!("xaudio2_9.dll");
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: u32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
impl Default for XAUDIO2_EFFECT_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Default)]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub InitialState: windows_core::BOOL,
    pub OutputChannels: u32,
}
pub const XAUDIO2_END_OF_STREAM: u32 = 64u32;
pub const XAUDIO2_E_DEVICE_INVALIDATED: windows_core::HRESULT = windows_core::HRESULT(0x88960004_u32 as _);
pub const XAUDIO2_E_INVALID_CALL: windows_core::HRESULT = windows_core::HRESULT(0x88960001_u32 as _);
pub const XAUDIO2_E_XAPO_CREATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88960003_u32 as _);
pub const XAUDIO2_E_XMA_DECODER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88960002_u32 as _);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XAUDIO2_FILTER_TYPE(pub i32);
pub const XAUDIO2_LOG_API_CALLS: u32 = 16u32;
pub const XAUDIO2_LOG_DETAIL: u32 = 8u32;
pub const XAUDIO2_LOG_ERRORS: u32 = 1u32;
pub const XAUDIO2_LOG_FUNC_CALLS: u32 = 32u32;
pub const XAUDIO2_LOG_INFO: u32 = 4u32;
pub const XAUDIO2_LOG_LOCKS: u32 = 128u32;
pub const XAUDIO2_LOG_MEMORY: u32 = 256u32;
pub const XAUDIO2_LOG_STREAMING: u32 = 4096u32;
pub const XAUDIO2_LOG_TIMING: u32 = 64u32;
pub const XAUDIO2_LOG_WARNINGS: u32 = 2u32;
pub const XAUDIO2_LOOP_INFINITE: u32 = 255u32;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64u32;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: u32 = 2u32;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648u32;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1f32;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5f32;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024f32;
pub const XAUDIO2_MAX_INSTANCES: u32 = 8u32;
pub const XAUDIO2_MAX_LOOP_COUNT: u32 = 254u32;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: u32 = 64u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: u32 = 600000u32;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: u32 = 300000u32;
pub const XAUDIO2_MAX_SAMPLE_RATE: u32 = 200000u32;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216f32;
pub const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1000u32;
pub const XAUDIO2_NO_LOOP_REGION: u32 = 0u32;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: u32 = 65536u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: u64,
    pub TotalCyclesSinceLastQuery: u64,
    pub MinimumCyclesPerQuantum: u32,
    pub MaximumCyclesPerQuantum: u32,
    pub MemoryUsageInBytes: u32,
    pub CurrentLatencyInSamples: u32,
    pub GlitchesSinceEngineStarted: u32,
    pub ActiveSourceVoiceCount: u32,
    pub TotalSourceVoiceCount: u32,
    pub ActiveSubmixVoiceCount: u32,
    pub ActiveResamplerCount: u32,
    pub ActiveMatrixMixCount: u32,
    pub ActiveXmaSourceVoices: u32,
    pub ActiveXmaStreams: u32,
}
pub const XAUDIO2_PLAY_TAILS: u32 = 32u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: u32 = 100u32;
pub const XAUDIO2_QUANTUM_NUMERATOR: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Default)]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: u32,
    pub pOutputVoice: core::mem::ManuallyDrop<Option<IXAudio2Voice>>,
}
pub const XAUDIO2_SEND_USEFILTER: u32 = 128u32;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: u32 = 8192u32;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: u32 = 0u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: u32,
    pub ActiveFlags: u32,
    pub InputChannels: u32,
    pub InputSampleRate: u32,
}
pub const XAUDIO2_VOICE_NOPITCH: u32 = 2u32;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 256u32;
pub const XAUDIO2_VOICE_NOSRC: u32 = 4u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: u32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
impl Default for XAUDIO2_VOICE_SENDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut core::ffi::c_void,
    pub BuffersQueued: u32,
    pub SamplesPlayed: u64,
}
impl Default for XAUDIO2_VOICE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XAUDIO2_VOICE_USEFILTER: u32 = 8u32;
