#[cfg(feature = "Win32_Media_Audio_Apo")]
pub mod Apo;
#[cfg(feature = "Win32_Media_Audio_DirectMusic")]
pub mod DirectMusic;
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub mod DirectSound;
#[cfg(feature = "Win32_Media_Audio_Endpoints")]
pub mod Endpoints;
#[cfg(feature = "Win32_Media_Audio_XAudio2")]
pub mod XAudio2;
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<P0, P3>(deviceinterfacepath: P0, riid: *const windows_core::GUID, activationparams: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>, completionhandler: P3) -> windows_core::Result<IActivateAudioInterfaceAsyncOperation>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IActivateAudioInterfaceCompletionHandler>,
{
    windows_core::link!("mmdevapi.dll" "system" fn ActivateAudioInterfaceAsync(deviceinterfacepath : windows_core::PCWSTR, riid : *const windows_core::GUID, activationparams : *const super::super::System::Com::StructuredStorage:: PROPVARIANT, completionhandler : * mut core::ffi::c_void, activationoperation : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ActivateAudioInterfaceAsync(deviceinterfacepath.param().abi(), riid, activationparams.unwrap_or(core::mem::zeroed()) as _, completionhandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoRegisterMessageFilter<P0>(lpmessagefilter: P0, lplpmessagefilter: Option<*mut Option<IMessageFilter>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMessageFilter>,
{
    windows_core::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : * mut core::ffi::c_void, lplpmessagefilter : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CoRegisterMessageFilter(lpmessagefilter.param().abi(), lplpmessagefilter.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitor() -> windows_core::Result<IAudioStateMonitor> {
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitor(audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateCaptureAudioStateMonitor(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<IAudioStateMonitor> {
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategory(category : AUDIO_STREAM_CATEGORY, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategory(category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId<P1>(category: AUDIO_STREAM_CATEGORY, deviceid: P1) -> windows_core::Result<IAudioStateMonitor>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category : AUDIO_STREAM_CATEGORY, deviceid : windows_core::PCWSTR, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category, deviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> windows_core::Result<IAudioStateMonitor> {
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category : AUDIO_STREAM_CATEGORY, role : ERole, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitor() -> windows_core::Result<IAudioStateMonitor> {
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitor(audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateRenderAudioStateMonitor(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<IAudioStateMonitor> {
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategory(category : AUDIO_STREAM_CATEGORY, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategory(category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceId<P1>(category: AUDIO_STREAM_CATEGORY, deviceid: P1) -> windows_core::Result<IAudioStateMonitor>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category : AUDIO_STREAM_CATEGORY, deviceid : windows_core::PCWSTR, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceId(category, deviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> windows_core::Result<IAudioStateMonitor> {
    windows_core::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category : AUDIO_STREAM_CATEGORY, role : ERole, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn PlaySoundA<P0>(pszsound: P0, hmod: Option<super::super::Foundation::HMODULE>, fdwsound: SND_FLAGS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn PlaySoundA(pszsound : windows_core::PCSTR, hmod : super::super::Foundation:: HMODULE, fdwsound : SND_FLAGS) -> windows_core::BOOL);
    unsafe { PlaySoundA(pszsound.param().abi(), hmod.unwrap_or(core::mem::zeroed()) as _, fdwsound) }
}
#[inline]
pub unsafe fn PlaySoundW<P0>(pszsound: P0, hmod: Option<super::super::Foundation::HMODULE>, fdwsound: SND_FLAGS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn PlaySoundW(pszsound : windows_core::PCWSTR, hmod : super::super::Foundation:: HMODULE, fdwsound : SND_FLAGS) -> windows_core::BOOL);
    unsafe { PlaySoundW(pszsound.param().abi(), hmod.unwrap_or(core::mem::zeroed()) as _, fdwsound) }
}
#[inline]
pub unsafe fn acmDriverAddA(phadid: *mut HACMDRIVERID, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverAddA(phadid : *mut HACMDRIVERID, hinstmodule : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM, dwpriority : u32, fdwadd : u32) -> u32);
    unsafe { acmDriverAddA(phadid as _, hinstmodule, lparam, dwpriority, fdwadd) }
}
#[inline]
pub unsafe fn acmDriverAddW(phadid: *mut HACMDRIVERID, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverAddW(phadid : *mut HACMDRIVERID, hinstmodule : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM, dwpriority : u32, fdwadd : u32) -> u32);
    unsafe { acmDriverAddW(phadid as _, hinstmodule, lparam, dwpriority, fdwadd) }
}
#[inline]
pub unsafe fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverClose(had : HACMDRIVER, fdwclose : u32) -> u32);
    unsafe { acmDriverClose(had, fdwclose) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverDetailsA(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSA, fdwdetails : u32) -> u32);
    unsafe { acmDriverDetailsA(hadid, padd as _, fdwdetails) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverDetailsW(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSW, fdwdetails : u32) -> u32);
    unsafe { acmDriverDetailsW(hadid, padd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverEnum(fncallback : ACMDRIVERENUMCB, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmDriverEnum(fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmDriverID(hao: HACMOBJ, phadid: *mut HACMDRIVERID, fdwdriverid: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverID(hao : HACMOBJ, phadid : *mut HACMDRIVERID, fdwdriverid : u32) -> u32);
    unsafe { acmDriverID(hao, phadid as _, fdwdriverid) }
}
#[inline]
pub unsafe fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT {
    windows_core::link!("msacm32.dll" "system" fn acmDriverMessage(had : HACMDRIVER, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    unsafe { acmDriverMessage(had, umsg, lparam1, lparam2) }
}
#[inline]
pub unsafe fn acmDriverOpen(phad: *mut HACMDRIVER, hadid: HACMDRIVERID, fdwopen: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverOpen(phad : *mut HACMDRIVER, hadid : HACMDRIVERID, fdwopen : u32) -> u32);
    unsafe { acmDriverOpen(phad as _, hadid, fdwopen) }
}
#[inline]
pub unsafe fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverPriority(hadid : HACMDRIVERID, dwpriority : u32, fdwpriority : u32) -> u32);
    unsafe { acmDriverPriority(hadid, dwpriority, fdwpriority) }
}
#[inline]
pub unsafe fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmDriverRemove(hadid : HACMDRIVERID, fdwremove : u32) -> u32);
    unsafe { acmDriverRemove(hadid, fdwremove) }
}
#[inline]
pub unsafe fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterChooseA(pafltrc : *mut ACMFILTERCHOOSEA) -> u32);
    unsafe { acmFilterChooseA(pafltrc as _) }
}
#[inline]
pub unsafe fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterChooseW(pafltrc : *mut ACMFILTERCHOOSEW) -> u32);
    unsafe { acmFilterChooseW(pafltrc as _) }
}
#[inline]
pub unsafe fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterDetailsA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fdwdetails : u32) -> u32);
    unsafe { acmFilterDetailsA(had, pafd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterDetailsW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fdwdetails : u32) -> u32);
    unsafe { acmFilterDetailsW(had, pafd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterEnumA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fncallback : ACMFILTERENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFilterEnumA(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterEnumW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fncallback : ACMFILTERENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFilterEnumW(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fdwdetails : u32) -> u32);
    unsafe { acmFilterTagDetailsA(had, paftd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fdwdetails : u32) -> u32);
    unsafe { acmFilterTagDetailsW(had, paftd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagEnumA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fncallback : ACMFILTERTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFilterTagEnumA(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFilterTagEnumW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fncallback : ACMFILTERTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFilterTagEnumW(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatChooseA(pafmtc : *mut ACMFORMATCHOOSEA) -> u32);
    unsafe { acmFormatChooseA(pafmtc as _) }
}
#[inline]
pub unsafe fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatChooseW(pafmtc : *mut ACMFORMATCHOOSEW) -> u32);
    unsafe { acmFormatChooseW(pafmtc as _) }
}
#[inline]
pub unsafe fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatDetailsA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fdwdetails : u32) -> u32);
    unsafe { acmFormatDetailsA(had, pafd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatDetailsW(had : HACMDRIVER, pafd : *mut tACMFORMATDETAILSW, fdwdetails : u32) -> u32);
    unsafe { acmFormatDetailsW(had, pafd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatEnumA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fncallback : ACMFORMATENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFormatEnumA(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatEnumW(had : HACMDRIVER, pafd : *mut tACMFORMATDETAILSW, fncallback : ACMFORMATENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFormatEnumW(had, pafd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatSuggest(had : HACMDRIVER, pwfxsrc : *mut WAVEFORMATEX, pwfxdst : *mut WAVEFORMATEX, cbwfxdst : u32, fdwsuggest : u32) -> u32);
    unsafe { acmFormatSuggest(had, pwfxsrc as _, pwfxdst as _, cbwfxdst, fdwsuggest) }
}
#[inline]
pub unsafe fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fdwdetails : u32) -> u32);
    unsafe { acmFormatTagDetailsA(had, paftd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fdwdetails : u32) -> u32);
    unsafe { acmFormatTagDetailsW(had, paftd as _, fdwdetails) }
}
#[inline]
pub unsafe fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagEnumA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fncallback : ACMFORMATTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFormatTagEnumA(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmFormatTagEnumW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fncallback : ACMFORMATTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    unsafe { acmFormatTagEnumW(had, paftd as _, fncallback, dwinstance, fdwenum) }
}
#[inline]
pub unsafe fn acmGetVersion() -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmGetVersion() -> u32);
    unsafe { acmGetVersion() }
}
#[inline]
pub unsafe fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmMetrics(hao : HACMOBJ, umetric : u32, pmetric : *mut core::ffi::c_void) -> u32);
    unsafe { acmMetrics(hao, umetric, pmetric as _) }
}
#[inline]
pub unsafe fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamClose(has : HACMSTREAM, fdwclose : u32) -> u32);
    unsafe { acmStreamClose(has, fdwclose) }
}
#[inline]
pub unsafe fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamConvert(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwconvert : u32) -> u32);
    unsafe { acmStreamConvert(has, pash as _, fdwconvert) }
}
#[inline]
pub unsafe fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamMessage(has : HACMSTREAM, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> u32);
    unsafe { acmStreamMessage(has, umsg, lparam1, lparam2) }
}
#[inline]
pub unsafe fn acmStreamOpen(phas: *mut HACMSTREAM, had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamOpen(phas : *mut HACMSTREAM, had : HACMDRIVER, pwfxsrc : *mut WAVEFORMATEX, pwfxdst : *mut WAVEFORMATEX, pwfltr : *mut WAVEFILTER, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
    unsafe { acmStreamOpen(phas as _, had, pwfxsrc as _, pwfxdst as _, pwfltr as _, dwcallback, dwinstance, fdwopen) }
}
#[inline]
pub unsafe fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamPrepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwprepare : u32) -> u32);
    unsafe { acmStreamPrepareHeader(has, pash as _, fdwprepare) }
}
#[inline]
pub unsafe fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamReset(has : HACMSTREAM, fdwreset : u32) -> u32);
    unsafe { acmStreamReset(has, fdwreset) }
}
#[inline]
pub unsafe fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamSize(has : HACMSTREAM, cbinput : u32, pdwoutputbytes : *mut u32, fdwsize : u32) -> u32);
    unsafe { acmStreamSize(has, cbinput, pdwoutputbytes as _, fdwsize) }
}
#[inline]
pub unsafe fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32 {
    windows_core::link!("msacm32.dll" "system" fn acmStreamUnprepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwunprepare : u32) -> u32);
    unsafe { acmStreamUnprepareHeader(has, pash as _, fdwunprepare) }
}
#[inline]
pub unsafe fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxGetDevCapsA(udeviceid : usize, pac : *mut AUXCAPSA, cbac : u32) -> u32);
    unsafe { auxGetDevCapsA(udeviceid, pac as _, cbac) }
}
#[inline]
pub unsafe fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxGetDevCapsW(udeviceid : usize, pac : *mut AUXCAPSW, cbac : u32) -> u32);
    unsafe { auxGetDevCapsW(udeviceid, pac as _, cbac) }
}
#[inline]
pub unsafe fn auxGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxGetNumDevs() -> u32);
    unsafe { auxGetNumDevs() }
}
#[inline]
pub unsafe fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxGetVolume(udeviceid : u32, pdwvolume : *mut u32) -> u32);
    unsafe { auxGetVolume(udeviceid, pdwvolume as _) }
}
#[inline]
pub unsafe fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxOutMessage(udeviceid : u32, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    unsafe { auxOutMessage(udeviceid, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn auxSetVolume(udeviceid : u32, dwvolume : u32) -> u32);
    unsafe { auxSetVolume(udeviceid, dwvolume) }
}
#[inline]
pub unsafe fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiConnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> u32);
    unsafe { midiConnect(hmi, hmo, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiDisconnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> u32);
    unsafe { midiDisconnect(hmi, hmo, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInAddBuffer(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiInAddBuffer(hmi, pmh as _, cbmh) }
}
#[inline]
pub unsafe fn midiInClose(hmi: HMIDIIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInClose(hmi : HMIDIIN) -> u32);
    unsafe { midiInClose(hmi) }
}
#[inline]
pub unsafe fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetDevCapsA(udeviceid : usize, pmic : *mut MIDIINCAPSA, cbmic : u32) -> u32);
    unsafe { midiInGetDevCapsA(udeviceid, pmic as _, cbmic) }
}
#[inline]
pub unsafe fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetDevCapsW(udeviceid : usize, pmic : *mut MIDIINCAPSW, cbmic : u32) -> u32);
    unsafe { midiInGetDevCapsW(udeviceid, pmic as _, cbmic) }
}
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    unsafe { midiInGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    unsafe { midiInGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetID(hmi : HMIDIIN, pudeviceid : *mut u32) -> u32);
    unsafe { midiInGetID(hmi, pudeviceid as _) }
}
#[inline]
pub unsafe fn midiInGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInGetNumDevs() -> u32);
    unsafe { midiInGetNumDevs() }
}
#[inline]
pub unsafe fn midiInMessage(hmi: Option<HMIDIIN>, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInMessage(hmi : HMIDIIN, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    unsafe { midiInMessage(hmi.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInOpen(phmi : *mut HMIDIIN, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    unsafe { midiInOpen(phmi as _, udeviceid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInPrepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiInPrepareHeader(hmi, pmh as _, cbmh) }
}
#[inline]
pub unsafe fn midiInReset(hmi: HMIDIIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInReset(hmi : HMIDIIN) -> u32);
    unsafe { midiInReset(hmi) }
}
#[inline]
pub unsafe fn midiInStart(hmi: HMIDIIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInStart(hmi : HMIDIIN) -> u32);
    unsafe { midiInStart(hmi) }
}
#[inline]
pub unsafe fn midiInStop(hmi: HMIDIIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInStop(hmi : HMIDIIN) -> u32);
    unsafe { midiInStop(hmi) }
}
#[inline]
pub unsafe fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiInUnprepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiInUnprepareHeader(hmi, pmh as _, cbmh) }
}
#[inline]
pub unsafe fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: &[u16; 128], fucache: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutCacheDrumPatches(hmo : HMIDIOUT, upatch : u32, pwkya : *const u16, fucache : u32) -> u32);
    unsafe { midiOutCacheDrumPatches(hmo, upatch, core::mem::transmute(pwkya.as_ptr()), fucache) }
}
#[inline]
pub unsafe fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: &[u16; 128], fucache: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutCachePatches(hmo : HMIDIOUT, ubank : u32, pwpa : *const u16, fucache : u32) -> u32);
    unsafe { midiOutCachePatches(hmo, ubank, core::mem::transmute(pwpa.as_ptr()), fucache) }
}
#[inline]
pub unsafe fn midiOutClose(hmo: HMIDIOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutClose(hmo : HMIDIOUT) -> u32);
    unsafe { midiOutClose(hmo) }
}
#[inline]
pub unsafe fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetDevCapsA(udeviceid : usize, pmoc : *mut MIDIOUTCAPSA, cbmoc : u32) -> u32);
    unsafe { midiOutGetDevCapsA(udeviceid, pmoc as _, cbmoc) }
}
#[inline]
pub unsafe fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetDevCapsW(udeviceid : usize, pmoc : *mut MIDIOUTCAPSW, cbmoc : u32) -> u32);
    unsafe { midiOutGetDevCapsW(udeviceid, pmoc as _, cbmoc) }
}
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    unsafe { midiOutGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    unsafe { midiOutGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetID(hmo : HMIDIOUT, pudeviceid : *mut u32) -> u32);
    unsafe { midiOutGetID(hmo, pudeviceid as _) }
}
#[inline]
pub unsafe fn midiOutGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetNumDevs() -> u32);
    unsafe { midiOutGetNumDevs() }
}
#[inline]
pub unsafe fn midiOutGetVolume(hmo: Option<HMIDIOUT>, pdwvolume: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutGetVolume(hmo : HMIDIOUT, pdwvolume : *mut u32) -> u32);
    unsafe { midiOutGetVolume(hmo.unwrap_or(core::mem::zeroed()) as _, pdwvolume as _) }
}
#[inline]
pub unsafe fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutLongMsg(hmo : HMIDIOUT, pmh : *const MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiOutLongMsg(hmo, pmh, cbmh) }
}
#[inline]
pub unsafe fn midiOutMessage(hmo: Option<HMIDIOUT>, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutMessage(hmo : HMIDIOUT, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    unsafe { midiOutMessage(hmo.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutOpen(phmo : *mut HMIDIOUT, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    unsafe { midiOutOpen(phmo as _, udeviceid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutPrepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiOutPrepareHeader(hmo, pmh as _, cbmh) }
}
#[inline]
pub unsafe fn midiOutReset(hmo: HMIDIOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutReset(hmo : HMIDIOUT) -> u32);
    unsafe { midiOutReset(hmo) }
}
#[inline]
pub unsafe fn midiOutSetVolume(hmo: Option<HMIDIOUT>, dwvolume: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutSetVolume(hmo : HMIDIOUT, dwvolume : u32) -> u32);
    unsafe { midiOutSetVolume(hmo.unwrap_or(core::mem::zeroed()) as _, dwvolume) }
}
#[inline]
pub unsafe fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutShortMsg(hmo : HMIDIOUT, dwmsg : u32) -> u32);
    unsafe { midiOutShortMsg(hmo, dwmsg) }
}
#[inline]
pub unsafe fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiOutUnprepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiOutUnprepareHeader(hmo, pmh as _, cbmh) }
}
#[inline]
pub unsafe fn midiStreamClose(hms: HMIDISTRM) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamClose(hms : HMIDISTRM) -> u32);
    unsafe { midiStreamClose(hms) }
}
#[inline]
pub unsafe fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: &mut [u32], dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamOpen(phms : *mut HMIDISTRM, pudeviceid : *mut u32, cmidi : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
    unsafe { midiStreamOpen(phms as _, core::mem::transmute(pudeviceid.as_ptr()), pudeviceid.len().try_into().unwrap(), dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamOut(hms : HMIDISTRM, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    unsafe { midiStreamOut(hms, pmh as _, cbmh) }
}
#[inline]
pub unsafe fn midiStreamPause(hms: HMIDISTRM) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamPause(hms : HMIDISTRM) -> u32);
    unsafe { midiStreamPause(hms) }
}
#[inline]
pub unsafe fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamPosition(hms : HMIDISTRM, lpmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
    unsafe { midiStreamPosition(hms, lpmmt as _, cbmmt) }
}
#[inline]
pub unsafe fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamProperty(hms : HMIDISTRM, lppropdata : *mut u8, dwproperty : u32) -> u32);
    unsafe { midiStreamProperty(hms, lppropdata as _, dwproperty) }
}
#[inline]
pub unsafe fn midiStreamRestart(hms: HMIDISTRM) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamRestart(hms : HMIDISTRM) -> u32);
    unsafe { midiStreamRestart(hms) }
}
#[inline]
pub unsafe fn midiStreamStop(hms: HMIDISTRM) -> u32 {
    windows_core::link!("winmm.dll" "system" fn midiStreamStop(hms : HMIDISTRM) -> u32);
    unsafe { midiStreamStop(hms) }
}
#[inline]
pub unsafe fn mixerClose(hmx: HMIXER) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerClose(hmx : HMIXER) -> u32);
    unsafe { mixerClose(hmx) }
}
#[inline]
pub unsafe fn mixerGetControlDetailsA(hmxobj: Option<HMIXEROBJ>, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetControlDetailsA(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
    unsafe { mixerGetControlDetailsA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd as _, fdwdetails) }
}
#[inline]
pub unsafe fn mixerGetControlDetailsW(hmxobj: Option<HMIXEROBJ>, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetControlDetailsW(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
    unsafe { mixerGetControlDetailsW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd as _, fdwdetails) }
}
#[inline]
pub unsafe fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetDevCapsA(umxid : usize, pmxcaps : *mut MIXERCAPSA, cbmxcaps : u32) -> u32);
    unsafe { mixerGetDevCapsA(umxid, pmxcaps as _, cbmxcaps) }
}
#[inline]
pub unsafe fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetDevCapsW(umxid : usize, pmxcaps : *mut MIXERCAPSW, cbmxcaps : u32) -> u32);
    unsafe { mixerGetDevCapsW(umxid, pmxcaps as _, cbmxcaps) }
}
#[inline]
pub unsafe fn mixerGetID(hmxobj: Option<HMIXEROBJ>, pumxid: *mut u32, fdwid: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetID(hmxobj : HMIXEROBJ, pumxid : *mut u32, fdwid : u32) -> u32);
    unsafe { mixerGetID(hmxobj.unwrap_or(core::mem::zeroed()) as _, pumxid as _, fdwid) }
}
#[inline]
pub unsafe fn mixerGetLineControlsA(hmxobj: Option<HMIXEROBJ>, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineControlsA(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSA, fdwcontrols : u32) -> u32);
    unsafe { mixerGetLineControlsA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxlc as _, fdwcontrols) }
}
#[inline]
pub unsafe fn mixerGetLineControlsW(hmxobj: Option<HMIXEROBJ>, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineControlsW(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSW, fdwcontrols : u32) -> u32);
    unsafe { mixerGetLineControlsW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxlc as _, fdwcontrols) }
}
#[inline]
pub unsafe fn mixerGetLineInfoA(hmxobj: Option<HMIXEROBJ>, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineInfoA(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEA, fdwinfo : u32) -> u32);
    unsafe { mixerGetLineInfoA(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxl as _, fdwinfo) }
}
#[inline]
pub unsafe fn mixerGetLineInfoW(hmxobj: Option<HMIXEROBJ>, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetLineInfoW(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEW, fdwinfo : u32) -> u32);
    unsafe { mixerGetLineInfoW(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxl as _, fdwinfo) }
}
#[inline]
pub unsafe fn mixerGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerGetNumDevs() -> u32);
    unsafe { mixerGetNumDevs() }
}
#[inline]
pub unsafe fn mixerMessage(hmx: Option<HMIXER>, umsg: u32, dwparam1: Option<usize>, dwparam2: Option<usize>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerMessage(hmx : HMIXER, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
    unsafe { mixerMessage(hmx.unwrap_or(core::mem::zeroed()) as _, umsg, dwparam1.unwrap_or(core::mem::zeroed()) as _, dwparam2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn mixerOpen(phmx: Option<*mut HMIXER>, umxid: u32, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerOpen(phmx : *mut HMIXER, umxid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
    unsafe { mixerOpen(phmx.unwrap_or(core::mem::zeroed()) as _, umxid, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn mixerSetControlDetails(hmxobj: Option<HMIXEROBJ>, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn mixerSetControlDetails(hmxobj : HMIXEROBJ, pmxcd : *const MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
    unsafe { mixerSetControlDetails(hmxobj.unwrap_or(core::mem::zeroed()) as _, pmxcd, fdwdetails) }
}
#[inline]
pub unsafe fn sndPlaySoundA<P0>(pszsound: P0, fusound: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn sndPlaySoundA(pszsound : windows_core::PCSTR, fusound : u32) -> windows_core::BOOL);
    unsafe { sndPlaySoundA(pszsound.param().abi(), fusound) }
}
#[inline]
pub unsafe fn sndPlaySoundW<P0>(pszsound: P0, fusound: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn sndPlaySoundW(pszsound : windows_core::PCWSTR, fusound : u32) -> windows_core::BOOL);
    unsafe { sndPlaySoundW(pszsound.param().abi(), fusound) }
}
#[inline]
pub unsafe fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInAddBuffer(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    unsafe { waveInAddBuffer(hwi, pwh as _, cbwh) }
}
#[inline]
pub unsafe fn waveInClose(hwi: HWAVEIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInClose(hwi : HWAVEIN) -> u32);
    unsafe { waveInClose(hwi) }
}
#[inline]
pub unsafe fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetDevCapsA(udeviceid : usize, pwic : *mut WAVEINCAPSA, cbwic : u32) -> u32);
    unsafe { waveInGetDevCapsA(udeviceid, pwic as _, cbwic) }
}
#[inline]
pub unsafe fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetDevCapsW(udeviceid : usize, pwic : *mut WAVEINCAPSW, cbwic : u32) -> u32);
    unsafe { waveInGetDevCapsW(udeviceid, pwic as _, cbwic) }
}
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    unsafe { waveInGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    unsafe { waveInGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetID(hwi : HWAVEIN, pudeviceid : *const u32) -> u32);
    unsafe { waveInGetID(hwi, pudeviceid) }
}
#[inline]
pub unsafe fn waveInGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetNumDevs() -> u32);
    unsafe { waveInGetNumDevs() }
}
#[inline]
pub unsafe fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInGetPosition(hwi : HWAVEIN, pmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
    unsafe { waveInGetPosition(hwi, pmmt as _, cbmmt) }
}
#[inline]
pub unsafe fn waveInMessage(hwi: Option<HWAVEIN>, umsg: u32, dw1: Option<usize>, dw2: Option<usize>) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInMessage(hwi : HWAVEIN, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    unsafe { waveInMessage(hwi.unwrap_or(core::mem::zeroed()) as _, umsg, dw1.unwrap_or(core::mem::zeroed()) as _, dw2.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn waveInOpen(phwi: Option<*mut HWAVEIN>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInOpen(phwi : *mut HWAVEIN, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    unsafe { waveInOpen(phwi.unwrap_or(core::mem::zeroed()) as _, udeviceid, pwfx, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInPrepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    unsafe { waveInPrepareHeader(hwi, pwh as _, cbwh) }
}
#[inline]
pub unsafe fn waveInReset(hwi: HWAVEIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInReset(hwi : HWAVEIN) -> u32);
    unsafe { waveInReset(hwi) }
}
#[inline]
pub unsafe fn waveInStart(hwi: HWAVEIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInStart(hwi : HWAVEIN) -> u32);
    unsafe { waveInStart(hwi) }
}
#[inline]
pub unsafe fn waveInStop(hwi: HWAVEIN) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInStop(hwi : HWAVEIN) -> u32);
    unsafe { waveInStop(hwi) }
}
#[inline]
pub unsafe fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveInUnprepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    unsafe { waveInUnprepareHeader(hwi, pwh as _, cbwh) }
}
#[inline]
pub unsafe fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutBreakLoop(hwo : HWAVEOUT) -> u32);
    unsafe { waveOutBreakLoop(hwo) }
}
#[inline]
pub unsafe fn waveOutClose(hwo: HWAVEOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutClose(hwo : HWAVEOUT) -> u32);
    unsafe { waveOutClose(hwo) }
}
#[inline]
pub unsafe fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetDevCapsA(udeviceid : usize, pwoc : *mut WAVEOUTCAPSA, cbwoc : u32) -> u32);
    unsafe { waveOutGetDevCapsA(udeviceid, pwoc as _, cbwoc) }
}
#[inline]
pub unsafe fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetDevCapsW(udeviceid : usize, pwoc : *mut WAVEOUTCAPSW, cbwoc : u32) -> u32);
    unsafe { waveOutGetDevCapsW(udeviceid, pwoc as _, cbwoc) }
}
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    unsafe { waveOutGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    unsafe { waveOutGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetID(hwo : HWAVEOUT, pudeviceid : *mut u32) -> u32);
    unsafe { waveOutGetID(hwo, pudeviceid as _) }
}
#[inline]
pub unsafe fn waveOutGetNumDevs() -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetNumDevs() -> u32);
    unsafe { waveOutGetNumDevs() }
}
#[inline]
pub unsafe fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPitch(hwo : HWAVEOUT, pdwpitch : *mut u32) -> u32);
    unsafe { waveOutGetPitch(hwo, pdwpitch as _) }
}
#[inline]
pub unsafe fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPlaybackRate(hwo : HWAVEOUT, pdwrate : *mut u32) -> u32);
    unsafe { waveOutGetPlaybackRate(hwo, pdwrate as _) }
}
#[inline]
pub unsafe fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetPosition(hwo : HWAVEOUT, pmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
    unsafe { waveOutGetPosition(hwo, pmmt as _, cbmmt) }
}
#[inline]
pub unsafe fn waveOutGetVolume(hwo: Option<HWAVEOUT>, pdwvolume: *mut u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutGetVolume(hwo : HWAVEOUT, pdwvolume : *mut u32) -> u32);
    unsafe { waveOutGetVolume(hwo.unwrap_or(core::mem::zeroed()) as _, pdwvolume as _) }
}
#[inline]
pub unsafe fn waveOutMessage(hwo: Option<HWAVEOUT>, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutMessage(hwo : HWAVEOUT, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    unsafe { waveOutMessage(hwo.unwrap_or(core::mem::zeroed()) as _, umsg, dw1, dw2) }
}
#[inline]
pub unsafe fn waveOutOpen(phwo: Option<*mut HWAVEOUT>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: Option<usize>, dwinstance: Option<usize>, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutOpen(phwo : *mut HWAVEOUT, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    unsafe { waveOutOpen(phwo.unwrap_or(core::mem::zeroed()) as _, udeviceid, pwfx, dwcallback.unwrap_or(core::mem::zeroed()) as _, dwinstance.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn waveOutPause(hwo: HWAVEOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutPause(hwo : HWAVEOUT) -> u32);
    unsafe { waveOutPause(hwo) }
}
#[inline]
pub unsafe fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutPrepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    unsafe { waveOutPrepareHeader(hwo, pwh as _, cbwh) }
}
#[inline]
pub unsafe fn waveOutReset(hwo: HWAVEOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutReset(hwo : HWAVEOUT) -> u32);
    unsafe { waveOutReset(hwo) }
}
#[inline]
pub unsafe fn waveOutRestart(hwo: HWAVEOUT) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutRestart(hwo : HWAVEOUT) -> u32);
    unsafe { waveOutRestart(hwo) }
}
#[inline]
pub unsafe fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutSetPitch(hwo : HWAVEOUT, dwpitch : u32) -> u32);
    unsafe { waveOutSetPitch(hwo, dwpitch) }
}
#[inline]
pub unsafe fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutSetPlaybackRate(hwo : HWAVEOUT, dwrate : u32) -> u32);
    unsafe { waveOutSetPlaybackRate(hwo, dwrate) }
}
#[inline]
pub unsafe fn waveOutSetVolume(hwo: Option<HWAVEOUT>, dwvolume: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutSetVolume(hwo : HWAVEOUT, dwvolume : u32) -> u32);
    unsafe { waveOutSetVolume(hwo.unwrap_or(core::mem::zeroed()) as _, dwvolume) }
}
#[inline]
pub unsafe fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutUnprepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    unsafe { waveOutUnprepareHeader(hwo, pwh as _, cbwh) }
}
#[inline]
pub unsafe fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn waveOutWrite(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    unsafe { waveOutWrite(hwo, pwh as _, cbwh) }
}
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
pub const ACMDM_USER: u32 = 16384u32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [i8; 32],
    pub szLongName: [i8; 128],
    pub szCopyright: [i8; 80],
    pub szLicensing: [i8; 128],
    pub szFeatures: [i8; 512],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
pub type ACMDRIVERENUMCB = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub cbwfxSrc: u32,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub cbwfxDst: u32,
}
impl Default for ACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: windows_core::PCSTR,
    pub pszAliasName: windows_core::PCSTR,
    pub dnDevNode: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ACMDRVOPENDESCW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: windows_core::PCWSTR,
    pub pszAliasName: windows_core::PCWSTR,
    pub dnDevNode: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMDRVSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub fdwConvert: u32,
    pub padshNext: *mut ACMDRVSTREAMHEADER,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub fdwPrepared: u32,
    pub dwPrepared: usize,
    pub pbPreparedSrc: *mut u8,
    pub cbPreparedSrcLength: u32,
    pub pbPreparedDst: *mut u8,
    pub cbPreparedDstLength: u32,
}
impl Default for ACMDRVSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMDRVSTREAMINSTANCE {
    pub cbStruct: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub pwfltr: *mut WAVEFILTER,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub fdwOpen: u32,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub has: HACMSTREAM,
}
impl Default for ACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
pub const ACMERR_BASE: u32 = 512u32;
pub const ACMERR_BUSY: u32 = 513u32;
pub const ACMERR_CANCELED: u32 = 515u32;
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: windows_core::PCSTR,
    pub szFilterTag: [i8; 48],
    pub szFilter: [i8; 128],
    pub pszName: windows_core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
impl Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ACMFILTERCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type ACMFILTERCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: windows_core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
impl Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [i8; 128],
}
impl Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
impl Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
pub type ACMFILTERENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub type ACMFILTERENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [i8; 48],
}
impl Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
pub type ACMFILTERTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub type ACMFILTERTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_core::PCSTR,
    pub szFormatTag: [i8; 48],
    pub szFormat: [i8; 128],
    pub pszName: windows_core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
impl Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ACMFORMATCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type ACMFORMATCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: windows_core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
impl Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [i8; 128],
}
impl Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
pub type ACMFORMATENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub type ACMFORMATENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [i8; 48],
}
impl Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
pub type ACMFORMATTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub type ACMFORMATTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_core::BOOL>;
pub const ACMHELPMSGCONTEXTHELP: windows_core::PCWSTR = windows_core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPA: windows_core::PCSTR = windows_core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPW: windows_core::PCWSTR = windows_core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTMENU: windows_core::PCWSTR = windows_core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUA: windows_core::PCSTR = windows_core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUW: windows_core::PCWSTR = windows_core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGSTRING: windows_core::PCWSTR = windows_core::w!("acmchoose_help");
pub const ACMHELPMSGSTRINGA: windows_core::PCSTR = windows_core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("acmchoose_help");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 10],
}
#[cfg(target_arch = "x86")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 15],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMBISONICS_NORMALIZATION(pub i32);
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: *mut u32,
}
impl Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AMBISONICS_TYPE(pub i32);
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(1i32);
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(4i32);
pub const AUDCLNT_E_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x88890002_u32 as _);
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: windows_core::HRESULT = windows_core::HRESULT(0x88890013_u32 as _);
pub const AUDCLNT_E_BUFFER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88890018_u32 as _);
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x8889000B_u32 as _);
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88890016_u32 as _);
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: windows_core::HRESULT = windows_core::HRESULT(0x88890019_u32 as _);
pub const AUDCLNT_E_BUFFER_TOO_LARGE: windows_core::HRESULT = windows_core::HRESULT(0x88890006_u32 as _);
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x88890017_u32 as _);
pub const AUDCLNT_E_DEVICE_INVALIDATED: windows_core::HRESULT = windows_core::HRESULT(0x88890004_u32 as _);
pub const AUDCLNT_E_DEVICE_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8889000A_u32 as _);
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88890041_u32 as _);
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x88890042_u32 as _);
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8889000F_u32 as _);
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: windows_core::HRESULT = windows_core::HRESULT(0x88890022_u32 as _);
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x88890029_u32 as _);
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: windows_core::HRESULT = windows_core::HRESULT(0x88890028_u32 as _);
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x88890011_u32 as _);
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x88890014_u32 as _);
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8889000E_u32 as _);
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x88890012_u32 as _);
pub const AUDCLNT_E_HEADTRACKING_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0x88890030_u32 as _);
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88890040_u32 as _);
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x88890015_u32 as _);
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: windows_core::HRESULT = windows_core::HRESULT(0x88890020_u32 as _);
pub const AUDCLNT_E_INVALID_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x88890009_u32 as _);
pub const AUDCLNT_E_INVALID_STREAM_FLAG: windows_core::HRESULT = windows_core::HRESULT(0x88890021_u32 as _);
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x88890025_u32 as _);
pub const AUDCLNT_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x88890001_u32 as _);
pub const AUDCLNT_E_NOT_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x88890005_u32 as _);
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x88890024_u32 as _);
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0x88890023_u32 as _);
pub const AUDCLNT_E_OUT_OF_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x88890007_u32 as _);
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88890027_u32 as _);
pub const AUDCLNT_E_RESOURCES_INVALIDATED: windows_core::HRESULT = windows_core::HRESULT(0x88890026_u32 as _);
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x88890010_u32 as _);
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8889000C_u32 as _);
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x88890008_u32 as _);
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x88890003_u32 as _);
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDCLNT_SHAREMODE(pub i32);
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDCLNT_STREAMOPTIONS(pub i32);
impl AUDCLNT_STREAMOPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AUDCLNT_STREAMOPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AUDCLNT_STREAMOPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4i32);
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2i32);
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0i32);
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1i32);
pub const AUDCLNT_S_BUFFER_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x8890001_u32 as _);
pub const AUDCLNT_S_POSITION_STALLED: windows_core::HRESULT = windows_core::HRESULT(0x8890003_u32 as _);
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8890002_u32 as _);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl Default for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}
impl Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(0i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIO_DUCKING_OPTIONS(pub i32);
impl AUDIO_DUCKING_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AUDIO_DUCKING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AUDIO_DUCKING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0i32);
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUDIO_EFFECT {
    pub id: windows_core::GUID,
    pub canSetState: windows_core::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIO_EFFECT_STATE(pub i32);
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIO_STREAM_CATEGORY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(pub i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(0i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(3i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(1i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: windows_core::GUID,
    pub bMuted: windows_core::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
impl Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct AUXCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct AUXCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AUXCAPS_AUXIN: u32 = 2u32;
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
pub const AUXCAPS_VOLUME: u32 = 1u32;
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(4i32);
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(3i32);
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(12i32);
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(1i32);
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(8i32);
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(6i32);
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(7i32);
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(11i32);
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(10i32);
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(0i32);
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(5i32);
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(9i32);
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(13i32);
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(14i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: windows_core::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: core::mem::ManuallyDrop<Option<IMMDevice>>,
    pub pPnpInterface: core::mem::ManuallyDrop<Option<IMMDevice>>,
    pub pPnpDevnode: core::mem::ManuallyDrop<Option<IMMDevice>>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AudioObjectType(pub i32);
impl AudioObjectType {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AudioObjectType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AudioObjectType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AudioObjectType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AudioObjectType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AudioObjectType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AudioObjectType_BackCenter: AudioObjectType = AudioObjectType(131072i32);
pub const AudioObjectType_BackLeft: AudioObjectType = AudioObjectType(128i32);
pub const AudioObjectType_BackRight: AudioObjectType = AudioObjectType(256i32);
pub const AudioObjectType_BottomBackLeft: AudioObjectType = AudioObjectType(32768i32);
pub const AudioObjectType_BottomBackRight: AudioObjectType = AudioObjectType(65536i32);
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = AudioObjectType(8192i32);
pub const AudioObjectType_BottomFrontRight: AudioObjectType = AudioObjectType(16384i32);
pub const AudioObjectType_Dynamic: AudioObjectType = AudioObjectType(1i32);
pub const AudioObjectType_FrontCenter: AudioObjectType = AudioObjectType(8i32);
pub const AudioObjectType_FrontLeft: AudioObjectType = AudioObjectType(2i32);
pub const AudioObjectType_FrontRight: AudioObjectType = AudioObjectType(4i32);
pub const AudioObjectType_LowFrequency: AudioObjectType = AudioObjectType(16i32);
pub const AudioObjectType_None: AudioObjectType = AudioObjectType(0i32);
pub const AudioObjectType_SideLeft: AudioObjectType = AudioObjectType(32i32);
pub const AudioObjectType_SideRight: AudioObjectType = AudioObjectType(64i32);
pub const AudioObjectType_TopBackLeft: AudioObjectType = AudioObjectType(2048i32);
pub const AudioObjectType_TopBackRight: AudioObjectType = AudioObjectType(4096i32);
pub const AudioObjectType_TopFrontLeft: AudioObjectType = AudioObjectType(512i32);
pub const AudioObjectType_TopFrontRight: AudioObjectType = AudioObjectType(1024i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AudioSessionDisconnectReason(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AudioSessionState(pub i32);
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AudioStateMonitorSoundLevel(pub i32);
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
pub const Connector: PartType = PartType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVICE_STATE(pub u32);
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
pub const DEVICE_STATE_ACTIVE: DEVICE_STATE = DEVICE_STATE(1u32);
pub const DEVICE_STATE_DISABLED: DEVICE_STATE = DEVICE_STATE(2u32);
pub const DEVICE_STATE_NOTPRESENT: DEVICE_STATE = DEVICE_STATE(4u32);
pub const DEVICE_STATE_UNPLUGGED: DEVICE_STATE = DEVICE_STATE(8u32);
pub const DEVINTERFACE_AUDIO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: windows_core::GUID = windows_core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: windows_core::GUID = windows_core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: windows_core::GUID = windows_core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: windows_core::GUID,
    pub dwAudioStreamFlags: u32,
}
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataFlow(pub i32);
pub const DeviceTopology: windows_core::GUID = windows_core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = AudioSessionDisconnectReason(0i32);
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = AudioSessionDisconnectReason(5i32);
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = AudioSessionDisconnectReason(2i32);
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = AudioSessionDisconnectReason(1i32);
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = AudioSessionDisconnectReason(4i32);
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = AudioSessionDisconnectReason(3i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EDataFlow(pub i32);
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ERole(pub i32);
pub const ERole_enum_count: ERole = ERole(3i32);
pub const EVENTCONTEXT_VOLUMESLIDER: windows_core::GUID = windows_core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EndpointFormFactor(pub i32);
pub const EndpointFormFactor_enum_count: EndpointFormFactor = EndpointFormFactor(11i32);
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
pub const Full: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMDRIVER(pub *mut core::ffi::c_void);
impl HACMDRIVER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HACMDRIVER {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("msacm32.dll" "system" fn acmDriverClose(had : *mut core::ffi::c_void, fdwclose : u32) -> u32);
            unsafe {
                acmDriverClose(self.0, 0);
            }
        }
    }
}
impl Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMDRIVERID(pub *mut core::ffi::c_void);
impl HACMDRIVERID {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMOBJ(pub *mut core::ffi::c_void);
impl HACMOBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HACMOBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HACMSTREAM(pub *mut core::ffi::c_void);
impl HACMSTREAM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HACMSTREAM {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("msacm32.dll" "system" fn acmStreamClose(has : *mut core::ffi::c_void, fdwclose : u32) -> u32);
            unsafe {
                acmStreamClose(self.0, 0);
            }
        }
    }
}
impl Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDI(pub *mut core::ffi::c_void);
impl HMIDI {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HMIDI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDIIN(pub *mut core::ffi::c_void);
impl HMIDIIN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HMIDIIN {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("winmm.dll" "system" fn midiInClose(hmi : *mut core::ffi::c_void) -> u32);
            unsafe {
                midiInClose(self.0);
            }
        }
    }
}
impl Default for HMIDIIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDIOUT(pub *mut core::ffi::c_void);
impl HMIDIOUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HMIDIOUT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("winmm.dll" "system" fn midiOutClose(hmo : *mut core::ffi::c_void) -> u32);
            unsafe {
                midiOutClose(self.0);
            }
        }
    }
}
impl Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIDISTRM(pub *mut core::ffi::c_void);
impl HMIDISTRM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HMIDISTRM {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("winmm.dll" "system" fn midiStreamClose(hms : *mut core::ffi::c_void) -> u32);
            unsafe {
                midiStreamClose(self.0);
            }
        }
    }
}
impl Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIXER(pub *mut core::ffi::c_void);
impl HMIXER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HMIXER {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("winmm.dll" "system" fn mixerClose(hmx : *mut core::ffi::c_void) -> u32);
            unsafe {
                mixerClose(self.0);
            }
        }
    }
}
impl Default for HMIXER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMIXEROBJ(pub *mut core::ffi::c_void);
impl HMIXEROBJ {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWAVE(pub *mut core::ffi::c_void);
impl HWAVE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HWAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWAVEIN(pub *mut core::ffi::c_void);
impl HWAVEIN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HWAVEIN {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("winmm.dll" "system" fn waveInClose(hwi : *mut core::ffi::c_void) -> u32);
            unsafe {
                waveInClose(self.0);
            }
        }
    }
}
impl Default for HWAVEIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HWAVEOUT(pub *mut core::ffi::c_void);
impl HWAVEOUT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HWAVEOUT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_core::link!("winmm.dll" "system" fn waveOutClose(hwo : *mut core::ffi::c_void) -> u32);
            unsafe {
                waveOutClose(self.0);
            }
        }
    }
}
impl Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
windows_core::imp::define_interface!(IAcousticEchoCancellationControl, IAcousticEchoCancellationControl_Vtbl, 0xf4ae25b5_aaa3_437d_b6b3_dbbe2d0e9549);
windows_core::imp::interface_hierarchy!(IAcousticEchoCancellationControl, windows_core::IUnknown);
impl IAcousticEchoCancellationControl {
    pub unsafe fn SetEchoCancellationRenderEndpoint<P0>(&self, endpointid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEchoCancellationRenderEndpoint)(windows_core::Interface::as_raw(self), endpointid.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcousticEchoCancellationControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetEchoCancellationRenderEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAcousticEchoCancellationControl_Impl: windows_core::IUnknownImpl {
    fn SetEchoCancellationRenderEndpoint(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAcousticEchoCancellationControl_Vtbl {
    pub const fn new<Identity: IAcousticEchoCancellationControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEchoCancellationRenderEndpoint<Identity: IAcousticEchoCancellationControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAcousticEchoCancellationControl_Impl::SetEchoCancellationRenderEndpoint(this, core::mem::transmute(&endpointid)).into()
            }
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
impl windows_core::RuntimeName for IAcousticEchoCancellationControl {}
windows_core::imp::define_interface!(IActivateAudioInterfaceAsyncOperation, IActivateAudioInterfaceAsyncOperation_Vtbl, 0x72a22d78_cde4_431d_b8cc_843a71199b6d);
windows_core::imp::interface_hierarchy!(IActivateAudioInterfaceAsyncOperation, windows_core::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    pub unsafe fn GetActivateResult(&self, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetActivateResult)(windows_core::Interface::as_raw(self), activateresult as _, core::mem::transmute(activatedinterface)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivateResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActivateAudioInterfaceAsyncOperation_Impl: windows_core::IUnknownImpl {
    fn GetActivateResult(&self, activateresult: *mut windows_core::HRESULT, activatedinterface: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub const fn new<Identity: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActivateResult<Identity: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActivateAudioInterfaceAsyncOperation_Impl::GetActivateResult(this, core::mem::transmute_copy(&activateresult), core::mem::transmute_copy(&activatedinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivateResult: GetActivateResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceAsyncOperation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivateAudioInterfaceAsyncOperation {}
windows_core::imp::define_interface!(IActivateAudioInterfaceCompletionHandler, IActivateAudioInterfaceCompletionHandler_Vtbl, 0x41d949ab_9862_444a_80f6_c261334da5eb);
windows_core::imp::interface_hierarchy!(IActivateAudioInterfaceCompletionHandler, windows_core::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    pub unsafe fn ActivateCompleted<P0>(&self, activateoperation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActivateAudioInterfaceAsyncOperation>,
    {
        unsafe { (windows_core::Interface::vtable(self).ActivateCompleted)(windows_core::Interface::as_raw(self), activateoperation.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActivateAudioInterfaceCompletionHandler_Impl: windows_core::IUnknownImpl {
    fn ActivateCompleted(&self, activateoperation: windows_core::Ref<IActivateAudioInterfaceAsyncOperation>) -> windows_core::Result<()>;
}
impl IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub const fn new<Identity: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateCompleted<Identity: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activateoperation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActivateAudioInterfaceCompletionHandler_Impl::ActivateCompleted(this, core::mem::transmute_copy(&activateoperation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ActivateCompleted: ActivateCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceCompletionHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivateAudioInterfaceCompletionHandler {}
windows_core::imp::define_interface!(IAudioAmbisonicsControl, IAudioAmbisonicsControl_Vtbl, 0x28724c91_df35_4856_9f76_d6a26413f3df);
windows_core::imp::interface_hierarchy!(IAudioAmbisonicsControl, windows_core::IUnknown);
impl IAudioAmbisonicsControl {
    pub unsafe fn SetData(&self, pambisonicsparams: &[AMBISONICS_PARAMS]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute(pambisonicsparams.as_ptr()), pambisonicsparams.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetHeadTracking(&self, benableheadtracking: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHeadTracking)(windows_core::Interface::as_raw(self), benableheadtracking.into()).ok() }
    }
    pub unsafe fn GetHeadTracking(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHeadTracking)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), x, y, z, w).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAmbisonicsControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const AMBISONICS_PARAMS, u32) -> windows_core::HRESULT,
    pub SetHeadTracking: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetHeadTracking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub trait IAudioAmbisonicsControl_Impl: windows_core::IUnknownImpl {
    fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::Result<()>;
    fn SetHeadTracking(&self, benableheadtracking: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetHeadTracking(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> windows_core::Result<()>;
}
impl IAudioAmbisonicsControl_Vtbl {
    pub const fn new<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetData<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAmbisonicsControl_Impl::SetData(this, core::mem::transmute_copy(&pambisonicsparams), core::mem::transmute_copy(&cbambisonicsparams)).into()
            }
        }
        unsafe extern "system" fn SetHeadTracking<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benableheadtracking: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAmbisonicsControl_Impl::SetHeadTracking(this, core::mem::transmute_copy(&benableheadtracking)).into()
            }
        }
        unsafe extern "system" fn GetHeadTracking<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenableheadtracking: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioAmbisonicsControl_Impl::GetHeadTracking(this) {
                    Ok(ok__) => {
                        pbenableheadtracking.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAmbisonicsControl_Impl::SetRotation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&w)).into()
            }
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
impl windows_core::RuntimeName for IAudioAmbisonicsControl {}
windows_core::imp::define_interface!(IAudioAutoGainControl, IAudioAutoGainControl_Vtbl, 0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
windows_core::imp::interface_hierarchy!(IAudioAutoGainControl, windows_core::IUnknown);
impl IAudioAutoGainControl {
    pub unsafe fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, benable: bool, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioAutoGainControl_Impl: windows_core::IUnknownImpl {
    fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnabled(&self, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioAutoGainControl_Vtbl {
    pub const fn new<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabled<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioAutoGainControl_Impl::GetEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAutoGainControl_Impl::SetEnabled(this, core::mem::transmute_copy(&benable), core::mem::transmute_copy(&pguideventcontext)).into()
            }
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
impl windows_core::RuntimeName for IAudioAutoGainControl {}
windows_core::imp::define_interface!(IAudioBass, IAudioBass_Vtbl, 0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
impl core::ops::Deref for IAudioBass {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioBass, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBass_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
pub trait IAudioBass_Impl: IPerChannelDbLevel_Impl {}
impl IAudioBass_Vtbl {
    pub const fn new<Identity: IAudioBass_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioBass as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioBass {}
windows_core::imp::define_interface!(IAudioCaptureClient, IAudioCaptureClient_Vtbl, 0xc8adbd64_e71e_48a0_a4de_185c395cd317);
windows_core::imp::interface_hierarchy!(IAudioCaptureClient, windows_core::IUnknown);
impl IAudioCaptureClient {
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: Option<*mut u64>, pu64qpcposition: Option<*mut u64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppdata as _, pnumframestoread as _, pdwflags as _, pu64deviceposition.unwrap_or(core::mem::zeroed()) as _, pu64qpcposition.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframesread).ok() }
    }
    pub unsafe fn GetNextPacketSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNextPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioCaptureClient_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> windows_core::Result<()>;
    fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::Result<()>;
    fn GetNextPacketSize(&self) -> windows_core::Result<u32>;
}
impl IAudioCaptureClient_Vtbl {
    pub const fn new<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioCaptureClient_Impl::GetBuffer(this, core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pnumframestoread), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pu64deviceposition), core::mem::transmute_copy(&pu64qpcposition)).into()
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframesread: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioCaptureClient_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&numframesread)).into()
            }
        }
        unsafe extern "system" fn GetNextPacketSize<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioCaptureClient_Impl::GetNextPacketSize(this) {
                    Ok(ok__) => {
                        pnumframesinnextpacket.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IAudioCaptureClient {}
windows_core::imp::define_interface!(IAudioChannelConfig, IAudioChannelConfig_Vtbl, 0xbb11c46f_ec28_493c_b88a_5db88062ce98);
windows_core::imp::interface_hierarchy!(IAudioChannelConfig, windows_core::IUnknown);
impl IAudioChannelConfig {
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelConfig)(windows_core::Interface::as_raw(self), dwconfig, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetChannelConfig(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioChannelConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioChannelConfig_Impl: windows_core::IUnknownImpl {
    fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelConfig(&self) -> windows_core::Result<u32>;
}
impl IAudioChannelConfig_Vtbl {
    pub const fn new<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetChannelConfig<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfig: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioChannelConfig_Impl::SetChannelConfig(this, core::mem::transmute_copy(&dwconfig), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelConfig<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconfig: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioChannelConfig_Impl::GetChannelConfig(this) {
                    Ok(ok__) => {
                        pdwconfig.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IAudioChannelConfig {}
windows_core::imp::define_interface!(IAudioClient, IAudioClient_Vtbl, 0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
windows_core::imp::interface_hierarchy!(IAudioClient, windows_core::IUnknown);
impl IAudioClient {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, audiosessionguid.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetBufferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamLatency(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentPadding(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: Option<*mut *mut WAVEFORMATEX>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsFormatSupported)(windows_core::Interface::as_raw(self), sharemode, pformat, ppclosestmatch.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetMixFormat(&self) -> windows_core::Result<*mut WAVEFORMATEX> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMixFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: Option<*mut i64>, phnsminimumdeviceperiod: Option<*mut i64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDevicePeriod)(windows_core::Interface::as_raw(self), phnsdefaultdeviceperiod.unwrap_or(core::mem::zeroed()) as _, phnsminimumdeviceperiod.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetEventHandle(&self, eventhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), eventhandle).ok() }
    }
    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, AUDCLNT_SHAREMODE, u32, i64, i64, *const WAVEFORMATEX, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStreamLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetCurrentPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, AUDCLNT_SHAREMODE, *const WAVEFORMATEX, *mut *mut WAVEFORMATEX) -> windows_core::HRESULT,
    pub GetMixFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WAVEFORMATEX) -> windows_core::HRESULT,
    pub GetDevicePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut i64) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioClient_Impl: windows_core::IUnknownImpl {
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
impl IAudioClient_Vtbl {
    pub const fn new<Identity: IAudioClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Initialize(this, core::mem::transmute_copy(&sharemode), core::mem::transmute_copy(&streamflags), core::mem::transmute_copy(&hnsbufferduration), core::mem::transmute_copy(&hnsperiodicity), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&audiosessionguid)).into()
            }
        }
        unsafe extern "system" fn GetBufferSize<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumbufferframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetBufferSize(this) {
                    Ok(ok__) => {
                        pnumbufferframes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamLatency<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnslatency: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetStreamLatency(this) {
                    Ok(ok__) => {
                        phnslatency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPadding<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpaddingframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetCurrentPadding(this) {
                    Ok(ok__) => {
                        pnumpaddingframes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFormatSupported<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::IsFormatSupported(this, core::mem::transmute_copy(&sharemode), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&ppclosestmatch))
            }
        }
        unsafe extern "system" fn GetMixFormat<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetMixFormat(this) {
                    Ok(ok__) => {
                        ppdeviceformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevicePeriod<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::GetDevicePeriod(this, core::mem::transmute_copy(&phnsdefaultdeviceperiod), core::mem::transmute_copy(&phnsminimumdeviceperiod)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn SetEventHandle<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::SetEventHandle(this, core::mem::transmute_copy(&eventhandle)).into()
            }
        }
        unsafe extern "system" fn GetService<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::GetService(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
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
impl windows_core::RuntimeName for IAudioClient {}
windows_core::imp::define_interface!(IAudioClient2, IAudioClient2_Vtbl, 0x726778cd_f60a_4eda_82de_e47610cd78aa);
impl core::ops::Deref for IAudioClient2 {
    type Target = IAudioClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClient2, windows_core::IUnknown, IAudioClient);
impl IAudioClient2 {
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOffloadCapable)(windows_core::Interface::as_raw(self), category, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientProperties)(windows_core::Interface::as_raw(self), pproperties).ok() }
    }
    pub unsafe fn GetBufferSizeLimits(&self, pformat: *const WAVEFORMATEX, beventdriven: bool, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBufferSizeLimits)(windows_core::Interface::as_raw(self), pformat, beventdriven.into(), phnsminbufferduration as _, phnsmaxbufferduration as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient2_Vtbl {
    pub base__: IAudioClient_Vtbl,
    pub IsOffloadCapable: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_STREAM_CATEGORY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClientProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const AudioClientProperties) -> windows_core::HRESULT,
    pub GetBufferSizeLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX, windows_core::BOOL, *mut i64, *mut i64) -> windows_core::HRESULT,
}
pub trait IAudioClient2_Impl: IAudioClient_Impl {
    fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL>;
    fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> windows_core::Result<()>;
    fn GetBufferSizeLimits(&self, pformat: *const WAVEFORMATEX, beventdriven: windows_core::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> windows_core::Result<()>;
}
impl IAudioClient2_Vtbl {
    pub const fn new<Identity: IAudioClient2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsOffloadCapable<Identity: IAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient2_Impl::IsOffloadCapable(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        pboffloadcapable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientProperties<Identity: IAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperties: *const AudioClientProperties) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient2_Impl::SetClientProperties(this, core::mem::transmute_copy(&pproperties)).into()
            }
        }
        unsafe extern "system" fn GetBufferSizeLimits<Identity: IAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: windows_core::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient2_Impl::GetBufferSizeLimits(this, core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&beventdriven), core::mem::transmute_copy(&phnsminbufferduration), core::mem::transmute_copy(&phnsmaxbufferduration)).into()
            }
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
impl windows_core::RuntimeName for IAudioClient2 {}
windows_core::imp::define_interface!(IAudioClient3, IAudioClient3_Vtbl, 0x7ed4ee07_8e67_4cd4_8c1a_2b7a5987ad42);
impl core::ops::Deref for IAudioClient3 {
    type Target = IAudioClient2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClient3, windows_core::IUnknown, IAudioClient, IAudioClient2);
impl IAudioClient3 {
    pub unsafe fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSharedModeEnginePeriod)(windows_core::Interface::as_raw(self), pformat, pdefaultperiodinframes as _, pfundamentalperiodinframes as _, pminperiodinframes as _, pmaxperiodinframes as _).ok() }
    }
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentSharedModeEnginePeriod)(windows_core::Interface::as_raw(self), ppformat as _, pcurrentperiodinframes as _).ok() }
    }
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InitializeSharedAudioStream)(windows_core::Interface::as_raw(self), streamflags, periodinframes, pformat, audiosessionguid.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient3_Vtbl {
    pub base__: IAudioClient2_Vtbl,
    pub GetSharedModeEnginePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentSharedModeEnginePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    pub InitializeSharedAudioStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const WAVEFORMATEX, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioClient3_Impl: IAudioClient2_Impl {
    fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::Result<()>;
    fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioClient3_Vtbl {
    pub const fn new<Identity: IAudioClient3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Identity: IAudioClient3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient3_Impl::GetSharedModeEnginePeriod(this, core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pdefaultperiodinframes), core::mem::transmute_copy(&pfundamentalperiodinframes), core::mem::transmute_copy(&pminperiodinframes), core::mem::transmute_copy(&pmaxperiodinframes)).into()
            }
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Identity: IAudioClient3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient3_Impl::GetCurrentSharedModeEnginePeriod(this, core::mem::transmute_copy(&ppformat), core::mem::transmute_copy(&pcurrentperiodinframes)).into()
            }
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Identity: IAudioClient3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient3_Impl::InitializeSharedAudioStream(this, core::mem::transmute_copy(&streamflags), core::mem::transmute_copy(&periodinframes), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&audiosessionguid)).into()
            }
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
impl windows_core::RuntimeName for IAudioClient3 {}
windows_core::imp::define_interface!(IAudioClientDuckingControl, IAudioClientDuckingControl_Vtbl, 0xc789d381_a28c_4168_b28f_d3a837924dc3);
windows_core::imp::interface_hierarchy!(IAudioClientDuckingControl, windows_core::IUnknown);
impl IAudioClientDuckingControl {
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDuckingOptionsForCurrentStream)(windows_core::Interface::as_raw(self), options).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClientDuckingControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDuckingOptionsForCurrentStream: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT,
}
pub trait IAudioClientDuckingControl_Impl: windows_core::IUnknownImpl {
    fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> windows_core::Result<()>;
}
impl IAudioClientDuckingControl_Vtbl {
    pub const fn new<Identity: IAudioClientDuckingControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Identity: IAudioClientDuckingControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClientDuckingControl_Impl::SetDuckingOptionsForCurrentStream(this, core::mem::transmute_copy(&options)).into()
            }
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
impl windows_core::RuntimeName for IAudioClientDuckingControl {}
windows_core::imp::define_interface!(IAudioClock, IAudioClock_Vtbl, 0xcd63314f_3fba_4a1b_812c_ef96358728e7);
windows_core::imp::interface_hierarchy!(IAudioClock, windows_core::IUnknown);
impl IAudioClock {
    pub unsafe fn GetFrequency(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: Option<*mut u64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), pu64position as _, pu64qpcposition.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioClock_Impl: windows_core::IUnknownImpl {
    fn GetFrequency(&self) -> windows_core::Result<u64>;
    fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> windows_core::Result<()>;
    fn GetCharacteristics(&self) -> windows_core::Result<u32>;
}
impl IAudioClock_Vtbl {
    pub const fn new<Identity: IAudioClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrequency<Identity: IAudioClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu64frequency: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClock_Impl::GetFrequency(this) {
                    Ok(ok__) => {
                        pu64frequency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IAudioClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClock_Impl::GetPosition(this, core::mem::transmute_copy(&pu64position), core::mem::transmute_copy(&pu64qpcposition)).into()
            }
        }
        unsafe extern "system" fn GetCharacteristics<Identity: IAudioClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClock_Impl::GetCharacteristics(this) {
                    Ok(ok__) => {
                        pdwcharacteristics.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IAudioClock {}
windows_core::imp::define_interface!(IAudioClock2, IAudioClock2_Vtbl, 0x6f49ff73_6727_49ac_a008_d98cf5e70048);
windows_core::imp::interface_hierarchy!(IAudioClock2, windows_core::IUnknown);
impl IAudioClock2 {
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: Option<*mut u64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDevicePosition)(windows_core::Interface::as_raw(self), deviceposition as _, qpcposition.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevicePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
}
pub trait IAudioClock2_Impl: windows_core::IUnknownImpl {
    fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> windows_core::Result<()>;
}
impl IAudioClock2_Vtbl {
    pub const fn new<Identity: IAudioClock2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevicePosition<Identity: IAudioClock2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClock2_Impl::GetDevicePosition(this, core::mem::transmute_copy(&deviceposition), core::mem::transmute_copy(&qpcposition)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDevicePosition: GetDevicePosition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClock2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioClock2 {}
windows_core::imp::define_interface!(IAudioClockAdjustment, IAudioClockAdjustment_Vtbl, 0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
windows_core::imp::interface_hierarchy!(IAudioClockAdjustment, windows_core::IUnknown);
impl IAudioClockAdjustment {
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSampleRate)(windows_core::Interface::as_raw(self), flsamplerate).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClockAdjustment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSampleRate: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IAudioClockAdjustment_Impl: windows_core::IUnknownImpl {
    fn SetSampleRate(&self, flsamplerate: f32) -> windows_core::Result<()>;
}
impl IAudioClockAdjustment_Vtbl {
    pub const fn new<Identity: IAudioClockAdjustment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSampleRate<Identity: IAudioClockAdjustment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flsamplerate: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClockAdjustment_Impl::SetSampleRate(this, core::mem::transmute_copy(&flsamplerate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSampleRate: SetSampleRate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClockAdjustment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioClockAdjustment {}
windows_core::imp::define_interface!(IAudioEffectsChangedNotificationClient, IAudioEffectsChangedNotificationClient_Vtbl, 0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
windows_core::imp::interface_hierarchy!(IAudioEffectsChangedNotificationClient, windows_core::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    pub unsafe fn OnAudioEffectsChanged(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAudioEffectsChanged)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsChangedNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAudioEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioEffectsChangedNotificationClient_Impl: windows_core::IUnknownImpl {
    fn OnAudioEffectsChanged(&self) -> windows_core::Result<()>;
}
impl IAudioEffectsChangedNotificationClient_Vtbl {
    pub const fn new<Identity: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAudioEffectsChanged<Identity: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsChangedNotificationClient_Impl::OnAudioEffectsChanged(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAudioEffectsChanged: OnAudioEffectsChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEffectsChangedNotificationClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEffectsChangedNotificationClient {}
windows_core::imp::define_interface!(IAudioEffectsManager, IAudioEffectsManager_Vtbl, 0x4460b3ae_4b44_4527_8676_7548a8acd260);
windows_core::imp::interface_hierarchy!(IAudioEffectsManager, windows_core::IUnknown);
impl IAudioEffectsManager {
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEffectsChangedNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterAudioEffectsChangedNotificationCallback)(windows_core::Interface::as_raw(self), client.param().abi()).ok() }
    }
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEffectsChangedNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAudioEffectsChangedNotificationCallback)(windows_core::Interface::as_raw(self), client.param().abi()).ok() }
    }
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAudioEffects)(windows_core::Interface::as_raw(self), effects as _, numeffects as _).ok() }
    }
    pub unsafe fn SetAudioEffectState(&self, effectid: windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAudioEffectState)(windows_core::Interface::as_raw(self), core::mem::transmute(effectid), state).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAudioEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut AUDIO_EFFECT, *mut u32) -> windows_core::HRESULT,
    pub SetAudioEffectState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, AUDIO_EFFECT_STATE) -> windows_core::HRESULT,
}
pub trait IAudioEffectsManager_Impl: windows_core::IUnknownImpl {
    fn RegisterAudioEffectsChangedNotificationCallback(&self, client: windows_core::Ref<IAudioEffectsChangedNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterAudioEffectsChangedNotificationCallback(&self, client: windows_core::Ref<IAudioEffectsChangedNotificationClient>) -> windows_core::Result<()>;
    fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::Result<()>;
    fn SetAudioEffectState(&self, effectid: &windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::Result<()>;
}
impl IAudioEffectsManager_Vtbl {
    pub const fn new<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, client: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::RegisterAudioEffectsChangedNotificationCallback(this, core::mem::transmute_copy(&client)).into()
            }
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, client: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::UnregisterAudioEffectsChangedNotificationCallback(this, core::mem::transmute_copy(&client)).into()
            }
        }
        unsafe extern "system" fn GetAudioEffects<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::GetAudioEffects(this, core::mem::transmute_copy(&effects), core::mem::transmute_copy(&numeffects)).into()
            }
        }
        unsafe extern "system" fn SetAudioEffectState<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::SetAudioEffectState(this, core::mem::transmute(&effectid), core::mem::transmute_copy(&state)).into()
            }
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
impl windows_core::RuntimeName for IAudioEffectsManager {}
windows_core::imp::define_interface!(IAudioFormatEnumerator, IAudioFormatEnumerator_Vtbl, 0xdcdaa858_895a_4a22_a5eb_67bda506096d);
windows_core::imp::interface_hierarchy!(IAudioFormatEnumerator, windows_core::IUnknown);
impl IAudioFormatEnumerator {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFormat(&self, index: u32) -> windows_core::Result<*mut WAVEFORMATEX> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFormatEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut WAVEFORMATEX) -> windows_core::HRESULT,
}
pub trait IAudioFormatEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetFormat(&self, index: u32) -> windows_core::Result<*mut WAVEFORMATEX>;
}
impl IAudioFormatEnumerator_Vtbl {
    pub const fn new<Identity: IAudioFormatEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioFormatEnumerator_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioFormatEnumerator_Impl::GetFormat(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        format.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetFormat: GetFormat::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFormatEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioFormatEnumerator {}
windows_core::imp::define_interface!(IAudioInputSelector, IAudioInputSelector_Vtbl, 0x4f03dc02_5e6e_4653_8f72_a030c123d598);
windows_core::imp::interface_hierarchy!(IAudioInputSelector, windows_core::IUnknown);
impl IAudioInputSelector {
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioInputSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioInputSelector_Vtbl {
    pub const fn new<Identity: IAudioInputSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioInputSelector_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pnidselected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelection<Identity: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioInputSelector_Impl::SetSelection(this, core::mem::transmute_copy(&nidselect), core::mem::transmute_copy(&pguideventcontext)).into()
            }
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
impl windows_core::RuntimeName for IAudioInputSelector {}
windows_core::imp::define_interface!(IAudioLoudness, IAudioLoudness_Vtbl, 0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
windows_core::imp::interface_hierarchy!(IAudioLoudness, windows_core::IUnknown);
impl IAudioLoudness {
    pub unsafe fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, benable: bool, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioLoudness_Impl: windows_core::IUnknownImpl {
    fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnabled(&self, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioLoudness_Vtbl {
    pub const fn new<Identity: IAudioLoudness_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabled<Identity: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioLoudness_Impl::GetEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioLoudness_Impl::SetEnabled(this, core::mem::transmute_copy(&benable), core::mem::transmute_copy(&pguideventcontext)).into()
            }
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
impl windows_core::RuntimeName for IAudioLoudness {}
windows_core::imp::define_interface!(IAudioMidrange, IAudioMidrange_Vtbl, 0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
impl core::ops::Deref for IAudioMidrange {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioMidrange, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMidrange_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
pub trait IAudioMidrange_Impl: IPerChannelDbLevel_Impl {}
impl IAudioMidrange_Vtbl {
    pub const fn new<Identity: IAudioMidrange_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMidrange as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioMidrange {}
windows_core::imp::define_interface!(IAudioMute, IAudioMute_Vtbl, 0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
windows_core::imp::interface_hierarchy!(IAudioMute, windows_core::IUnknown);
impl IAudioMute {
    pub unsafe fn SetMute(&self, bmuted: bool, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmuted.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAudioMute_Impl: windows_core::IUnknownImpl {
    fn SetMute(&self, bmuted: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAudioMute_Vtbl {
    pub const fn new<Identity: IAudioMute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMute<Identity: IAudioMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmuted: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioMute_Impl::SetMute(this, core::mem::transmute_copy(&bmuted), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IAudioMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmuted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMute_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmuted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMute: SetMute::<Identity, OFFSET>, GetMute: GetMute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMute as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioMute {}
windows_core::imp::define_interface!(IAudioOutputSelector, IAudioOutputSelector_Vtbl, 0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
windows_core::imp::interface_hierarchy!(IAudioOutputSelector, windows_core::IUnknown);
impl IAudioOutputSelector {
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioOutputSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioOutputSelector_Vtbl {
    pub const fn new<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioOutputSelector_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pnidselected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelection<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioOutputSelector_Impl::SetSelection(this, core::mem::transmute_copy(&nidselect), core::mem::transmute_copy(&pguideventcontext)).into()
            }
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
impl windows_core::RuntimeName for IAudioOutputSelector {}
windows_core::imp::define_interface!(IAudioPeakMeter, IAudioPeakMeter_Vtbl, 0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
windows_core::imp::interface_hierarchy!(IAudioPeakMeter, windows_core::IUnknown);
impl IAudioPeakMeter {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioPeakMeter_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
}
impl IAudioPeakMeter_Vtbl {
    pub const fn new<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioPeakMeter_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pcchannels.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLevel<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioPeakMeter_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pflevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IAudioPeakMeter {}
windows_core::imp::define_interface!(IAudioRenderClient, IAudioRenderClient_Vtbl, 0xf294acfc_3146_4483_a7bf_addca7c260e2);
windows_core::imp::interface_hierarchy!(IAudioRenderClient, windows_core::IUnknown);
impl IAudioRenderClient {
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), numframesrequested, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframeswritten, dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IAudioRenderClient_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, numframesrequested: u32) -> windows_core::Result<*mut u8>;
    fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> windows_core::Result<()>;
}
impl IAudioRenderClient_Vtbl {
    pub const fn new<Identity: IAudioRenderClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioRenderClient_Impl::GetBuffer(this, core::mem::transmute_copy(&numframesrequested)) {
                    Ok(ok__) => {
                        ppdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioRenderClient_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&numframeswritten), core::mem::transmute_copy(&dwflags)).into()
            }
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
impl windows_core::RuntimeName for IAudioRenderClient {}
windows_core::imp::define_interface!(IAudioSessionControl, IAudioSessionControl_Vtbl, 0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
windows_core::imp::interface_hierarchy!(IAudioSessionControl, windows_core::IUnknown);
impl IAudioSessionControl {
    pub unsafe fn GetState(&self) -> windows_core::Result<AudioSessionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayName<P0>(&self, value: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), value.param().abi(), eventcontext).ok() }
    }
    pub unsafe fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIconPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIconPath<P0>(&self, value: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIconPath)(windows_core::Interface::as_raw(self), value.param().abi(), eventcontext).ok() }
    }
    pub unsafe fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroupingParam)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGroupingParam)(windows_core::Interface::as_raw(self), r#override, eventcontext).ok() }
    }
    pub unsafe fn RegisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterAudioSessionNotification)(windows_core::Interface::as_raw(self), newnotifications.param().abi()).ok() }
    }
    pub unsafe fn UnregisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAudioSessionNotification)(windows_core::Interface::as_raw(self), newnotifications.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioSessionState) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGroupingParam: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGroupingParam: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterAudioSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAudioSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionControl_Impl: windows_core::IUnknownImpl {
    fn GetState(&self) -> windows_core::Result<AudioSessionState>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDisplayName(&self, value: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIconPath(&self, value: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGroupingParam(&self, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterAudioSessionNotification(&self, newnotifications: windows_core::Ref<IAudioSessionEvents>) -> windows_core::Result<()>;
    fn UnregisterAudioSessionNotification(&self, newnotifications: windows_core::Ref<IAudioSessionEvents>) -> windows_core::Result<()>;
}
impl IAudioSessionControl_Vtbl {
    pub const fn new<Identity: IAudioSessionControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetState<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut AudioSessionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetState(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::SetDisplayName(this, core::mem::transmute(&value), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetIconPath<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetIconPath(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIconPath<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::SetIconPath(this, core::mem::transmute(&value), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetGroupingParam<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetGroupingParam(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupingParam<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::SetGroupingParam(this, core::mem::transmute_copy(&r#override), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newnotifications: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::RegisterAudioSessionNotification(this, core::mem::transmute_copy(&newnotifications)).into()
            }
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newnotifications: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::UnregisterAudioSessionNotification(this, core::mem::transmute_copy(&newnotifications)).into()
            }
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
impl windows_core::RuntimeName for IAudioSessionControl {}
windows_core::imp::define_interface!(IAudioSessionControl2, IAudioSessionControl2_Vtbl, 0xbfb7ff88_7239_4fc9_8fa2_07c950be9c6d);
impl core::ops::Deref for IAudioSessionControl2 {
    type Target = IAudioSessionControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionControl2, windows_core::IUnknown, IAudioSessionControl);
impl IAudioSessionControl2 {
    pub unsafe fn GetSessionIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionInstanceIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProcessId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSystemSoundsSession(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsSystemSoundsSession)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetDuckingPreference(&self, optout: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDuckingPreference)(windows_core::Interface::as_raw(self), optout.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl2_Vtbl {
    pub base__: IAudioSessionControl_Vtbl,
    pub GetSessionIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSessionInstanceIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsSystemSoundsSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuckingPreference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAudioSessionControl2_Impl: IAudioSessionControl_Impl {
    fn GetSessionIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSessionInstanceIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProcessId(&self) -> windows_core::Result<u32>;
    fn IsSystemSoundsSession(&self) -> windows_core::HRESULT;
    fn SetDuckingPreference(&self, optout: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IAudioSessionControl2_Vtbl {
    pub const fn new<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSessionIdentifier<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl2_Impl::GetSessionIdentifier(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl2_Impl::GetSessionInstanceIdentifier(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProcessId<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl2_Impl::GetProcessId(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSystemSoundsSession<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl2_Impl::IsSystemSoundsSession(this)
            }
        }
        unsafe extern "system" fn SetDuckingPreference<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optout: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl2_Impl::SetDuckingPreference(this, core::mem::transmute_copy(&optout)).into()
            }
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
impl windows_core::RuntimeName for IAudioSessionControl2 {}
windows_core::imp::define_interface!(IAudioSessionEnumerator, IAudioSessionEnumerator_Vtbl, 0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
windows_core::imp::interface_hierarchy!(IAudioSessionEnumerator, windows_core::IUnknown);
impl IAudioSessionEnumerator {
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSession(&self, sessioncount: i32) -> windows_core::Result<IAudioSessionControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSession)(windows_core::Interface::as_raw(self), sessioncount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetSession: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetSession(&self, sessioncount: i32) -> windows_core::Result<IAudioSessionControl>;
}
impl IAudioSessionEnumerator_Vtbl {
    pub const fn new<Identity: IAudioSessionEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessioncount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionEnumerator_Impl::GetCount(this) {
                    Ok(ok__) => {
                        sessioncount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSession<Identity: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessioncount: i32, session: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionEnumerator_Impl::GetSession(this, core::mem::transmute_copy(&sessioncount)) {
                    Ok(ok__) => {
                        session.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IAudioSessionEnumerator {}
windows_core::imp::define_interface!(IAudioSessionEvents, IAudioSessionEvents_Vtbl, 0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
windows_core::imp::interface_hierarchy!(IAudioSessionEvents, windows_core::IUnknown);
impl IAudioSessionEvents {
    pub unsafe fn OnDisplayNameChanged<P0>(&self, newdisplayname: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDisplayNameChanged)(windows_core::Interface::as_raw(self), newdisplayname.param().abi(), eventcontext).ok() }
    }
    pub unsafe fn OnIconPathChanged<P0>(&self, newiconpath: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIconPathChanged)(windows_core::Interface::as_raw(self), newiconpath.param().abi(), eventcontext).ok() }
    }
    pub unsafe fn OnSimpleVolumeChanged(&self, newvolume: f32, newmute: bool, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSimpleVolumeChanged)(windows_core::Interface::as_raw(self), newvolume, newmute.into(), eventcontext).ok() }
    }
    pub unsafe fn OnChannelVolumeChanged(&self, newchannelvolumearray: &[f32], changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnChannelVolumeChanged)(windows_core::Interface::as_raw(self), newchannelvolumearray.len().try_into().unwrap(), core::mem::transmute(newchannelvolumearray.as_ptr()), changedchannel, eventcontext).ok() }
    }
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnGroupingParamChanged)(windows_core::Interface::as_raw(self), newgroupingparam, eventcontext).ok() }
    }
    pub unsafe fn OnStateChanged(&self, newstate: AudioSessionState) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), newstate).ok() }
    }
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSessionDisconnected)(windows_core::Interface::as_raw(self), disconnectreason).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDisplayNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnIconPathChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnSimpleVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, f32, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnChannelVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnGroupingParamChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, AudioSessionState) -> windows_core::HRESULT,
    pub OnSessionDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, AudioSessionDisconnectReason) -> windows_core::HRESULT,
}
pub trait IAudioSessionEvents_Impl: windows_core::IUnknownImpl {
    fn OnDisplayNameChanged(&self, newdisplayname: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnIconPathChanged(&self, newiconpath: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnSimpleVolumeChanged(&self, newvolume: f32, newmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnChannelVolumeChanged(&self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnGroupingParamChanged(&self, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnStateChanged(&self, newstate: AudioSessionState) -> windows_core::Result<()>;
    fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> windows_core::Result<()>;
}
impl IAudioSessionEvents_Vtbl {
    pub const fn new<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDisplayNameChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newdisplayname: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnDisplayNameChanged(this, core::mem::transmute(&newdisplayname), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnIconPathChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newiconpath: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnIconPathChanged(this, core::mem::transmute(&newiconpath), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newvolume: f32, newmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnSimpleVolumeChanged(this, core::mem::transmute_copy(&newvolume), core::mem::transmute_copy(&newmute), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnChannelVolumeChanged(this, core::mem::transmute_copy(&channelcount), core::mem::transmute_copy(&newchannelvolumearray), core::mem::transmute_copy(&changedchannel), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnGroupingParamChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnGroupingParamChanged(this, core::mem::transmute_copy(&newgroupingparam), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnStateChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: AudioSessionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnStateChanged(this, core::mem::transmute_copy(&newstate)).into()
            }
        }
        unsafe extern "system" fn OnSessionDisconnected<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnSessionDisconnected(this, core::mem::transmute_copy(&disconnectreason)).into()
            }
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
impl windows_core::RuntimeName for IAudioSessionEvents {}
windows_core::imp::define_interface!(IAudioSessionManager, IAudioSessionManager_Vtbl, 0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
windows_core::imp::interface_hierarchy!(IAudioSessionManager, windows_core::IUnknown);
impl IAudioSessionManager {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: Option<*const windows_core::GUID>, streamflags: u32) -> windows_core::Result<IAudioSessionControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioSessionControl)(windows_core::Interface::as_raw(self), audiosessionguid.unwrap_or(core::mem::zeroed()) as _, streamflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: Option<*const windows_core::GUID>, streamflags: u32) -> windows_core::Result<ISimpleAudioVolume> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSimpleAudioVolume)(windows_core::Interface::as_raw(self), audiosessionguid.unwrap_or(core::mem::zeroed()) as _, streamflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAudioSessionControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSimpleAudioVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionManager_Impl: windows_core::IUnknownImpl {
    fn GetAudioSessionControl(&self, audiosessionguid: *const windows_core::GUID, streamflags: u32) -> windows_core::Result<IAudioSessionControl>;
    fn GetSimpleAudioVolume(&self, audiosessionguid: *const windows_core::GUID, streamflags: u32) -> windows_core::Result<ISimpleAudioVolume>;
}
impl IAudioSessionManager_Vtbl {
    pub const fn new<Identity: IAudioSessionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAudioSessionControl<Identity: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *const windows_core::GUID, streamflags: u32, sessioncontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionManager_Impl::GetAudioSessionControl(this, core::mem::transmute_copy(&audiosessionguid), core::mem::transmute_copy(&streamflags)) {
                    Ok(ok__) => {
                        sessioncontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Identity: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *const windows_core::GUID, streamflags: u32, audiovolume: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionManager_Impl::GetSimpleAudioVolume(this, core::mem::transmute_copy(&audiosessionguid), core::mem::transmute_copy(&streamflags)) {
                    Ok(ok__) => {
                        audiovolume.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IAudioSessionManager {}
windows_core::imp::define_interface!(IAudioSessionManager2, IAudioSessionManager2_Vtbl, 0x77aa99a0_1bd6_484f_8bc7_2c654c9a9b6f);
impl core::ops::Deref for IAudioSessionManager2 {
    type Target = IAudioSessionManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionManager2, windows_core::IUnknown, IAudioSessionManager);
impl IAudioSessionManager2 {
    pub unsafe fn GetSessionEnumerator(&self) -> windows_core::Result<IAudioSessionEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterSessionNotification<P0>(&self, sessionnotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterSessionNotification)(windows_core::Interface::as_raw(self), sessionnotification.param().abi()).ok() }
    }
    pub unsafe fn UnregisterSessionNotification<P0>(&self, sessionnotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterSessionNotification)(windows_core::Interface::as_raw(self), sessionnotification.param().abi()).ok() }
    }
    pub unsafe fn RegisterDuckNotification<P0, P1>(&self, sessionid: P0, ducknotification: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IAudioVolumeDuckNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterDuckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi(), ducknotification.param().abi()).ok() }
    }
    pub unsafe fn UnregisterDuckNotification<P0>(&self, ducknotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioVolumeDuckNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterDuckNotification)(windows_core::Interface::as_raw(self), ducknotification.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager2_Vtbl {
    pub base__: IAudioSessionManager_Vtbl,
    pub GetSessionEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionManager2_Impl: IAudioSessionManager_Impl {
    fn GetSessionEnumerator(&self) -> windows_core::Result<IAudioSessionEnumerator>;
    fn RegisterSessionNotification(&self, sessionnotification: windows_core::Ref<IAudioSessionNotification>) -> windows_core::Result<()>;
    fn UnregisterSessionNotification(&self, sessionnotification: windows_core::Ref<IAudioSessionNotification>) -> windows_core::Result<()>;
    fn RegisterDuckNotification(&self, sessionid: &windows_core::PCWSTR, ducknotification: windows_core::Ref<IAudioVolumeDuckNotification>) -> windows_core::Result<()>;
    fn UnregisterDuckNotification(&self, ducknotification: windows_core::Ref<IAudioVolumeDuckNotification>) -> windows_core::Result<()>;
}
impl IAudioSessionManager2_Vtbl {
    pub const fn new<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSessionEnumerator<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionManager2_Impl::GetSessionEnumerator(this) {
                    Ok(ok__) => {
                        sessionenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterSessionNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionnotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::RegisterSessionNotification(this, core::mem::transmute_copy(&sessionnotification)).into()
            }
        }
        unsafe extern "system" fn UnregisterSessionNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionnotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::UnregisterSessionNotification(this, core::mem::transmute_copy(&sessionnotification)).into()
            }
        }
        unsafe extern "system" fn RegisterDuckNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, ducknotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::RegisterDuckNotification(this, core::mem::transmute(&sessionid), core::mem::transmute_copy(&ducknotification)).into()
            }
        }
        unsafe extern "system" fn UnregisterDuckNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ducknotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::UnregisterDuckNotification(this, core::mem::transmute_copy(&ducknotification)).into()
            }
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
impl windows_core::RuntimeName for IAudioSessionManager2 {}
windows_core::imp::define_interface!(IAudioSessionNotification, IAudioSessionNotification_Vtbl, 0x641dd20b_4d41_49cc_aba3_174b9477bb08);
windows_core::imp::interface_hierarchy!(IAudioSessionNotification, windows_core::IUnknown);
impl IAudioSessionNotification {
    pub unsafe fn OnSessionCreated<P0>(&self, newsession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSessionCreated)(windows_core::Interface::as_raw(self), newsession.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSessionCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionNotification_Impl: windows_core::IUnknownImpl {
    fn OnSessionCreated(&self, newsession: windows_core::Ref<IAudioSessionControl>) -> windows_core::Result<()>;
}
impl IAudioSessionNotification_Vtbl {
    pub const fn new<Identity: IAudioSessionNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSessionCreated<Identity: IAudioSessionNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newsession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionNotification_Impl::OnSessionCreated(this, core::mem::transmute_copy(&newsession)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSessionCreated: OnSessionCreated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioSessionNotification {}
windows_core::imp::define_interface!(IAudioStateMonitor, IAudioStateMonitor_Vtbl, 0x63bd8738_e30d_4c77_bf5c_834e87c657e2);
windows_core::imp::interface_hierarchy!(IAudioStateMonitor, windows_core::IUnknown);
impl IAudioStateMonitor {
    pub unsafe fn RegisterCallback(&self, callback: PAudioStateMonitorCallback, context: Option<*const core::ffi::c_void>) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterCallback)(windows_core::Interface::as_raw(self), callback, context.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterCallback(&self, registration: i64) {
        unsafe { (windows_core::Interface::vtable(self).UnregisterCallback)(windows_core::Interface::as_raw(self), registration) }
    }
    pub unsafe fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        unsafe { (windows_core::Interface::vtable(self).GetSoundLevel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, PAudioStateMonitorCallback, *const core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub UnregisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i64),
    pub GetSoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> AudioStateMonitorSoundLevel,
}
pub trait IAudioStateMonitor_Impl: windows_core::IUnknownImpl {
    fn RegisterCallback(&self, callback: PAudioStateMonitorCallback, context: *const core::ffi::c_void) -> windows_core::Result<i64>;
    fn UnregisterCallback(&self, registration: i64);
    fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel;
}
impl IAudioStateMonitor_Vtbl {
    pub const fn new<Identity: IAudioStateMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterCallback<Identity: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: PAudioStateMonitorCallback, context: *const core::ffi::c_void, registration: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioStateMonitor_Impl::RegisterCallback(this, core::mem::transmute_copy(&callback), core::mem::transmute_copy(&context)) {
                    Ok(ok__) => {
                        registration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterCallback<Identity: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registration: i64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStateMonitor_Impl::UnregisterCallback(this, core::mem::transmute_copy(&registration))
            }
        }
        unsafe extern "system" fn GetSoundLevel<Identity: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> AudioStateMonitorSoundLevel {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStateMonitor_Impl::GetSoundLevel(this)
            }
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
impl windows_core::RuntimeName for IAudioStateMonitor {}
windows_core::imp::define_interface!(IAudioStreamVolume, IAudioStreamVolume_Vtbl, 0x93014887_242d_4068_8a15_cf5e93b90fe3);
windows_core::imp::interface_hierarchy!(IAudioStreamVolume, windows_core::IUnknown);
impl IAudioStreamVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel).ok() }
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())).ok() }
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioStreamVolume_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> windows_core::Result<()>;
}
impl IAudioStreamVolume_Vtbl {
    pub const fn new<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioStreamVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStreamVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolume<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioStreamVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pflevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStreamVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
        }
        unsafe extern "system" fn GetAllVolumes<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStreamVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
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
impl windows_core::RuntimeName for IAudioStreamVolume {}
windows_core::imp::define_interface!(IAudioSystemEffectsPropertyChangeNotificationClient, IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl, 0x20049d40_56d5_400e_a2ef_385599feed49);
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsPropertyChangeNotificationClient, windows_core::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    pub unsafe fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnPropertyChanged)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(key)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
}
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Impl: windows_core::IUnknownImpl {
    fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: &super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
}
impl IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub const fn new<Identity: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPropertyChanged<Identity: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyChangeNotificationClient_Impl::OnPropertyChanged(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&key)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPropertyChanged: OnPropertyChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyChangeNotificationClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioSystemEffectsPropertyChangeNotificationClient {}
windows_core::imp::define_interface!(IAudioSystemEffectsPropertyStore, IAudioSystemEffectsPropertyStore_Vtbl, 0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsPropertyStore, windows_core::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDefaultPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenUserPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenVolatilePropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ResetUserPropertyStore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetUserPropertyStore)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ResetVolatilePropertyStore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResetVolatilePropertyStore)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RegisterPropertyChangeNotification<P0>(&self, callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterPropertyChangeNotification)(windows_core::Interface::as_raw(self), callback.param().abi()).ok() }
    }
    pub unsafe fn UnregisterPropertyChangeNotification<P0>(&self, callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterPropertyChangeNotification)(windows_core::Interface::as_raw(self), callback.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenDefaultPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenDefaultPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenUserPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenUserPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenVolatilePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenVolatilePropertyStore: usize,
    pub ResetUserPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetVolatilePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPropertyChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterPropertyChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyStore_Impl: windows_core::IUnknownImpl {
    fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenUserPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn ResetUserPropertyStore(&self) -> windows_core::Result<()>;
    fn ResetVolatilePropertyStore(&self) -> windows_core::Result<()>;
    fn RegisterPropertyChangeNotification(&self, callback: windows_core::Ref<IAudioSystemEffectsPropertyChangeNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterPropertyChangeNotification(&self, callback: windows_core::Ref<IAudioSystemEffectsPropertyChangeNotificationClient>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyStore_Vtbl {
    pub const fn new<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenDefaultPropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSystemEffectsPropertyStore_Impl::OpenDefaultPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        propstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenUserPropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSystemEffectsPropertyStore_Impl::OpenUserPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        propstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSystemEffectsPropertyStore_Impl::OpenVolatilePropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        propstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetUserPropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::ResetUserPropertyStore(this).into()
            }
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::ResetVolatilePropertyStore(this).into()
            }
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::RegisterPropertyChangeNotification(this, core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::UnregisterPropertyChangeNotification(this, core::mem::transmute_copy(&callback)).into()
            }
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for IAudioSystemEffectsPropertyStore {}
windows_core::imp::define_interface!(IAudioTreble, IAudioTreble_Vtbl, 0x0a717812_694e_4907_b74b_bafa5cfdca7b);
impl core::ops::Deref for IAudioTreble {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioTreble, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTreble_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
pub trait IAudioTreble_Impl: IPerChannelDbLevel_Impl {}
impl IAudioTreble_Vtbl {
    pub const fn new<Identity: IAudioTreble_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioTreble as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioTreble {}
windows_core::imp::define_interface!(IAudioViewManagerService, IAudioViewManagerService_Vtbl, 0xa7a7ef10_1f49_45e0_ad35_612057cc8f74);
windows_core::imp::interface_hierarchy!(IAudioViewManagerService, windows_core::IUnknown);
impl IAudioViewManagerService {
    pub unsafe fn SetAudioStreamWindow(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAudioStreamWindow)(windows_core::Interface::as_raw(self), hwnd).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioViewManagerService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAudioStreamWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IAudioViewManagerService_Impl: windows_core::IUnknownImpl {
    fn SetAudioStreamWindow(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
impl IAudioViewManagerService_Vtbl {
    pub const fn new<Identity: IAudioViewManagerService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAudioStreamWindow<Identity: IAudioViewManagerService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioViewManagerService_Impl::SetAudioStreamWindow(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetAudioStreamWindow: SetAudioStreamWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioViewManagerService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioViewManagerService {}
windows_core::imp::define_interface!(IAudioVolumeDuckNotification, IAudioVolumeDuckNotification_Vtbl, 0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
windows_core::imp::interface_hierarchy!(IAudioVolumeDuckNotification, windows_core::IUnknown);
impl IAudioVolumeDuckNotification {
    pub unsafe fn OnVolumeDuckNotification<P0>(&self, sessionid: P0, countcommunicationsessions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnVolumeDuckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi(), countcommunicationsessions).ok() }
    }
    pub unsafe fn OnVolumeUnduckNotification<P0>(&self, sessionid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnVolumeUnduckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeDuckNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnVolumeDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnVolumeUnduckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAudioVolumeDuckNotification_Impl: windows_core::IUnknownImpl {
    fn OnVolumeDuckNotification(&self, sessionid: &windows_core::PCWSTR, countcommunicationsessions: u32) -> windows_core::Result<()>;
    fn OnVolumeUnduckNotification(&self, sessionid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAudioVolumeDuckNotification_Vtbl {
    pub const fn new<Identity: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnVolumeDuckNotification<Identity: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, countcommunicationsessions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioVolumeDuckNotification_Impl::OnVolumeDuckNotification(this, core::mem::transmute(&sessionid), core::mem::transmute_copy(&countcommunicationsessions)).into()
            }
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Identity: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioVolumeDuckNotification_Impl::OnVolumeUnduckNotification(this, core::mem::transmute(&sessionid)).into()
            }
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
impl windows_core::RuntimeName for IAudioVolumeDuckNotification {}
windows_core::imp::define_interface!(IAudioVolumeLevel, IAudioVolumeLevel_Vtbl, 0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
impl core::ops::Deref for IAudioVolumeLevel {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioVolumeLevel, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeLevel_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
pub trait IAudioVolumeLevel_Impl: IPerChannelDbLevel_Impl {}
impl IAudioVolumeLevel_Vtbl {
    pub const fn new<Identity: IAudioVolumeLevel_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioVolumeLevel as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioVolumeLevel {}
windows_core::imp::define_interface!(IChannelAudioVolume, IChannelAudioVolume_Vtbl, 0x1c158861_b533_4b30_b1cf_e853e51c59b8);
windows_core::imp::interface_hierarchy!(IChannelAudioVolume, windows_core::IUnknown);
impl IChannelAudioVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel, eventcontext).ok() }
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32], eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr()), eventcontext).ok() }
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IChannelAudioVolume_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> windows_core::Result<()>;
}
impl IChannelAudioVolume_Vtbl {
    pub const fn new<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IChannelAudioVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelAudioVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolume<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IChannelAudioVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pflevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelAudioVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetAllVolumes<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelAudioVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
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
impl windows_core::RuntimeName for IChannelAudioVolume {}
windows_core::imp::define_interface!(IConnector, IConnector_Vtbl, 0x9c2c4058_23f5_41de_877a_df3af236a09e);
windows_core::imp::interface_hierarchy!(IConnector, windows_core::IUnknown);
impl IConnector {
    pub unsafe fn GetType(&self) -> windows_core::Result<ConnectorType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataFlow(&self) -> windows_core::Result<DataFlow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ConnectTo<P0>(&self, pconnectto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConnector>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectTo)(windows_core::Interface::as_raw(self), pconnectto.param().abi()).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectedTo(&self) -> windows_core::Result<IConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectorIdConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceIdConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConnectorType) -> windows_core::HRESULT,
    pub GetDataFlow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataFlow) -> windows_core::HRESULT,
    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnectorIdConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDeviceIdConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IConnector_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<ConnectorType>;
    fn GetDataFlow(&self) -> windows_core::Result<DataFlow>;
    fn ConnectTo(&self, pconnectto: windows_core::Ref<IConnector>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetConnectedTo(&self) -> windows_core::Result<IConnector>;
    fn GetConnectorIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDeviceIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IConnector_Vtbl {
    pub const fn new<Identity: IConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut ConnectorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataFlow<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflow: *mut DataFlow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetDataFlow(this) {
                    Ok(ok__) => {
                        pflow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::ConnectTo(this, core::mem::transmute_copy(&pconnectto)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbconnected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pbconnected.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetConnectedTo(this) {
                    Ok(ok__) => {
                        ppconto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrconnectorid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetConnectorIdConnectedTo(this) {
                    Ok(ok__) => {
                        ppwstrconnectorid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetDeviceIdConnectedTo(this) {
                    Ok(ok__) => {
                        ppwstrdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IConnector {}
windows_core::imp::define_interface!(IControlChangeNotify, IControlChangeNotify_Vtbl, 0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
windows_core::imp::interface_hierarchy!(IControlChangeNotify, windows_core::IUnknown);
impl IControlChangeNotify {
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), dwsenderprocessid, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IControlChangeNotify_Impl: windows_core::IUnknownImpl {
    fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IControlChangeNotify_Vtbl {
    pub const fn new<Identity: IControlChangeNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNotify<Identity: IControlChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlChangeNotify_Impl::OnNotify(this, core::mem::transmute_copy(&dwsenderprocessid), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlChangeNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IControlChangeNotify {}
windows_core::imp::define_interface!(IControlInterface, IControlInterface_Vtbl, 0x45d37c3f_5140_444a_ae24_400789f3cbf3);
windows_core::imp::interface_hierarchy!(IControlInterface, windows_core::IUnknown);
impl IControlInterface {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IControlInterface_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetIID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IControlInterface_Vtbl {
    pub const fn new<Identity: IControlInterface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IControlInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlInterface_Impl::GetName(this) {
                    Ok(ok__) => {
                        ppwstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIID<Identity: IControlInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlInterface_Impl::GetIID(this) {
                    Ok(ok__) => {
                        piid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetIID: GetIID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlInterface as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IControlInterface {}
windows_core::imp::define_interface!(IDeviceSpecificProperty, IDeviceSpecificProperty_Vtbl, 0x3b22bcbf_2586_4af0_8583_205d391b807c);
windows_core::imp::interface_hierarchy!(IDeviceSpecificProperty, windows_core::IUnknown);
impl IDeviceSpecificProperty {
    pub unsafe fn GetType(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pvvalue as _, pcbvalue as _).ok() }
    }
    pub unsafe fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), pvvalue, cbvalue, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Get4BRange)(windows_core::Interface::as_raw(self), plmin as _, plmax as _, plstepping as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSpecificProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Get4BRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
pub trait IDeviceSpecificProperty_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<u16>;
    fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::Result<()>;
    fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::Result<()>;
}
impl IDeviceSpecificProperty_Vtbl {
    pub const fn new<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceSpecificProperty_Impl::GetType(this) {
                    Ok(ok__) => {
                        pvtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::GetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&pcbvalue)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::SetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&cbvalue), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn Get4BRange<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::Get4BRange(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plstepping)).into()
            }
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
impl windows_core::RuntimeName for IDeviceSpecificProperty {}
windows_core::imp::define_interface!(IDeviceTopology, IDeviceTopology_Vtbl, 0x2a07407e_6497_4a18_9787_32f79bd0d98f);
windows_core::imp::interface_hierarchy!(IDeviceTopology, windows_core::IUnknown);
impl IDeviceTopology {
    pub unsafe fn GetConnectorCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnector)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSubunitCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubunitCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubunit(&self, nindex: u32) -> windows_core::Result<ISubunit> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubunit)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPartById(&self, nid: u32) -> windows_core::Result<IPart> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartById)(windows_core::Interface::as_raw(self), nid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignalPath<P0, P1>(&self, pipartfrom: P0, pipartto: P1, brejectmixedpaths: bool) -> windows_core::Result<IPartsList>
    where
        P0: windows_core::Param<IPart>,
        P1: windows_core::Param<IPart>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignalPath)(windows_core::Interface::as_raw(self), pipartfrom.param().abi(), pipartto.param().abi(), brejectmixedpaths.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubunitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSubunit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDeviceTopology_Impl: windows_core::IUnknownImpl {
    fn GetConnectorCount(&self) -> windows_core::Result<u32>;
    fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector>;
    fn GetSubunitCount(&self) -> windows_core::Result<u32>;
    fn GetSubunit(&self, nindex: u32) -> windows_core::Result<ISubunit>;
    fn GetPartById(&self, nid: u32) -> windows_core::Result<IPart>;
    fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignalPath(&self, pipartfrom: windows_core::Ref<IPart>, pipartto: windows_core::Ref<IPart>, brejectmixedpaths: windows_core::BOOL) -> windows_core::Result<IPartsList>;
}
impl IDeviceTopology_Vtbl {
    pub const fn new<Identity: IDeviceTopology_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectorCount<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetConnectorCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnector<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetConnector(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubunitCount<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetSubunitCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubunit<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppsubunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetSubunit(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppsubunit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartById<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nid: u32, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetPartById(this, core::mem::transmute_copy(&nid)) {
                    Ok(ok__) => {
                        pppart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetDeviceId(this) {
                    Ok(ok__) => {
                        ppwstrdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignalPath<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipartfrom: *mut core::ffi::c_void, pipartto: *mut core::ffi::c_void, brejectmixedpaths: windows_core::BOOL, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetSignalPath(this, core::mem::transmute_copy(&pipartfrom), core::mem::transmute_copy(&pipartto), core::mem::transmute_copy(&brejectmixedpaths)) {
                    Ok(ok__) => {
                        ppparts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IDeviceTopology {}
windows_core::imp::define_interface!(IMMDevice, IMMDevice_Vtbl, 0xd666063f_1587_4e43_81f1_b948e807363f);
windows_core::imp::interface_hierarchy!(IMMDevice, windows_core::IUnknown);
impl IMMDevice {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Activate<T>(&self, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &T::IID, dwclsctx, pactivationparams.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::STGM) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<DEVICE_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::System::Com::CLSCTX, *const super::super::System::Com::StructuredStorage::PROPVARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Activate: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Com::STGM, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DEVICE_STATE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMMDevice_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, iid: *const windows_core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::STGM) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetState(&self) -> windows_core::Result<DEVICE_STATE>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMMDevice_Vtbl {
    pub const fn new<Identity: IMMDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDevice_Impl::Activate(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&dwclsctx), core::mem::transmute_copy(&pactivationparams), core::mem::transmute_copy(&ppinterface)).into()
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: super::super::System::Com::STGM, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::OpenPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetId<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstrid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::GetId(this) {
                    Ok(ok__) => {
                        ppstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetState<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut DEVICE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::GetState(this) {
                    Ok(ok__) => {
                        pdwstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IMMDevice {}
windows_core::imp::define_interface!(IMMDeviceActivator, IMMDeviceActivator_Vtbl, 0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
windows_core::imp::interface_hierarchy!(IMMDeviceActivator, windows_core::IUnknown);
impl IMMDeviceActivator {
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Activate<P1>(&self, iid: *const windows_core::GUID, pdevice: P1, pactivationparams: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IMMDevice>,
    {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), iid, pdevice.param().abi(), pactivationparams.unwrap_or(core::mem::zeroed()) as _, ppinterface as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceActivator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const super::super::System::Com::StructuredStorage::PROPVARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Activate: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IMMDeviceActivator_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, iid: *const windows_core::GUID, pdevice: windows_core::Ref<IMMDevice>, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl IMMDeviceActivator_Vtbl {
    pub const fn new<Identity: IMMDeviceActivator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IMMDeviceActivator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, pdevice: *mut core::ffi::c_void, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDeviceActivator_Impl::Activate(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pactivationparams), core::mem::transmute_copy(&ppinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceActivator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMMDeviceActivator {}
windows_core::imp::define_interface!(IMMDeviceCollection, IMMDeviceCollection_Vtbl, 0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
windows_core::imp::interface_hierarchy!(IMMDeviceCollection, windows_core::IUnknown);
impl IMMDeviceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), ndevice, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMMDeviceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice>;
}
impl IMMDeviceCollection_Vtbl {
    pub const fn new<Identity: IMMDeviceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdevices: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcdevices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ndevice: u32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceCollection_Impl::Item(this, core::mem::transmute_copy(&ndevice)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, Item: Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMDeviceCollection {}
windows_core::imp::define_interface!(IMMDeviceEnumerator, IMMDeviceEnumerator_Vtbl, 0xa95664d2_9614_4f35_a746_de8db63617e6);
windows_core::imp::interface_hierarchy!(IMMDeviceEnumerator, windows_core::IUnknown);
impl IMMDeviceEnumerator {
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: DEVICE_STATE) -> windows_core::Result<IMMDeviceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAudioEndpoints)(windows_core::Interface::as_raw(self), dataflow, dwstatemask, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultAudioEndpoint)(windows_core::Interface::as_raw(self), dataflow, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDevice<P0>(&self, pwstrid: P0) -> windows_core::Result<IMMDevice>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), pwstrid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMMNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterEndpointNotificationCallback)(windows_core::Interface::as_raw(self), pclient.param().abi()).ok() }
    }
    pub unsafe fn UnregisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMMNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterEndpointNotificationCallback)(windows_core::Interface::as_raw(self), pclient.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumAudioEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, DEVICE_STATE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEndpointNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterEndpointNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMMDeviceEnumerator_Impl: windows_core::IUnknownImpl {
    fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: DEVICE_STATE) -> windows_core::Result<IMMDeviceCollection>;
    fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice>;
    fn GetDevice(&self, pwstrid: &windows_core::PCWSTR) -> windows_core::Result<IMMDevice>;
    fn RegisterEndpointNotificationCallback(&self, pclient: windows_core::Ref<IMMNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterEndpointNotificationCallback(&self, pclient: windows_core::Ref<IMMNotificationClient>) -> windows_core::Result<()>;
}
impl IMMDeviceEnumerator_Vtbl {
    pub const fn new<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAudioEndpoints<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataflow: EDataFlow, dwstatemask: DEVICE_STATE, ppdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceEnumerator_Impl::EnumAudioEndpoints(this, core::mem::transmute_copy(&dataflow), core::mem::transmute_copy(&dwstatemask)) {
                    Ok(ok__) => {
                        ppdevices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceEnumerator_Impl::GetDefaultAudioEndpoint(this, core::mem::transmute_copy(&dataflow), core::mem::transmute_copy(&role)) {
                    Ok(ok__) => {
                        ppendpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrid: windows_core::PCWSTR, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceEnumerator_Impl::GetDevice(this, core::mem::transmute(&pwstrid)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclient: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDeviceEnumerator_Impl::RegisterEndpointNotificationCallback(this, core::mem::transmute_copy(&pclient)).into()
            }
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclient: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDeviceEnumerator_Impl::UnregisterEndpointNotificationCallback(this, core::mem::transmute_copy(&pclient)).into()
            }
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
impl windows_core::RuntimeName for IMMDeviceEnumerator {}
windows_core::imp::define_interface!(IMMEndpoint, IMMEndpoint_Vtbl, 0x1be09788_6894_4089_8586_9a2a6c265ac5);
windows_core::imp::interface_hierarchy!(IMMEndpoint, windows_core::IUnknown);
impl IMMEndpoint {
    pub unsafe fn GetDataFlow(&self) -> windows_core::Result<EDataFlow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMEndpoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataFlow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EDataFlow) -> windows_core::HRESULT,
}
pub trait IMMEndpoint_Impl: windows_core::IUnknownImpl {
    fn GetDataFlow(&self) -> windows_core::Result<EDataFlow>;
}
impl IMMEndpoint_Vtbl {
    pub const fn new<Identity: IMMEndpoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDataFlow<Identity: IMMEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataflow: *mut EDataFlow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMEndpoint_Impl::GetDataFlow(this) {
                    Ok(ok__) => {
                        pdataflow.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDataFlow: GetDataFlow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMEndpoint as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMEndpoint {}
windows_core::imp::define_interface!(IMMNotificationClient, IMMNotificationClient_Vtbl, 0x7991eec9_7e89_4d85_8390_6c703cec60c0);
windows_core::imp::interface_hierarchy!(IMMNotificationClient, windows_core::IUnknown);
impl IMMNotificationClient {
    pub unsafe fn OnDeviceStateChanged<P0>(&self, pwstrdeviceid: P0, dwnewstate: DEVICE_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceStateChanged)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi(), dwnewstate).ok() }
    }
    pub unsafe fn OnDeviceAdded<P0>(&self, pwstrdeviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceAdded)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi()).ok() }
    }
    pub unsafe fn OnDeviceRemoved<P0>(&self, pwstrdeviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceRemoved)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi()).ok() }
    }
    pub unsafe fn OnDefaultDeviceChanged<P2>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDefaultDeviceChanged)(windows_core::Interface::as_raw(self), flow, role, pwstrdefaultdeviceid.param().abi()).ok() }
    }
    pub unsafe fn OnPropertyValueChanged<P0>(&self, pwstrdeviceid: P0, key: super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPropertyValueChanged)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi(), core::mem::transmute(key)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDeviceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DEVICE_STATE) -> windows_core::HRESULT,
    pub OnDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDefaultDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnPropertyValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT,
}
pub trait IMMNotificationClient_Impl: windows_core::IUnknownImpl {
    fn OnDeviceStateChanged(&self, pwstrdeviceid: &windows_core::PCWSTR, dwnewstate: DEVICE_STATE) -> windows_core::Result<()>;
    fn OnDeviceAdded(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDeviceRemoved(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDefaultDeviceChanged(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnPropertyValueChanged(&self, pwstrdeviceid: &windows_core::PCWSTR, key: &super::super::Foundation::PROPERTYKEY) -> windows_core::Result<()>;
}
impl IMMNotificationClient_Vtbl {
    pub const fn new<Identity: IMMNotificationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDeviceStateChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR, dwnewstate: DEVICE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDeviceStateChanged(this, core::mem::transmute(&pwstrdeviceid), core::mem::transmute_copy(&dwnewstate)).into()
            }
        }
        unsafe extern "system" fn OnDeviceAdded<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDeviceAdded(this, core::mem::transmute(&pwstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn OnDeviceRemoved<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDeviceRemoved(this, core::mem::transmute(&pwstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDefaultDeviceChanged(this, core::mem::transmute_copy(&flow), core::mem::transmute_copy(&role), core::mem::transmute(&pwstrdefaultdeviceid)).into()
            }
        }
        unsafe extern "system" fn OnPropertyValueChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR, key: super::super::Foundation::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnPropertyValueChanged(this, core::mem::transmute(&pwstrdeviceid), core::mem::transmute(&key)).into()
            }
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
impl windows_core::RuntimeName for IMMNotificationClient {}
windows_core::imp::define_interface!(IMessageFilter, IMessageFilter_Vtbl, 0x00000016_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMessageFilter, windows_core::IUnknown);
impl IMessageFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleInComingCall(&self, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: Option<*const super::super::System::Com::INTERFACEINFO>) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).HandleInComingCall)(windows_core::Interface::as_raw(self), dwcalltype, htaskcaller, dwtickcount, lpinterfaceinfo.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RetryRejectedCall(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).RetryRejectedCall)(windows_core::Interface::as_raw(self), htaskcallee, dwtickcount, dwrejecttype) }
    }
    pub unsafe fn MessagePending(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).MessagePending)(windows_core::Interface::as_raw(self), htaskcallee, dwtickcount, dwpendingtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleInComingCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HTASK, u32, *const super::super::System::Com::INTERFACEINFO) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleInComingCall: usize,
    pub RetryRejectedCall: unsafe extern "system" fn(*mut core::ffi::c_void, super::HTASK, u32, u32) -> u32,
    pub MessagePending: unsafe extern "system" fn(*mut core::ffi::c_void, super::HTASK, u32, u32) -> u32,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageFilter_Impl: windows_core::IUnknownImpl {
    fn HandleInComingCall(&self, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32;
    fn RetryRejectedCall(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32;
    fn MessagePending(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl IMessageFilter_Vtbl {
    pub const fn new<Identity: IMessageFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleInComingCall<Identity: IMessageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageFilter_Impl::HandleInComingCall(this, core::mem::transmute_copy(&dwcalltype), core::mem::transmute_copy(&htaskcaller), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&lpinterfaceinfo))
            }
        }
        unsafe extern "system" fn RetryRejectedCall<Identity: IMessageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageFilter_Impl::RetryRejectedCall(this, core::mem::transmute_copy(&htaskcallee), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&dwrejecttype))
            }
        }
        unsafe extern "system" fn MessagePending<Identity: IMessageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageFilter_Impl::MessagePending(this, core::mem::transmute_copy(&htaskcallee), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&dwpendingtype))
            }
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IMessageFilter {}
windows_core::imp::define_interface!(IPart, IPart_Vtbl, 0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
windows_core::imp::interface_hierarchy!(IPart, windows_core::IUnknown);
impl IPart {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlobalId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlobalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPartType(&self) -> windows_core::Result<PartType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetControlInterfaceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControlInterfaceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> windows_core::Result<IControlInterface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControlInterface)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumPartsIncoming(&self) -> windows_core::Result<IPartsList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPartsIncoming)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumPartsOutgoing(&self) -> windows_core::Result<IPartsList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPartsOutgoing)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTopologyObject(&self) -> windows_core::Result<IDeviceTopology> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopologyObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), dwclscontext, refiid, ppvobject.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn RegisterControlChangeCallback<P1>(&self, riid: *const windows_core::GUID, pnotify: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IControlChangeNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterControlChangeCallback)(windows_core::Interface::as_raw(self), riid, pnotify.param().abi()).ok() }
    }
    pub unsafe fn UnregisterControlChangeCallback<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IControlChangeNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterControlChangeCallback)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPart_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGlobalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPartType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PartType) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetControlInterfaceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetControlInterface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumPartsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumPartsOutgoing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTopologyObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterControlChangeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterControlChangeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPart_Impl: windows_core::IUnknownImpl {
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
    fn RegisterControlChangeCallback(&self, riid: *const windows_core::GUID, pnotify: windows_core::Ref<IControlChangeNotify>) -> windows_core::Result<()>;
    fn UnregisterControlChangeCallback(&self, pnotify: windows_core::Ref<IControlChangeNotify>) -> windows_core::Result<()>;
}
impl IPart_Vtbl {
    pub const fn new<Identity: IPart_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetName(this) {
                    Ok(ok__) => {
                        ppwstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalId<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetLocalId(this) {
                    Ok(ok__) => {
                        pnid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlobalId<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrglobalid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetGlobalId(this) {
                    Ok(ok__) => {
                        ppwstrglobalid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartType<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparttype: *mut PartType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetPartType(this) {
                    Ok(ok__) => {
                        pparttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubType<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetSubType(this) {
                    Ok(ok__) => {
                        psubtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetControlInterfaceCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetControlInterface<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetControlInterface(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppinterfacedesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::EnumPartsIncoming(this) {
                    Ok(ok__) => {
                        ppparts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::EnumPartsOutgoing(this) {
                    Ok(ok__) => {
                        ppparts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTopologyObject<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetTopologyObject(this) {
                    Ok(ok__) => {
                        pptopology.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activate<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::Activate(this, core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::RegisterControlChangeCallback(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::UnregisterControlChangeCallback(this, core::mem::transmute_copy(&pnotify)).into()
            }
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
impl windows_core::RuntimeName for IPart {}
windows_core::imp::define_interface!(IPartsList, IPartsList_Vtbl, 0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
windows_core::imp::interface_hierarchy!(IPartsList, windows_core::IUnknown);
impl IPartsList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPart(&self, nindex: u32) -> windows_core::Result<IPart> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPart)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPartsList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetPart(&self, nindex: u32) -> windows_core::Result<IPart>;
}
impl IPartsList_Vtbl {
    pub const fn new<Identity: IPartsList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPartsList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartsList_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPart<Identity: IPartsList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartsList_Impl::GetPart(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        pppart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetPart: GetPart::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartsList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartsList {}
windows_core::imp::define_interface!(IPerChannelDbLevel, IPerChannelDbLevel_Vtbl, 0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
windows_core::imp::interface_hierarchy!(IPerChannelDbLevel, windows_core::IUnknown);
impl IPerChannelDbLevel {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLevelRange)(windows_core::Interface::as_raw(self), nchannel, pfminleveldb as _, pfmaxleveldb as _, pfstepping as _).ok() }
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLevelUniform)(windows_core::Interface::as_raw(self), fleveldb, pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLevelAllChannels)(windows_core::Interface::as_raw(self), core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len().try_into().unwrap(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLevelRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetLevelUniform: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetLevelAllChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IPerChannelDbLevel_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::Result<()>;
    fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IPerChannelDbLevel_Vtbl {
    pub const fn new<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerChannelDbLevel_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pcchannels.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLevelRange<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::GetLevelRange(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&pfminleveldb), core::mem::transmute_copy(&pfmaxleveldb), core::mem::transmute_copy(&pfstepping)).into()
            }
        }
        unsafe extern "system" fn GetLevel<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerChannelDbLevel_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pfleveldb.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLevel<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetLevelUniform<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevelUniform(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetLevelAllChannels<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevelAllChannels(this, core::mem::transmute_copy(&alevelsdb), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pguideventcontext)).into()
            }
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
impl windows_core::RuntimeName for IPerChannelDbLevel {}
windows_core::imp::define_interface!(ISimpleAudioVolume, ISimpleAudioVolume_Vtbl, 0x87ce5498_68d6_44e5_9215_6da47ef883d8);
windows_core::imp::interface_hierarchy!(ISimpleAudioVolume, windows_core::IUnknown);
impl ISimpleAudioVolume {
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolume)(windows_core::Interface::as_raw(self), flevel, eventcontext).ok() }
    }
    pub unsafe fn GetMasterVolume(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMute(&self, bmute: bool, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.into(), eventcontext).ok() }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ISimpleAudioVolume_Impl: windows_core::IUnknownImpl {
    fn SetMasterVolume(&self, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMasterVolume(&self) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ISimpleAudioVolume_Vtbl {
    pub const fn new<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMasterVolume<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleAudioVolume_Impl::SetMasterVolume(this, core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMasterVolume<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimpleAudioVolume_Impl::GetMasterVolume(this) {
                    Ok(ok__) => {
                        pflevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMute<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleAudioVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimpleAudioVolume_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISimpleAudioVolume {}
windows_core::imp::define_interface!(ISpatialAudioClient, ISpatialAudioClient_Vtbl, 0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
windows_core::imp::interface_hierarchy!(ISpatialAudioClient, windows_core::IUnknown);
impl ISpatialAudioClient {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStaticObjectPosition)(windows_core::Interface::as_raw(self), r#type, x as _, y as _, z as _).ok() }
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> windows_core::Result<AudioObjectType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNativeStaticObjectTypeMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxDynamicObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> windows_core::Result<IAudioFormatEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedAudioObjectFormatEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxFrameCount)(windows_core::Interface::as_raw(self), objectformat, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsAudioObjectFormatSupported)(windows_core::Interface::as_raw(self), objectformat).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const windows_core::GUID, auxiliaryinfo: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsSpatialAudioStreamAvailable)(windows_core::Interface::as_raw(self), streamuuid, auxiliaryinfo.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn ActivateSpatialAudioStream<T>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ActivateSpatialAudioStream)(windows_core::Interface::as_raw(self), core::mem::transmute(activationparams), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStaticObjectPosition: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetNativeStaticObjectTypeMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioObjectType) -> windows_core::HRESULT,
    pub GetMaxDynamicObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedAudioObjectFormatEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    pub IsAudioObjectFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub IsSpatialAudioStreamAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    IsSpatialAudioStreamAvailable: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub ActivateSpatialAudioStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::System::Com::StructuredStorage::PROPVARIANT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    ActivateSpatialAudioStream: usize,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISpatialAudioClient_Impl: windows_core::IUnknownImpl {
    fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::Result<()>;
    fn GetNativeStaticObjectTypeMask(&self) -> windows_core::Result<AudioObjectType>;
    fn GetMaxDynamicObjectCount(&self) -> windows_core::Result<u32>;
    fn GetSupportedAudioObjectFormatEnumerator(&self) -> windows_core::Result<IAudioFormatEnumerator>;
    fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32>;
    fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<()>;
    fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const windows_core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::Result<()>;
    fn ActivateSpatialAudioStream(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const windows_core::GUID, stream: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISpatialAudioClient_Vtbl {
    pub const fn new<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStaticObjectPosition<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::GetStaticObjectPosition(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
            }
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: *mut AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetNativeStaticObjectTypeMask(this) {
                    Ok(ok__) => {
                        mask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetMaxDynamicObjectCount(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetSupportedAudioObjectFormatEnumerator(this) {
                    Ok(ok__) => {
                        enumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxFrameCount<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetMaxFrameCount(this, core::mem::transmute_copy(&objectformat)) {
                    Ok(ok__) => {
                        framecountperbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::IsAudioObjectFormatSupported(this, core::mem::transmute_copy(&objectformat)).into()
            }
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamuuid: *const windows_core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::IsSpatialAudioStreamAvailable(this, core::mem::transmute_copy(&streamuuid), core::mem::transmute_copy(&auxiliaryinfo)).into()
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const windows_core::GUID, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::ActivateSpatialAudioStream(this, core::mem::transmute_copy(&activationparams), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&stream)).into()
            }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpatialAudioClient {}
windows_core::imp::define_interface!(ISpatialAudioClient2, ISpatialAudioClient2_Vtbl, 0xcaabe452_a66a_4bee_a93e_e320463f6a53);
impl core::ops::Deref for ISpatialAudioClient2 {
    type Target = ISpatialAudioClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioClient2, windows_core::IUnknown, ISpatialAudioClient);
impl ISpatialAudioClient2 {
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOffloadCapable)(windows_core::Interface::as_raw(self), category, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxFrameCountForCategory(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: bool, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxFrameCountForCategory)(windows_core::Interface::as_raw(self), category, offloadenabled.into(), objectformat, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient2_Vtbl {
    pub base__: ISpatialAudioClient_Vtbl,
    pub IsOffloadCapable: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_STREAM_CATEGORY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetMaxFrameCountForCategory: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_STREAM_CATEGORY, windows_core::BOOL, *const WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISpatialAudioClient2_Impl: ISpatialAudioClient_Impl {
    fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL>;
    fn GetMaxFrameCountForCategory(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: windows_core::BOOL, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ISpatialAudioClient2_Vtbl {
    pub const fn new<Identity: ISpatialAudioClient2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsOffloadCapable<Identity: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient2_Impl::IsOffloadCapable(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        isoffloadcapable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Identity: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: windows_core::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient2_Impl::GetMaxFrameCountForCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&offloadenabled), core::mem::transmute_copy(&objectformat)) {
                    Ok(ok__) => {
                        framecountperbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpatialAudioClient2 {}
windows_core::imp::define_interface!(ISpatialAudioMetadataClient, ISpatialAudioMetadataClient_Vtbl, 0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataClient, windows_core::IUnknown);
impl ISpatialAudioMetadataClient {
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: Option<*mut Option<ISpatialAudioMetadataItemsBuffer>>, metadataitems: *mut Option<ISpatialAudioMetadataItems>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataItems)(windows_core::Interface::as_raw(self), maxitemcount, framecount, metadataitemsbuffer.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(metadataitems)).ok() }
    }
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpatialAudioMetadataItemsBufferLength)(windows_core::Interface::as_raw(self), maxitemcount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> windows_core::Result<ISpatialAudioMetadataWriter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataWriter)(windows_core::Interface::as_raw(self), overflowmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> windows_core::Result<ISpatialAudioMetadataCopier> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataCopier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> windows_core::Result<ISpatialAudioMetadataReader> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateSpatialAudioMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpatialAudioMetadataItemsBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialAudioMetadataWriterOverflowMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataCopier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataClient_Impl: windows_core::IUnknownImpl {
    fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: windows_core::OutRef<ISpatialAudioMetadataItemsBuffer>, metadataitems: windows_core::OutRef<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> windows_core::Result<u32>;
    fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> windows_core::Result<ISpatialAudioMetadataWriter>;
    fn ActivateSpatialAudioMetadataCopier(&self) -> windows_core::Result<ISpatialAudioMetadataCopier>;
    fn ActivateSpatialAudioMetadataReader(&self) -> windows_core::Result<ISpatialAudioMetadataReader>;
}
impl ISpatialAudioMetadataClient_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut *mut core::ffi::c_void, metadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataItems(this, core::mem::transmute_copy(&maxitemcount), core::mem::transmute_copy(&framecount), core::mem::transmute_copy(&metadataitemsbuffer), core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::GetSpatialAudioMetadataItemsBufferLength(this, core::mem::transmute_copy(&maxitemcount)) {
                    Ok(ok__) => {
                        bufferlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataWriter(this, core::mem::transmute_copy(&overflowmode)) {
                    Ok(ok__) => {
                        metadatawriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatacopier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataCopier(this) {
                    Ok(ok__) => {
                        metadatacopier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Identity: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatareader: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataClient_Impl::ActivateSpatialAudioMetadataReader(this) {
                    Ok(ok__) => {
                        metadatareader.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISpatialAudioMetadataClient {}
windows_core::imp::define_interface!(ISpatialAudioMetadataCopier, ISpatialAudioMetadataCopier_Vtbl, 0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataCopier, windows_core::IUnknown);
impl ISpatialAudioMetadataCopier {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()).ok() }
    }
    pub unsafe fn CopyMetadataForFrames<P2>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: P2) -> windows_core::Result<u16>
    where
        P2: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyMetadataForFrames)(windows_core::Interface::as_raw(self), copyframecount, copymode, dstmetadataitems.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataCopier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyMetadataForFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u16, SpatialAudioMetadataCopyMode, *mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataCopier_Impl: windows_core::IUnknownImpl {
    fn Open(&self, metadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn CopyMetadataForFrames(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<u16>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataCopier_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataCopier_Impl::Open(this, core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn CopyMetadataForFrames<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: *mut core::ffi::c_void, itemscopied: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataCopier_Impl::CopyMetadataForFrames(this, core::mem::transmute_copy(&copyframecount), core::mem::transmute_copy(&copymode), core::mem::transmute_copy(&dstmetadataitems)) {
                    Ok(ok__) => {
                        itemscopied.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataCopier_Impl::Close(this).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioMetadataCopier {}
windows_core::imp::define_interface!(ISpatialAudioMetadataItems, ISpatialAudioMetadataItems_Vtbl, 0xbcd7c78f_3098_4f22_b547_a2f25a381269);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataItems, windows_core::IUnknown);
impl ISpatialAudioMetadataItems {
    pub unsafe fn GetFrameCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxItemCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxValueBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxValueBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInfo(&self) -> windows_core::Result<SpatialAudioMetadataItemsInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxValueBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialAudioMetadataItemsInfo) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataItems_Impl: windows_core::IUnknownImpl {
    fn GetFrameCount(&self) -> windows_core::Result<u16>;
    fn GetItemCount(&self) -> windows_core::Result<u16>;
    fn GetMaxItemCount(&self) -> windows_core::Result<u16>;
    fn GetMaxValueBufferLength(&self) -> windows_core::Result<u32>;
    fn GetInfo(&self) -> windows_core::Result<SpatialAudioMetadataItemsInfo>;
}
impl ISpatialAudioMetadataItems_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameCount<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framecount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetFrameCount(this) {
                    Ok(ok__) => {
                        framecount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemCount<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemcount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetItemCount(this) {
                    Ok(ok__) => {
                        itemcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxItemCount<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxitemcount: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetMaxItemCount(this) {
                    Ok(ok__) => {
                        maxitemcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxvaluebufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetMaxValueBufferLength(this) {
                    Ok(ok__) => {
                        maxvaluebufferlength.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInfo<Identity: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioMetadataItems_Impl::GetInfo(this) {
                    Ok(ok__) => {
                        info.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISpatialAudioMetadataItems {}
windows_core::imp::define_interface!(ISpatialAudioMetadataItemsBuffer, ISpatialAudioMetadataItemsBuffer_Vtbl, 0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataItemsBuffer, windows_core::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    pub unsafe fn AttachToBuffer(&self, buffer: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachToBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AttachToPopulatedBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn DetachBuffer(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub AttachToPopulatedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataItemsBuffer_Impl: windows_core::IUnknownImpl {
    fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> windows_core::Result<()>;
    fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> windows_core::Result<()>;
    fn DetachBuffer(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AttachToBuffer<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataItemsBuffer_Impl::AttachToBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
            }
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataItemsBuffer_Impl::AttachToPopulatedBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataItemsBuffer_Impl::DetachBuffer(this).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioMetadataItemsBuffer {}
windows_core::imp::define_interface!(ISpatialAudioMetadataReader, ISpatialAudioMetadataReader_Vtbl, 0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataReader, windows_core::IUnknown);
impl ISpatialAudioMetadataReader {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()).ok() }
    }
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadNextItem)(windows_core::Interface::as_raw(self), commandcount as _, frameoffset as _).ok() }
    }
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadNextItemCommand)(windows_core::Interface::as_raw(self), commandid as _, valuebuffer as _, maxvaluebufferlength, valuebufferlength as _).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub ReadNextItemCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataReader_Impl: windows_core::IUnknownImpl {
    fn Open(&self, metadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::Result<()>;
    fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataReader_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::Open(this, core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn ReadNextItem<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::ReadNextItem(this, core::mem::transmute_copy(&commandcount), core::mem::transmute_copy(&frameoffset)).into()
            }
        }
        unsafe extern "system" fn ReadNextItemCommand<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::ReadNextItemCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&maxvaluebufferlength), core::mem::transmute_copy(&valuebufferlength)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataReader_Impl::Close(this).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioMetadataReader {}
windows_core::imp::define_interface!(ISpatialAudioMetadataWriter, ISpatialAudioMetadataWriter_Vtbl, 0x1b17ca01_2955_444d_a430_537dc589a844);
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataWriter, windows_core::IUnknown);
impl ISpatialAudioMetadataWriter {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()).ok() }
    }
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteNextItem)(windows_core::Interface::as_raw(self), frameoffset).ok() }
    }
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: Option<*const core::ffi::c_void>, valuebufferlength: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteNextItemCommand)(windows_core::Interface::as_raw(self), commandid, valuebuffer.unwrap_or(core::mem::zeroed()) as _, valuebufferlength).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteNextItemCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioMetadataWriter_Impl: windows_core::IUnknownImpl {
    fn Open(&self, metadataitems: windows_core::Ref<ISpatialAudioMetadataItems>) -> windows_core::Result<()>;
    fn WriteNextItem(&self, frameoffset: u16) -> windows_core::Result<()>;
    fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioMetadataWriter_Vtbl {
    pub const fn new<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::Open(this, core::mem::transmute_copy(&metadataitems)).into()
            }
        }
        unsafe extern "system" fn WriteNextItem<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameoffset: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::WriteNextItem(this, core::mem::transmute_copy(&frameoffset)).into()
            }
        }
        unsafe extern "system" fn WriteNextItemCommand<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::WriteNextItemCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&valuebufferlength)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioMetadataWriter_Impl::Close(this).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioMetadataWriter {}
windows_core::imp::define_interface!(ISpatialAudioObject, ISpatialAudioObject_Vtbl, 0xdde28967_521b_46e5_8f00_bd6f2bc8ab1d);
impl core::ops::Deref for ISpatialAudioObject {
    type Target = ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObject, windows_core::IUnknown, ISpatialAudioObjectBase);
impl ISpatialAudioObject {
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z).ok() }
    }
    pub unsafe fn SetVolume(&self, volume: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObject_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObject_Impl: ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()>;
    fn SetVolume(&self, volume: f32) -> windows_core::Result<()>;
}
impl ISpatialAudioObject_Vtbl {
    pub const fn new<Identity: ISpatialAudioObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPosition<Identity: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObject_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObject_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioObject {}
windows_core::imp::define_interface!(ISpatialAudioObjectBase, ISpatialAudioObjectBase_Vtbl, 0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectBase, windows_core::IUnknown);
impl ISpatialAudioObjectBase {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer as _, bufferlength as _).ok() }
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEndOfStream)(windows_core::Interface::as_raw(self), framecount).ok() }
    }
    pub unsafe fn IsActive(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAudioObjectType(&self) -> windows_core::Result<AudioObjectType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAudioObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioObjectType) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectBase_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::Result<()>;
    fn SetEndOfStream(&self, framecount: u32) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetAudioObjectType(&self) -> windows_core::Result<AudioObjectType>;
}
impl ISpatialAudioObjectBase_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectBase_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
            }
        }
        unsafe extern "system" fn SetEndOfStream<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectBase_Impl::SetEndOfStream(this, core::mem::transmute_copy(&framecount)).into()
            }
        }
        unsafe extern "system" fn IsActive<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isactive: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectBase_Impl::IsActive(this) {
                    Ok(ok__) => {
                        isactive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAudioObjectType<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectBase_Impl::GetAudioObjectType(this) {
                    Ok(ok__) => {
                        audioobjecttype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISpatialAudioObjectBase {}
windows_core::imp::define_interface!(ISpatialAudioObjectForHrtf, ISpatialAudioObjectForHrtf_Vtbl, 0xd7436ade_1978_4e14_aba0_555bd8eb83b4);
impl core::ops::Deref for ISpatialAudioObjectForHrtf {
    type Target = ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectForHrtf, windows_core::IUnknown, ISpatialAudioObjectBase);
impl ISpatialAudioObjectForHrtf {
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z).ok() }
    }
    pub unsafe fn SetGain(&self, gain: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGain)(windows_core::Interface::as_raw(self), gain).ok() }
    }
    pub unsafe fn SetOrientation(&self, orientation: *const *const f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOrientation)(windows_core::Interface::as_raw(self), orientation).ok() }
    }
    pub unsafe fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironment)(windows_core::Interface::as_raw(self), environment).ok() }
    }
    pub unsafe fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDistanceDecay)(windows_core::Interface::as_raw(self), distancedecay).ok() }
    }
    pub unsafe fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDirectivity)(windows_core::Interface::as_raw(self), directivity).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForHrtf_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const f32) -> windows_core::HRESULT,
    pub SetEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialAudioHrtfEnvironmentType) -> windows_core::HRESULT,
    pub SetDistanceDecay: unsafe extern "system" fn(*mut core::ffi::c_void, *const SpatialAudioHrtfDistanceDecay) -> windows_core::HRESULT,
    pub SetDirectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *const SpatialAudioHrtfDirectivityUnion) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectForHrtf_Impl: ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()>;
    fn SetGain(&self, gain: f32) -> windows_core::Result<()>;
    fn SetOrientation(&self, orientation: *const *const f32) -> windows_core::Result<()>;
    fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> windows_core::Result<()>;
    fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> windows_core::Result<()>;
    fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> windows_core::Result<()>;
}
impl ISpatialAudioObjectForHrtf_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPosition<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForHrtf_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
            }
        }
        unsafe extern "system" fn SetGain<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gain: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForHrtf_Impl::SetGain(this, core::mem::transmute_copy(&gain)).into()
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, orientation: *const *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForHrtf_Impl::SetOrientation(this, core::mem::transmute_copy(&orientation)).into()
            }
        }
        unsafe extern "system" fn SetEnvironment<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForHrtf_Impl::SetEnvironment(this, core::mem::transmute_copy(&environment)).into()
            }
        }
        unsafe extern "system" fn SetDistanceDecay<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForHrtf_Impl::SetDistanceDecay(this, core::mem::transmute_copy(&distancedecay)).into()
            }
        }
        unsafe extern "system" fn SetDirectivity<Identity: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForHrtf_Impl::SetDirectivity(this, core::mem::transmute_copy(&directivity)).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioObjectForHrtf {}
windows_core::imp::define_interface!(ISpatialAudioObjectForMetadataCommands, ISpatialAudioObjectForMetadataCommands_Vtbl, 0x0df2c94b_f5f9_472d_af6b_c46e0ac9cd05);
impl core::ops::Deref for ISpatialAudioObjectForMetadataCommands {
    type Target = ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectForMetadataCommands, windows_core::IUnknown, ISpatialAudioObjectBase);
impl ISpatialAudioObjectForMetadataCommands {
    pub unsafe fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: Option<*const core::ffi::c_void>, valuebufferlength: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteNextMetadataCommand)(windows_core::Interface::as_raw(self), commandid, valuebuffer.unwrap_or(core::mem::zeroed()) as _, valuebufferlength).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub WriteNextMetadataCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectForMetadataCommands_Impl: ISpatialAudioObjectBase_Impl {
    fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::Result<()>;
}
impl ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteNextMetadataCommand<Identity: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandid: u8, valuebuffer: *const core::ffi::c_void, valuebufferlength: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectForMetadataCommands_Impl::WriteNextMetadataCommand(this, core::mem::transmute_copy(&commandid), core::mem::transmute_copy(&valuebuffer), core::mem::transmute_copy(&valuebufferlength)).into()
            }
        }
        Self { base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(), WriteNextMetadataCommand: WriteNextMetadataCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataCommands as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObjectForMetadataCommands {}
windows_core::imp::define_interface!(ISpatialAudioObjectForMetadataItems, ISpatialAudioObjectForMetadataItems_Vtbl, 0xddea49ff_3bc0_4377_8aad_9fbcfd808566);
impl core::ops::Deref for ISpatialAudioObjectForMetadataItems {
    type Target = ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectForMetadataItems, windows_core::IUnknown, ISpatialAudioObjectBase);
impl ISpatialAudioObjectForMetadataItems {
    pub unsafe fn GetSpatialAudioMetadataItems(&self) -> windows_core::Result<ISpatialAudioMetadataItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpatialAudioMetadataItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataItems_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub GetSpatialAudioMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectForMetadataItems_Impl: ISpatialAudioObjectBase_Impl {
    fn GetSpatialAudioMetadataItems(&self) -> windows_core::Result<ISpatialAudioMetadataItems>;
}
impl ISpatialAudioObjectForMetadataItems_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Identity: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectForMetadataItems_Impl::GetSpatialAudioMetadataItems(this) {
                    Ok(ok__) => {
                        metadataitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(), GetSpatialAudioMetadataItems: GetSpatialAudioMetadataItems::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataItems as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObjectForMetadataItems {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStream, ISpatialAudioObjectRenderStream_Vtbl, 0xbab5f473_b423_477b_85f5_b5a332a04153);
impl core::ops::Deref for ISpatialAudioObjectRenderStream {
    type Target = ISpatialAudioObjectRenderStreamBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStream, windows_core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ISpatialAudioObjectRenderStream {
    pub unsafe fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObject)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStream_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObject: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStream_Impl: ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObject>;
}
impl ISpatialAudioObjectRenderStream_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioObject<Identity: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStream_Impl::ActivateSpatialAudioObject(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStream {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamBase, ISpatialAudioObjectRenderStreamBase_Vtbl, 0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamBase, windows_core::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableDynamicObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginUpdatingAudioObjects)(windows_core::Interface::as_raw(self), availabledynamicobjectcount as _, framecountperbuffer as _).ok() }
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndUpdatingAudioObjects)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableDynamicObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginUpdatingAudioObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub EndUpdatingAudioObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStreamBase_Impl: windows_core::IUnknownImpl {
    fn GetAvailableDynamicObjectCount(&self) -> windows_core::Result<u32>;
    fn GetService(&self, riid: *const windows_core::GUID, service: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::Result<()>;
    fn EndUpdatingAudioObjects(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamBase_Impl::GetAvailableDynamicObjectCount(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetService<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::GetService(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&service)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::BeginUpdatingAudioObjects(this, core::mem::transmute_copy(&availabledynamicobjectcount), core::mem::transmute_copy(&framecountperbuffer)).into()
            }
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::EndUpdatingAudioObjects(this).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamBase {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamForHrtf, ISpatialAudioObjectRenderStreamForHrtf_Vtbl, 0xe08deef9_5363_406e_9fdc_080ee247bbe0);
impl core::ops::Deref for ISpatialAudioObjectRenderStreamForHrtf {
    type Target = ISpatialAudioObjectRenderStreamBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamForHrtf, windows_core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ISpatialAudioObjectRenderStreamForHrtf {
    pub unsafe fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForHrtf> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForHrtf)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForHrtf: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStreamForHrtf_Impl: ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForHrtf>;
}
impl ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioObjectForHrtf<Identity: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamForHrtf_Impl::ActivateSpatialAudioObjectForHrtf(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamForHrtf {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamForMetadata, ISpatialAudioObjectRenderStreamForMetadata_Vtbl, 0xbbc9c907_48d5_4a2e_a0c7_f7f0d67c1fb1);
impl core::ops::Deref for ISpatialAudioObjectRenderStreamForMetadata {
    type Target = ISpatialAudioObjectRenderStreamBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamForMetadata, windows_core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ISpatialAudioObjectRenderStreamForMetadata {
    pub unsafe fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataCommands> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataCommands)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataItems)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForMetadataCommands: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioObjectForMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStreamForMetadata_Impl: ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataCommands>;
    fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataItems>;
}
impl ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamForMetadata_Impl::ActivateSpatialAudioObjectForMetadataCommands(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Identity: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamForMetadata_Impl::ActivateSpatialAudioObjectForMetadataItems(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamForMetadata {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamNotify, ISpatialAudioObjectRenderStreamNotify_Vtbl, 0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamNotify, windows_core::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    pub unsafe fn OnAvailableDynamicObjectCountChange<P0>(&self, sender: P0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioObjectRenderStreamBase>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAvailableDynamicObjectCountChange)(windows_core::Interface::as_raw(self), sender.param().abi(), hnscompliancedeadlinetime, availabledynamicobjectcountchange).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAvailableDynamicObjectCountChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStreamNotify_Impl: windows_core::IUnknownImpl {
    fn OnAvailableDynamicObjectCountChange(&self, sender: windows_core::Ref<ISpatialAudioObjectRenderStreamBase>, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::Result<()>;
}
impl ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Identity: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamNotify_Impl::OnAvailableDynamicObjectCountChange(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&hnscompliancedeadlinetime), core::mem::transmute_copy(&availabledynamicobjectcountchange)).into()
            }
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
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamNotify {}
windows_core::imp::define_interface!(ISubunit, ISubunit_Vtbl, 0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
windows_core::imp::interface_hierarchy!(ISubunit, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISubunit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISubunit_Impl: windows_core::IUnknownImpl {}
impl ISubunit_Vtbl {
    pub const fn new<Identity: ISubunit_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISubunit as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISubunit {}
pub const In: DataFlow = DataFlow(0i32);
pub type LPACMDRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub const LineLevel: EndpointFormFactor = EndpointFormFactor(2i32);
pub const Low: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(1i32);
pub const MEVT_COMMENT: u8 = 130u8;
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
pub const MEVT_F_LONG: i32 = -2147483648i32;
pub const MEVT_F_SHORT: i32 = 0i32;
pub const MEVT_LONGMSG: u8 = 128u8;
pub const MEVT_NOP: u8 = 2u8;
pub const MEVT_SHORTMSG: u8 = 0u8;
pub const MEVT_TEMPO: u8 = 1u8;
pub const MEVT_VERSION: u8 = 132u8;
pub const MHDR_DONE: u32 = 1u32;
pub const MHDR_INQUEUE: u32 = 4u32;
pub const MHDR_ISSTRM: u32 = 8u32;
pub const MHDR_PREPARED: u32 = 2u32;
pub const MIDICAPS_CACHE: u32 = 4u32;
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
pub const MIDICAPS_STREAM: u32 = 8u32;
pub const MIDICAPS_VOLUME: u32 = 1u32;
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
pub const MIDIERR_LASTERROR: u32 = 71u32;
pub const MIDIERR_NODEVICE: u32 = 68u32;
pub const MIDIERR_NOMAP: u32 = 66u32;
pub const MIDIERR_NOTREADY: u32 = 67u32;
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIHDR {
    pub lpData: windows_core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub lpNext: *mut MIDIHDR,
    pub reserved: usize,
    pub dwOffset: u32,
    pub dwReserved: [usize; 8],
}
impl Default for MIDIHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub dwSupport: u32,
}
impl Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIDIPATCHSIZE: u32 = 128u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
pub const MIDIPROP_GET: i32 = 1073741824i32;
pub const MIDIPROP_SET: i32 = -2147483648i32;
pub const MIDIPROP_TEMPO: i32 = 2i32;
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
pub const MIDISTRM_ERROR: i32 = -2i32;
pub const MIDI_CACHE_ALL: u32 = 1u32;
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
pub const MIDI_CACHE_QUERY: u32 = 3u32;
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
pub const MIDI_UNCACHE: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
impl MIDI_WAVE_OPEN_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLA {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [i8; 16],
    pub szName: [i8; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
impl Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_0 {
    pub Anonymous1: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: *mut core::ffi::c_void,
}
impl Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::super::Foundation::HWND,
    pub cMultipleItems: u32,
}
impl Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [i8; 64],
}
impl Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLW {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Bounds: MIXERCONTROLW_0,
    pub Metrics: MIXERCONTROLW_1,
}
impl Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_0 {
    pub Anonymous1: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINEA {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [i8; 16],
    pub szName: [i8; 64],
    pub Target: MIXERLINEA_0,
}
impl Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINEA_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
}
impl Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINECONTROLSA {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLA,
}
impl Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLW,
}
impl Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINEW {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Target: MIXERLINEW_0,
}
impl Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
}
impl Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(1u32);
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(5u32);
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(3u32);
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4u32);
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(6u32);
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(0u32);
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(8u32);
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(7u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4106u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4105u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4101u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4097u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4098u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4099u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4103u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4100u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4102u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4096u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4104u32);
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
pub const MIXERR_INVALLINE: u32 = 1024u32;
pub const MIXERR_INVALVALUE: u32 = 1026u32;
pub const MIXERR_LASTERROR: u32 = 1026u32;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
pub const MMDeviceEnumerator: windows_core::GUID = windows_core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
pub const MOD_FMSYNTH: u32 = 4u32;
pub const MOD_MAPPER: u32 = 5u32;
pub const MOD_MIDIPORT: u32 = 1u32;
pub const MOD_SQSYNTH: u32 = 3u32;
pub const MOD_SWSYNTH: u32 = 7u32;
pub const MOD_SYNTH: u32 = 2u32;
pub const MOD_WAVETABLE: u32 = 6u32;
pub const Microphone: EndpointFormFactor = EndpointFormFactor(4i32);
pub const Muted: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(0i32);
pub const Out: DataFlow = DataFlow(1i32);
pub type PAudioStateMonitorCallback = Option<unsafe extern "system" fn(audiostatemonitor: windows_core::Ref<IAudioStateMonitor>, context: *const core::ffi::c_void)>;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0 };
pub const PKEY_AudioEndpointLogo_IconPath: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1 };
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1 };
pub const PKEY_AudioEndpointSettings_MenuText: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0 };
pub const PKEY_AudioEndpoint_Association: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2 };
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1 };
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9 };
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5 };
pub const PKEY_AudioEndpoint_FormFactor: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0 };
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6 };
pub const PKEY_AudioEndpoint_GUID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4 };
pub const PKEY_AudioEndpoint_JackSubType: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8 };
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3 };
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7 };
pub const PKEY_AudioEngine_DeviceFormat: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0 };
pub const PKEY_AudioEngine_OEMFormat: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3 };
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(1i32);
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PartType(pub i32);
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
pub const SND_ALIAS: SND_FLAGS = SND_FLAGS(65536u32);
pub const SND_ALIAS_ID: SND_FLAGS = SND_FLAGS(1114112u32);
pub const SND_ALIAS_START: u32 = 0u32;
pub const SND_APPLICATION: SND_FLAGS = SND_FLAGS(128u32);
pub const SND_ASYNC: SND_FLAGS = SND_FLAGS(1u32);
pub const SND_FILENAME: SND_FLAGS = SND_FLAGS(131072u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SND_FLAGS(pub u32);
impl SND_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SND_LOOP: SND_FLAGS = SND_FLAGS(8u32);
pub const SND_MEMORY: SND_FLAGS = SND_FLAGS(4u32);
pub const SND_NODEFAULT: SND_FLAGS = SND_FLAGS(2u32);
pub const SND_NOSTOP: SND_FLAGS = SND_FLAGS(16u32);
pub const SND_NOWAIT: SND_FLAGS = SND_FLAGS(8192u32);
pub const SND_PURGE: SND_FLAGS = SND_FLAGS(64u32);
pub const SND_RESOURCE: SND_FLAGS = SND_FLAGS(262148u32);
pub const SND_RING: i32 = 1048576i32;
pub const SND_SENTRY: SND_FLAGS = SND_FLAGS(524288u32);
pub const SND_SYNC: SND_FLAGS = SND_FLAGS(0u32);
pub const SND_SYSTEM: SND_FLAGS = SND_FLAGS(2097152u32);
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub i32);
impl SPATIAL_AUDIO_STREAM_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(0i32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(1i32);
pub const SPDIF: EndpointFormFactor = EndpointFormFactor(8i32);
pub const SPTLAUDCLNT_E_DESTROYED: windows_core::HRESULT = windows_core::HRESULT(0x88890100_u32 as _);
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: windows_core::HRESULT = windows_core::HRESULT(0x88890105_u32 as _);
pub const SPTLAUDCLNT_E_INTERNAL: windows_core::HRESULT = windows_core::HRESULT(0x8889010D_u32 as _);
pub const SPTLAUDCLNT_E_INVALID_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0x88890108_u32 as _);
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88890106_u32 as _);
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0x88890103_u32 as _);
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x8889010C_u32 as _);
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: windows_core::HRESULT = windows_core::HRESULT(0x88890101_u32 as _);
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x88890104_u32 as _);
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: windows_core::HRESULT = windows_core::HRESULT(0x88890102_u32 as _);
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8889010B_u32 as _);
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x88890107_u32 as _);
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: windows_core::HRESULT = windows_core::HRESULT(0x8889010A_u32 as _);
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x88890214_u32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x88890207_u32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x88890208_u32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x88890224_u32 as _);
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: windows_core::HRESULT = windows_core::HRESULT(0x88890222_u32 as _);
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x88890200_u32 as _);
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x88890215_u32 as _);
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x88890223_u32 as _);
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x88890209_u32 as _);
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x88890218_u32 as _);
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: windows_core::HRESULT = windows_core::HRESULT(0x88890202_u32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x88890213_u32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: windows_core::HRESULT = windows_core::HRESULT(0x88890225_u32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x88890211_u32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: windows_core::HRESULT = windows_core::HRESULT(0x88890219_u32 as _);
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: windows_core::HRESULT = windows_core::HRESULT(0x88890205_u32 as _);
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x88890203_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x88890216_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: windows_core::HRESULT = windows_core::HRESULT(0x88890220_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x88890210_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: windows_core::HRESULT = windows_core::HRESULT(0x88890212_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: windows_core::HRESULT = windows_core::HRESULT(0x88890221_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: windows_core::HRESULT = windows_core::HRESULT(0x88890206_u32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: windows_core::HRESULT = windows_core::HRESULT(0x88890217_u32 as _);
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x88890201_u32 as _);
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x88890204_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: windows_core::GUID,
    pub appId: windows_core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
#[repr(C, packed(1))]
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
}
impl Default for SpatialAudioHrtfActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
impl Default for SpatialAudioHrtfActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union SpatialAudioHrtfDirectivityUnion {
    pub Cone: SpatialAudioHrtfDirectivityCone,
    pub Cardiod: SpatialAudioHrtfDirectivityCardioid,
    pub Omni: SpatialAudioHrtfDirectivity,
}
impl Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(1i32);
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(2i32);
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(0i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(1i32);
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(4i32);
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(2i32);
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(1i32);
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(3i32);
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialAudioMetadataCopyMode(pub i32);
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(1i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(3i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(2i32);
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(0i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(0i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(2i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(1i32);
#[repr(C, packed(1))]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
}
impl Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
impl Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: windows_core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: windows_core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
pub const Subunit: PartType = PartType(1i32);
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: windows_core::PCWSTR = windows_core::w!("VAD\\Process_Loopback");
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
pub const WAVECAPS_PITCH: u32 = 1u32;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
pub const WAVECAPS_SYNC: u32 = 16u32;
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: windows_core::GUID,
}
impl Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEHDR {
    pub lpData: windows_core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: *mut WAVEHDR,
    pub reserved: usize,
}
impl Default for WAVEHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: windows_core::GUID,
    pub ProductGuid: windows_core::GUID,
    pub NameGuid: windows_core::GUID,
}
impl Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
pub const WAVERR_BADFORMAT: u32 = 32u32;
pub const WAVERR_LASTERROR: u32 = 35u32;
pub const WAVERR_STILLPLAYING: u32 = 33u32;
pub const WAVERR_SYNC: u32 = 35u32;
pub const WAVERR_UNPREPARED: u32 = 34u32;
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(2u32);
pub const WAVE_FORMAT_1M08: u32 = 1u32;
pub const WAVE_FORMAT_1M16: u32 = 4u32;
pub const WAVE_FORMAT_1S08: u32 = 2u32;
pub const WAVE_FORMAT_1S16: u32 = 8u32;
pub const WAVE_FORMAT_2M08: u32 = 16u32;
pub const WAVE_FORMAT_2M16: u32 = 64u32;
pub const WAVE_FORMAT_2S08: u32 = 32u32;
pub const WAVE_FORMAT_2S16: u32 = 128u32;
pub const WAVE_FORMAT_44M08: u32 = 256u32;
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
pub const WAVE_FORMAT_44S08: u32 = 512u32;
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
pub const WAVE_FORMAT_4M08: u32 = 256u32;
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
pub const WAVE_FORMAT_4S08: u32 = 512u32;
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(8u32);
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(9u32);
pub const WAVE_FORMAT_PCM: u32 = 1u32;
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(1u32);
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(4u32);
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(16u32);
pub const WAVE_MAPPER: u32 = 4294967295u32;
pub const WHDR_BEGINLOOP: u32 = 4u32;
pub const WHDR_DONE: u32 = 1u32;
pub const WHDR_ENDLOOP: u32 = 8u32;
pub const WHDR_INQUEUE: u32 = 16u32;
pub const WHDR_PREPARED: u32 = 2u32;
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
pub const eAll: EDataFlow = EDataFlow(2i32);
pub const eCapture: EDataFlow = EDataFlow(1i32);
pub const eCommunications: ERole = ERole(2i32);
pub const eConsole: ERole = ERole(0i32);
pub const eMultimedia: ERole = ERole(1i32);
pub const eRender: EDataFlow = EDataFlow(0i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct tACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
impl Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
