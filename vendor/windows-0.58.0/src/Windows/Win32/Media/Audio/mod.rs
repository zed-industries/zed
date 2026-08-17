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
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<P0, P1>(deviceinterfacepath: P0, riid: *const windows_core::GUID, activationparams: Option<*const windows_core::PROPVARIANT>, completionhandler: P1) -> windows_core::Result<IActivateAudioInterfaceAsyncOperation>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IActivateAudioInterfaceCompletionHandler>,
{
    windows_targets::link!("mmdevapi.dll" "system" fn ActivateAudioInterfaceAsync(deviceinterfacepath : windows_core::PCWSTR, riid : *const windows_core::GUID, activationparams : *const core::mem::MaybeUninit < windows_core::PROPVARIANT >, completionhandler : * mut core::ffi::c_void, activationoperation : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ActivateAudioInterfaceAsync(deviceinterfacepath.param().abi(), riid, core::mem::transmute(activationparams.unwrap_or(std::ptr::null())), completionhandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoRegisterMessageFilter<P0>(lpmessagefilter: P0, lplpmessagefilter: Option<*mut Option<IMessageFilter>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMessageFilter>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : * mut core::ffi::c_void, lplpmessagefilter : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoRegisterMessageFilter(lpmessagefilter.param().abi(), core::mem::transmute(lplpmessagefilter.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitor() -> windows_core::Result<IAudioStateMonitor> {
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitor(audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateCaptureAudioStateMonitor(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<IAudioStateMonitor> {
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategory(category : AUDIO_STREAM_CATEGORY, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateCaptureAudioStateMonitorForCategory(category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId<P0>(category: AUDIO_STREAM_CATEGORY, deviceid: P0) -> windows_core::Result<IAudioStateMonitor>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category : AUDIO_STREAM_CATEGORY, deviceid : windows_core::PCWSTR, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category, deviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> windows_core::Result<IAudioStateMonitor> {
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category : AUDIO_STREAM_CATEGORY, role : ERole, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitor() -> windows_core::Result<IAudioStateMonitor> {
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitor(audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateRenderAudioStateMonitor(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<IAudioStateMonitor> {
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategory(category : AUDIO_STREAM_CATEGORY, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateRenderAudioStateMonitorForCategory(category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceId<P0>(category: AUDIO_STREAM_CATEGORY, deviceid: P0) -> windows_core::Result<IAudioStateMonitor>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category : AUDIO_STREAM_CATEGORY, deviceid : windows_core::PCWSTR, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateRenderAudioStateMonitorForCategoryAndDeviceId(category, deviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> windows_core::Result<IAudioStateMonitor> {
    windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category : AUDIO_STREAM_CATEGORY, role : ERole, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn PlaySoundA<P0, P1>(pszsound: P0, hmod: P1, fdwsound: SND_FLAGS) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("winmm.dll" "system" fn PlaySoundA(pszsound : windows_core::PCSTR, hmod : super::super::Foundation:: HMODULE, fdwsound : SND_FLAGS) -> super::super::Foundation:: BOOL);
    PlaySoundA(pszsound.param().abi(), hmod.param().abi(), fdwsound)
}
#[inline]
pub unsafe fn PlaySoundW<P0, P1>(pszsound: P0, hmod: P1, fdwsound: SND_FLAGS) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("winmm.dll" "system" fn PlaySoundW(pszsound : windows_core::PCWSTR, hmod : super::super::Foundation:: HMODULE, fdwsound : SND_FLAGS) -> super::super::Foundation:: BOOL);
    PlaySoundW(pszsound.param().abi(), hmod.param().abi(), fdwsound)
}
#[inline]
pub unsafe fn acmDriverAddA<P0, P1>(phadid: *mut HACMDRIVERID, hinstmodule: P0, lparam: P1, dwpriority: u32, fdwadd: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverAddA(phadid : *mut HACMDRIVERID, hinstmodule : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM, dwpriority : u32, fdwadd : u32) -> u32);
    acmDriverAddA(phadid, hinstmodule.param().abi(), lparam.param().abi(), dwpriority, fdwadd)
}
#[inline]
pub unsafe fn acmDriverAddW<P0, P1>(phadid: *mut HACMDRIVERID, hinstmodule: P0, lparam: P1, dwpriority: u32, fdwadd: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverAddW(phadid : *mut HACMDRIVERID, hinstmodule : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM, dwpriority : u32, fdwadd : u32) -> u32);
    acmDriverAddW(phadid, hinstmodule.param().abi(), lparam.param().abi(), dwpriority, fdwadd)
}
#[inline]
pub unsafe fn acmDriverClose<P0>(had: P0, fdwclose: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverClose(had : HACMDRIVER, fdwclose : u32) -> u32);
    acmDriverClose(had.param().abi(), fdwclose)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsA<P0>(hadid: P0, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVERID>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverDetailsA(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSA, fdwdetails : u32) -> u32);
    acmDriverDetailsA(hadid.param().abi(), padd, fdwdetails)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsW<P0>(hadid: P0, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVERID>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverDetailsW(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSW, fdwdetails : u32) -> u32);
    acmDriverDetailsW(hadid.param().abi(), padd, fdwdetails)
}
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32 {
    windows_targets::link!("msacm32.dll" "system" fn acmDriverEnum(fncallback : ACMDRIVERENUMCB, dwinstance : usize, fdwenum : u32) -> u32);
    acmDriverEnum(fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmDriverID<P0>(hao: P0, phadid: *mut HACMDRIVERID, fdwdriverid: u32) -> u32
where
    P0: windows_core::Param<HACMOBJ>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverID(hao : HACMOBJ, phadid : *mut HACMDRIVERID, fdwdriverid : u32) -> u32);
    acmDriverID(hao.param().abi(), phadid, fdwdriverid)
}
#[inline]
pub unsafe fn acmDriverMessage<P0, P1, P2>(had: P0, umsg: u32, lparam1: P1, lparam2: P2) -> super::super::Foundation::LRESULT
where
    P0: windows_core::Param<HACMDRIVER>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverMessage(had : HACMDRIVER, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
    acmDriverMessage(had.param().abi(), umsg, lparam1.param().abi(), lparam2.param().abi())
}
#[inline]
pub unsafe fn acmDriverOpen<P0>(phad: *mut HACMDRIVER, hadid: P0, fdwopen: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVERID>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverOpen(phad : *mut HACMDRIVER, hadid : HACMDRIVERID, fdwopen : u32) -> u32);
    acmDriverOpen(phad, hadid.param().abi(), fdwopen)
}
#[inline]
pub unsafe fn acmDriverPriority<P0>(hadid: P0, dwpriority: u32, fdwpriority: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVERID>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverPriority(hadid : HACMDRIVERID, dwpriority : u32, fdwpriority : u32) -> u32);
    acmDriverPriority(hadid.param().abi(), dwpriority, fdwpriority)
}
#[inline]
pub unsafe fn acmDriverRemove<P0>(hadid: P0, fdwremove: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVERID>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmDriverRemove(hadid : HACMDRIVERID, fdwremove : u32) -> u32);
    acmDriverRemove(hadid.param().abi(), fdwremove)
}
#[inline]
pub unsafe fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32 {
    windows_targets::link!("msacm32.dll" "system" fn acmFilterChooseA(pafltrc : *mut ACMFILTERCHOOSEA) -> u32);
    acmFilterChooseA(pafltrc)
}
#[inline]
pub unsafe fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32 {
    windows_targets::link!("msacm32.dll" "system" fn acmFilterChooseW(pafltrc : *mut ACMFILTERCHOOSEW) -> u32);
    acmFilterChooseW(pafltrc)
}
#[inline]
pub unsafe fn acmFilterDetailsA<P0>(had: P0, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterDetailsA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fdwdetails : u32) -> u32);
    acmFilterDetailsA(had.param().abi(), pafd, fdwdetails)
}
#[inline]
pub unsafe fn acmFilterDetailsW<P0>(had: P0, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterDetailsW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fdwdetails : u32) -> u32);
    acmFilterDetailsW(had.param().abi(), pafd, fdwdetails)
}
#[inline]
pub unsafe fn acmFilterEnumA<P0>(had: P0, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterEnumA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fncallback : ACMFILTERENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    acmFilterEnumA(had.param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFilterEnumW<P0>(had: P0, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterEnumW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fncallback : ACMFILTERENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    acmFilterEnumW(had.param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFilterTagDetailsA<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fdwdetails : u32) -> u32);
    acmFilterTagDetailsA(had.param().abi(), paftd, fdwdetails)
}
#[inline]
pub unsafe fn acmFilterTagDetailsW<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fdwdetails : u32) -> u32);
    acmFilterTagDetailsW(had.param().abi(), paftd, fdwdetails)
}
#[inline]
pub unsafe fn acmFilterTagEnumA<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterTagEnumA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fncallback : ACMFILTERTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    acmFilterTagEnumA(had.param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFilterTagEnumW<P0>(had: P0, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFilterTagEnumW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fncallback : ACMFILTERTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    acmFilterTagEnumW(had.param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32 {
    windows_targets::link!("msacm32.dll" "system" fn acmFormatChooseA(pafmtc : *mut ACMFORMATCHOOSEA) -> u32);
    acmFormatChooseA(pafmtc)
}
#[inline]
pub unsafe fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32 {
    windows_targets::link!("msacm32.dll" "system" fn acmFormatChooseW(pafmtc : *mut ACMFORMATCHOOSEW) -> u32);
    acmFormatChooseW(pafmtc)
}
#[inline]
pub unsafe fn acmFormatDetailsA<P0>(had: P0, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatDetailsA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fdwdetails : u32) -> u32);
    acmFormatDetailsA(had.param().abi(), pafd, fdwdetails)
}
#[inline]
pub unsafe fn acmFormatDetailsW<P0>(had: P0, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatDetailsW(had : HACMDRIVER, pafd : *mut tACMFORMATDETAILSW, fdwdetails : u32) -> u32);
    acmFormatDetailsW(had.param().abi(), pafd, fdwdetails)
}
#[inline]
pub unsafe fn acmFormatEnumA<P0>(had: P0, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatEnumA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fncallback : ACMFORMATENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    acmFormatEnumA(had.param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFormatEnumW<P0>(had: P0, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatEnumW(had : HACMDRIVER, pafd : *mut tACMFORMATDETAILSW, fncallback : ACMFORMATENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    acmFormatEnumW(had.param().abi(), pafd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFormatSuggest<P0>(had: P0, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatSuggest(had : HACMDRIVER, pwfxsrc : *mut WAVEFORMATEX, pwfxdst : *mut WAVEFORMATEX, cbwfxdst : u32, fdwsuggest : u32) -> u32);
    acmFormatSuggest(had.param().abi(), pwfxsrc, pwfxdst, cbwfxdst, fdwsuggest)
}
#[inline]
pub unsafe fn acmFormatTagDetailsA<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fdwdetails : u32) -> u32);
    acmFormatTagDetailsA(had.param().abi(), paftd, fdwdetails)
}
#[inline]
pub unsafe fn acmFormatTagDetailsW<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fdwdetails : u32) -> u32);
    acmFormatTagDetailsW(had.param().abi(), paftd, fdwdetails)
}
#[inline]
pub unsafe fn acmFormatTagEnumA<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatTagEnumA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fncallback : ACMFORMATTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
    acmFormatTagEnumA(had.param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmFormatTagEnumW<P0>(had: P0, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmFormatTagEnumW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fncallback : ACMFORMATTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
    acmFormatTagEnumW(had.param().abi(), paftd, fncallback, dwinstance, fdwenum)
}
#[inline]
pub unsafe fn acmGetVersion() -> u32 {
    windows_targets::link!("msacm32.dll" "system" fn acmGetVersion() -> u32);
    acmGetVersion()
}
#[inline]
pub unsafe fn acmMetrics<P0>(hao: P0, umetric: u32, pmetric: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<HACMOBJ>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmMetrics(hao : HACMOBJ, umetric : u32, pmetric : *mut core::ffi::c_void) -> u32);
    acmMetrics(hao.param().abi(), umetric, pmetric)
}
#[inline]
pub unsafe fn acmStreamClose<P0>(has: P0, fdwclose: u32) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamClose(has : HACMSTREAM, fdwclose : u32) -> u32);
    acmStreamClose(has.param().abi(), fdwclose)
}
#[inline]
pub unsafe fn acmStreamConvert<P0>(has: P0, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamConvert(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwconvert : u32) -> u32);
    acmStreamConvert(has.param().abi(), pash, fdwconvert)
}
#[inline]
pub unsafe fn acmStreamMessage<P0, P1, P2>(has: P0, umsg: u32, lparam1: P1, lparam2: P2) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
    P1: windows_core::Param<super::super::Foundation::LPARAM>,
    P2: windows_core::Param<super::super::Foundation::LPARAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamMessage(has : HACMSTREAM, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> u32);
    acmStreamMessage(has.param().abi(), umsg, lparam1.param().abi(), lparam2.param().abi())
}
#[inline]
pub unsafe fn acmStreamOpen<P0>(phas: *mut HACMSTREAM, had: P0, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32
where
    P0: windows_core::Param<HACMDRIVER>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamOpen(phas : *mut HACMSTREAM, had : HACMDRIVER, pwfxsrc : *mut WAVEFORMATEX, pwfxdst : *mut WAVEFORMATEX, pwfltr : *mut WAVEFILTER, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
    acmStreamOpen(phas, had.param().abi(), pwfxsrc, pwfxdst, pwfltr, dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn acmStreamPrepareHeader<P0>(has: P0, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamPrepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwprepare : u32) -> u32);
    acmStreamPrepareHeader(has.param().abi(), pash, fdwprepare)
}
#[inline]
pub unsafe fn acmStreamReset<P0>(has: P0, fdwreset: u32) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamReset(has : HACMSTREAM, fdwreset : u32) -> u32);
    acmStreamReset(has.param().abi(), fdwreset)
}
#[inline]
pub unsafe fn acmStreamSize<P0>(has: P0, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamSize(has : HACMSTREAM, cbinput : u32, pdwoutputbytes : *mut u32, fdwsize : u32) -> u32);
    acmStreamSize(has.param().abi(), cbinput, pdwoutputbytes, fdwsize)
}
#[inline]
pub unsafe fn acmStreamUnprepareHeader<P0>(has: P0, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32
where
    P0: windows_core::Param<HACMSTREAM>,
{
    windows_targets::link!("msacm32.dll" "system" fn acmStreamUnprepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwunprepare : u32) -> u32);
    acmStreamUnprepareHeader(has.param().abi(), pash, fdwunprepare)
}
#[inline]
pub unsafe fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn auxGetDevCapsA(udeviceid : usize, pac : *mut AUXCAPSA, cbac : u32) -> u32);
    auxGetDevCapsA(udeviceid, pac, cbac)
}
#[inline]
pub unsafe fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn auxGetDevCapsW(udeviceid : usize, pac : *mut AUXCAPSW, cbac : u32) -> u32);
    auxGetDevCapsW(udeviceid, pac, cbac)
}
#[inline]
pub unsafe fn auxGetNumDevs() -> u32 {
    windows_targets::link!("winmm.dll" "system" fn auxGetNumDevs() -> u32);
    auxGetNumDevs()
}
#[inline]
pub unsafe fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn auxGetVolume(udeviceid : u32, pdwvolume : *mut u32) -> u32);
    auxGetVolume(udeviceid, pdwvolume)
}
#[inline]
pub unsafe fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn auxOutMessage(udeviceid : u32, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    auxOutMessage(udeviceid, umsg, dw1, dw2)
}
#[inline]
pub unsafe fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn auxSetVolume(udeviceid : u32, dwvolume : u32) -> u32);
    auxSetVolume(udeviceid, dwvolume)
}
#[inline]
pub unsafe fn midiConnect<P0, P1>(hmi: P0, hmo: P1, preserved: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<HMIDI>,
    P1: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiConnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> u32);
    midiConnect(hmi.param().abi(), hmo.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn midiDisconnect<P0, P1>(hmi: P0, hmo: P1, preserved: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<HMIDI>,
    P1: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiDisconnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> u32);
    midiDisconnect(hmi.param().abi(), hmo.param().abi(), core::mem::transmute(preserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn midiInAddBuffer<P0>(hmi: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInAddBuffer(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    midiInAddBuffer(hmi.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiInClose<P0>(hmi: P0) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInClose(hmi : HMIDIIN) -> u32);
    midiInClose(hmi.param().abi())
}
#[inline]
pub unsafe fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiInGetDevCapsA(udeviceid : usize, pmic : *mut MIDIINCAPSA, cbmic : u32) -> u32);
    midiInGetDevCapsA(udeviceid, pmic, cbmic)
}
#[inline]
pub unsafe fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiInGetDevCapsW(udeviceid : usize, pmic : *mut MIDIINCAPSW, cbmic : u32) -> u32);
    midiInGetDevCapsW(udeviceid, pmic, cbmic)
}
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiInGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    midiInGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiInGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    midiInGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn midiInGetID<P0>(hmi: P0, pudeviceid: *mut u32) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInGetID(hmi : HMIDIIN, pudeviceid : *mut u32) -> u32);
    midiInGetID(hmi.param().abi(), pudeviceid)
}
#[inline]
pub unsafe fn midiInGetNumDevs() -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiInGetNumDevs() -> u32);
    midiInGetNumDevs()
}
#[inline]
pub unsafe fn midiInMessage<P0>(hmi: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInMessage(hmi : HMIDIIN, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    midiInMessage(hmi.param().abi(), umsg, dw1, dw2)
}
#[inline]
pub unsafe fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiInOpen(phmi : *mut HMIDIIN, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    midiInOpen(phmi, udeviceid, dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn midiInPrepareHeader<P0>(hmi: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInPrepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    midiInPrepareHeader(hmi.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiInReset<P0>(hmi: P0) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInReset(hmi : HMIDIIN) -> u32);
    midiInReset(hmi.param().abi())
}
#[inline]
pub unsafe fn midiInStart<P0>(hmi: P0) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInStart(hmi : HMIDIIN) -> u32);
    midiInStart(hmi.param().abi())
}
#[inline]
pub unsafe fn midiInStop<P0>(hmi: P0) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInStop(hmi : HMIDIIN) -> u32);
    midiInStop(hmi.param().abi())
}
#[inline]
pub unsafe fn midiInUnprepareHeader<P0>(hmi: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDIIN>,
{
    windows_targets::link!("winmm.dll" "system" fn midiInUnprepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    midiInUnprepareHeader(hmi.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiOutCacheDrumPatches<P0>(hmo: P0, upatch: u32, pwkya: &[u16; 128], fucache: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutCacheDrumPatches(hmo : HMIDIOUT, upatch : u32, pwkya : *const u16, fucache : u32) -> u32);
    midiOutCacheDrumPatches(hmo.param().abi(), upatch, core::mem::transmute(pwkya.as_ptr()), fucache)
}
#[inline]
pub unsafe fn midiOutCachePatches<P0>(hmo: P0, ubank: u32, pwpa: &[u16; 128], fucache: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutCachePatches(hmo : HMIDIOUT, ubank : u32, pwpa : *const u16, fucache : u32) -> u32);
    midiOutCachePatches(hmo.param().abi(), ubank, core::mem::transmute(pwpa.as_ptr()), fucache)
}
#[inline]
pub unsafe fn midiOutClose<P0>(hmo: P0) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutClose(hmo : HMIDIOUT) -> u32);
    midiOutClose(hmo.param().abi())
}
#[inline]
pub unsafe fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiOutGetDevCapsA(udeviceid : usize, pmoc : *mut MIDIOUTCAPSA, cbmoc : u32) -> u32);
    midiOutGetDevCapsA(udeviceid, pmoc, cbmoc)
}
#[inline]
pub unsafe fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiOutGetDevCapsW(udeviceid : usize, pmoc : *mut MIDIOUTCAPSW, cbmoc : u32) -> u32);
    midiOutGetDevCapsW(udeviceid, pmoc, cbmoc)
}
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiOutGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    midiOutGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiOutGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    midiOutGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn midiOutGetID<P0>(hmo: P0, pudeviceid: *mut u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutGetID(hmo : HMIDIOUT, pudeviceid : *mut u32) -> u32);
    midiOutGetID(hmo.param().abi(), pudeviceid)
}
#[inline]
pub unsafe fn midiOutGetNumDevs() -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiOutGetNumDevs() -> u32);
    midiOutGetNumDevs()
}
#[inline]
pub unsafe fn midiOutGetVolume<P0>(hmo: P0, pdwvolume: *mut u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutGetVolume(hmo : HMIDIOUT, pdwvolume : *mut u32) -> u32);
    midiOutGetVolume(hmo.param().abi(), pdwvolume)
}
#[inline]
pub unsafe fn midiOutLongMsg<P0>(hmo: P0, pmh: *const MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutLongMsg(hmo : HMIDIOUT, pmh : *const MIDIHDR, cbmh : u32) -> u32);
    midiOutLongMsg(hmo.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiOutMessage<P0>(hmo: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutMessage(hmo : HMIDIOUT, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    midiOutMessage(hmo.param().abi(), umsg, dw1, dw2)
}
#[inline]
pub unsafe fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiOutOpen(phmo : *mut HMIDIOUT, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    midiOutOpen(phmo, udeviceid, dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn midiOutPrepareHeader<P0>(hmo: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutPrepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    midiOutPrepareHeader(hmo.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiOutReset<P0>(hmo: P0) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutReset(hmo : HMIDIOUT) -> u32);
    midiOutReset(hmo.param().abi())
}
#[inline]
pub unsafe fn midiOutSetVolume<P0>(hmo: P0, dwvolume: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutSetVolume(hmo : HMIDIOUT, dwvolume : u32) -> u32);
    midiOutSetVolume(hmo.param().abi(), dwvolume)
}
#[inline]
pub unsafe fn midiOutShortMsg<P0>(hmo: P0, dwmsg: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutShortMsg(hmo : HMIDIOUT, dwmsg : u32) -> u32);
    midiOutShortMsg(hmo.param().abi(), dwmsg)
}
#[inline]
pub unsafe fn midiOutUnprepareHeader<P0>(hmo: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDIOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn midiOutUnprepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    midiOutUnprepareHeader(hmo.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiStreamClose<P0>(hms: P0) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamClose(hms : HMIDISTRM) -> u32);
    midiStreamClose(hms.param().abi())
}
#[inline]
pub unsafe fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: &mut [u32], dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn midiStreamOpen(phms : *mut HMIDISTRM, pudeviceid : *mut u32, cmidi : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
    midiStreamOpen(phms, core::mem::transmute(pudeviceid.as_ptr()), pudeviceid.len().try_into().unwrap(), dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn midiStreamOut<P0>(hms: P0, pmh: *mut MIDIHDR, cbmh: u32) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamOut(hms : HMIDISTRM, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
    midiStreamOut(hms.param().abi(), pmh, cbmh)
}
#[inline]
pub unsafe fn midiStreamPause<P0>(hms: P0) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamPause(hms : HMIDISTRM) -> u32);
    midiStreamPause(hms.param().abi())
}
#[inline]
pub unsafe fn midiStreamPosition<P0>(hms: P0, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamPosition(hms : HMIDISTRM, lpmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
    midiStreamPosition(hms.param().abi(), lpmmt, cbmmt)
}
#[inline]
pub unsafe fn midiStreamProperty<P0>(hms: P0, lppropdata: *mut u8, dwproperty: u32) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamProperty(hms : HMIDISTRM, lppropdata : *mut u8, dwproperty : u32) -> u32);
    midiStreamProperty(hms.param().abi(), lppropdata, dwproperty)
}
#[inline]
pub unsafe fn midiStreamRestart<P0>(hms: P0) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamRestart(hms : HMIDISTRM) -> u32);
    midiStreamRestart(hms.param().abi())
}
#[inline]
pub unsafe fn midiStreamStop<P0>(hms: P0) -> u32
where
    P0: windows_core::Param<HMIDISTRM>,
{
    windows_targets::link!("winmm.dll" "system" fn midiStreamStop(hms : HMIDISTRM) -> u32);
    midiStreamStop(hms.param().abi())
}
#[inline]
pub unsafe fn mixerClose<P0>(hmx: P0) -> u32
where
    P0: windows_core::Param<HMIXER>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerClose(hmx : HMIXER) -> u32);
    mixerClose(hmx.param().abi())
}
#[inline]
pub unsafe fn mixerGetControlDetailsA<P0>(hmxobj: P0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetControlDetailsA(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
    mixerGetControlDetailsA(hmxobj.param().abi(), pmxcd, fdwdetails)
}
#[inline]
pub unsafe fn mixerGetControlDetailsW<P0>(hmxobj: P0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetControlDetailsW(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
    mixerGetControlDetailsW(hmxobj.param().abi(), pmxcd, fdwdetails)
}
#[inline]
pub unsafe fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn mixerGetDevCapsA(umxid : usize, pmxcaps : *mut MIXERCAPSA, cbmxcaps : u32) -> u32);
    mixerGetDevCapsA(umxid, pmxcaps, cbmxcaps)
}
#[inline]
pub unsafe fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn mixerGetDevCapsW(umxid : usize, pmxcaps : *mut MIXERCAPSW, cbmxcaps : u32) -> u32);
    mixerGetDevCapsW(umxid, pmxcaps, cbmxcaps)
}
#[inline]
pub unsafe fn mixerGetID<P0>(hmxobj: P0, pumxid: *mut u32, fdwid: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetID(hmxobj : HMIXEROBJ, pumxid : *mut u32, fdwid : u32) -> u32);
    mixerGetID(hmxobj.param().abi(), pumxid, fdwid)
}
#[inline]
pub unsafe fn mixerGetLineControlsA<P0>(hmxobj: P0, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetLineControlsA(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSA, fdwcontrols : u32) -> u32);
    mixerGetLineControlsA(hmxobj.param().abi(), pmxlc, fdwcontrols)
}
#[inline]
pub unsafe fn mixerGetLineControlsW<P0>(hmxobj: P0, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetLineControlsW(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSW, fdwcontrols : u32) -> u32);
    mixerGetLineControlsW(hmxobj.param().abi(), pmxlc, fdwcontrols)
}
#[inline]
pub unsafe fn mixerGetLineInfoA<P0>(hmxobj: P0, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetLineInfoA(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEA, fdwinfo : u32) -> u32);
    mixerGetLineInfoA(hmxobj.param().abi(), pmxl, fdwinfo)
}
#[inline]
pub unsafe fn mixerGetLineInfoW<P0>(hmxobj: P0, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerGetLineInfoW(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEW, fdwinfo : u32) -> u32);
    mixerGetLineInfoW(hmxobj.param().abi(), pmxl, fdwinfo)
}
#[inline]
pub unsafe fn mixerGetNumDevs() -> u32 {
    windows_targets::link!("winmm.dll" "system" fn mixerGetNumDevs() -> u32);
    mixerGetNumDevs()
}
#[inline]
pub unsafe fn mixerMessage<P0>(hmx: P0, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32
where
    P0: windows_core::Param<HMIXER>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerMessage(hmx : HMIXER, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
    mixerMessage(hmx.param().abi(), umsg, dwparam1, dwparam2)
}
#[inline]
pub unsafe fn mixerOpen(phmx: Option<*mut HMIXER>, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn mixerOpen(phmx : *mut HMIXER, umxid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
    mixerOpen(core::mem::transmute(phmx.unwrap_or(std::ptr::null_mut())), umxid, dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn mixerSetControlDetails<P0>(hmxobj: P0, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32
where
    P0: windows_core::Param<HMIXEROBJ>,
{
    windows_targets::link!("winmm.dll" "system" fn mixerSetControlDetails(hmxobj : HMIXEROBJ, pmxcd : *const MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
    mixerSetControlDetails(hmxobj.param().abi(), pmxcd, fdwdetails)
}
#[inline]
pub unsafe fn sndPlaySoundA<P0>(pszsound: P0, fusound: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("winmm.dll" "system" fn sndPlaySoundA(pszsound : windows_core::PCSTR, fusound : u32) -> super::super::Foundation:: BOOL);
    sndPlaySoundA(pszsound.param().abi(), fusound)
}
#[inline]
pub unsafe fn sndPlaySoundW<P0>(pszsound: P0, fusound: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("winmm.dll" "system" fn sndPlaySoundW(pszsound : windows_core::PCWSTR, fusound : u32) -> super::super::Foundation:: BOOL);
    sndPlaySoundW(pszsound.param().abi(), fusound)
}
#[inline]
pub unsafe fn waveInAddBuffer<P0>(hwi: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInAddBuffer(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    waveInAddBuffer(hwi.param().abi(), pwh, cbwh)
}
#[inline]
pub unsafe fn waveInClose<P0>(hwi: P0) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInClose(hwi : HWAVEIN) -> u32);
    waveInClose(hwi.param().abi())
}
#[inline]
pub unsafe fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveInGetDevCapsA(udeviceid : usize, pwic : *mut WAVEINCAPSA, cbwic : u32) -> u32);
    waveInGetDevCapsA(udeviceid, pwic, cbwic)
}
#[inline]
pub unsafe fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveInGetDevCapsW(udeviceid : usize, pwic : *mut WAVEINCAPSW, cbwic : u32) -> u32);
    waveInGetDevCapsW(udeviceid, pwic, cbwic)
}
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveInGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    waveInGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveInGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    waveInGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn waveInGetID<P0>(hwi: P0, pudeviceid: *const u32) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInGetID(hwi : HWAVEIN, pudeviceid : *const u32) -> u32);
    waveInGetID(hwi.param().abi(), pudeviceid)
}
#[inline]
pub unsafe fn waveInGetNumDevs() -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveInGetNumDevs() -> u32);
    waveInGetNumDevs()
}
#[inline]
pub unsafe fn waveInGetPosition<P0>(hwi: P0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInGetPosition(hwi : HWAVEIN, pmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
    waveInGetPosition(hwi.param().abi(), pmmt, cbmmt)
}
#[inline]
pub unsafe fn waveInMessage<P0>(hwi: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInMessage(hwi : HWAVEIN, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    waveInMessage(hwi.param().abi(), umsg, dw1, dw2)
}
#[inline]
pub unsafe fn waveInOpen(phwi: Option<*mut HWAVEIN>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveInOpen(phwi : *mut HWAVEIN, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    waveInOpen(core::mem::transmute(phwi.unwrap_or(std::ptr::null_mut())), udeviceid, pwfx, dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn waveInPrepareHeader<P0>(hwi: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInPrepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    waveInPrepareHeader(hwi.param().abi(), pwh, cbwh)
}
#[inline]
pub unsafe fn waveInReset<P0>(hwi: P0) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInReset(hwi : HWAVEIN) -> u32);
    waveInReset(hwi.param().abi())
}
#[inline]
pub unsafe fn waveInStart<P0>(hwi: P0) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInStart(hwi : HWAVEIN) -> u32);
    waveInStart(hwi.param().abi())
}
#[inline]
pub unsafe fn waveInStop<P0>(hwi: P0) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInStop(hwi : HWAVEIN) -> u32);
    waveInStop(hwi.param().abi())
}
#[inline]
pub unsafe fn waveInUnprepareHeader<P0>(hwi: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: windows_core::Param<HWAVEIN>,
{
    windows_targets::link!("winmm.dll" "system" fn waveInUnprepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    waveInUnprepareHeader(hwi.param().abi(), pwh, cbwh)
}
#[inline]
pub unsafe fn waveOutBreakLoop<P0>(hwo: P0) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutBreakLoop(hwo : HWAVEOUT) -> u32);
    waveOutBreakLoop(hwo.param().abi())
}
#[inline]
pub unsafe fn waveOutClose<P0>(hwo: P0) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutClose(hwo : HWAVEOUT) -> u32);
    waveOutClose(hwo.param().abi())
}
#[inline]
pub unsafe fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveOutGetDevCapsA(udeviceid : usize, pwoc : *mut WAVEOUTCAPSA, cbwoc : u32) -> u32);
    waveOutGetDevCapsA(udeviceid, pwoc, cbwoc)
}
#[inline]
pub unsafe fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveOutGetDevCapsW(udeviceid : usize, pwoc : *mut WAVEOUTCAPSW, cbwoc : u32) -> u32);
    waveOutGetDevCapsW(udeviceid, pwoc, cbwoc)
}
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveOutGetErrorTextA(mmrerror : u32, psztext : windows_core::PSTR, cchtext : u32) -> u32);
    waveOutGetErrorTextA(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveOutGetErrorTextW(mmrerror : u32, psztext : windows_core::PWSTR, cchtext : u32) -> u32);
    waveOutGetErrorTextW(mmrerror, core::mem::transmute(psztext.as_ptr()), psztext.len().try_into().unwrap())
}
#[inline]
pub unsafe fn waveOutGetID<P0>(hwo: P0, pudeviceid: *mut u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutGetID(hwo : HWAVEOUT, pudeviceid : *mut u32) -> u32);
    waveOutGetID(hwo.param().abi(), pudeviceid)
}
#[inline]
pub unsafe fn waveOutGetNumDevs() -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveOutGetNumDevs() -> u32);
    waveOutGetNumDevs()
}
#[inline]
pub unsafe fn waveOutGetPitch<P0>(hwo: P0, pdwpitch: *mut u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutGetPitch(hwo : HWAVEOUT, pdwpitch : *mut u32) -> u32);
    waveOutGetPitch(hwo.param().abi(), pdwpitch)
}
#[inline]
pub unsafe fn waveOutGetPlaybackRate<P0>(hwo: P0, pdwrate: *mut u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutGetPlaybackRate(hwo : HWAVEOUT, pdwrate : *mut u32) -> u32);
    waveOutGetPlaybackRate(hwo.param().abi(), pdwrate)
}
#[inline]
pub unsafe fn waveOutGetPosition<P0>(hwo: P0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutGetPosition(hwo : HWAVEOUT, pmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
    waveOutGetPosition(hwo.param().abi(), pmmt, cbmmt)
}
#[inline]
pub unsafe fn waveOutGetVolume<P0>(hwo: P0, pdwvolume: *mut u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutGetVolume(hwo : HWAVEOUT, pdwvolume : *mut u32) -> u32);
    waveOutGetVolume(hwo.param().abi(), pdwvolume)
}
#[inline]
pub unsafe fn waveOutMessage<P0>(hwo: P0, umsg: u32, dw1: usize, dw2: usize) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutMessage(hwo : HWAVEOUT, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
    waveOutMessage(hwo.param().abi(), umsg, dw1, dw2)
}
#[inline]
pub unsafe fn waveOutOpen(phwo: Option<*mut HWAVEOUT>, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    windows_targets::link!("winmm.dll" "system" fn waveOutOpen(phwo : *mut HWAVEOUT, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
    waveOutOpen(core::mem::transmute(phwo.unwrap_or(std::ptr::null_mut())), udeviceid, pwfx, dwcallback, dwinstance, fdwopen)
}
#[inline]
pub unsafe fn waveOutPause<P0>(hwo: P0) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutPause(hwo : HWAVEOUT) -> u32);
    waveOutPause(hwo.param().abi())
}
#[inline]
pub unsafe fn waveOutPrepareHeader<P0>(hwo: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutPrepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    waveOutPrepareHeader(hwo.param().abi(), pwh, cbwh)
}
#[inline]
pub unsafe fn waveOutReset<P0>(hwo: P0) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutReset(hwo : HWAVEOUT) -> u32);
    waveOutReset(hwo.param().abi())
}
#[inline]
pub unsafe fn waveOutRestart<P0>(hwo: P0) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutRestart(hwo : HWAVEOUT) -> u32);
    waveOutRestart(hwo.param().abi())
}
#[inline]
pub unsafe fn waveOutSetPitch<P0>(hwo: P0, dwpitch: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutSetPitch(hwo : HWAVEOUT, dwpitch : u32) -> u32);
    waveOutSetPitch(hwo.param().abi(), dwpitch)
}
#[inline]
pub unsafe fn waveOutSetPlaybackRate<P0>(hwo: P0, dwrate: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutSetPlaybackRate(hwo : HWAVEOUT, dwrate : u32) -> u32);
    waveOutSetPlaybackRate(hwo.param().abi(), dwrate)
}
#[inline]
pub unsafe fn waveOutSetVolume<P0>(hwo: P0, dwvolume: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutSetVolume(hwo : HWAVEOUT, dwvolume : u32) -> u32);
    waveOutSetVolume(hwo.param().abi(), dwvolume)
}
#[inline]
pub unsafe fn waveOutUnprepareHeader<P0>(hwo: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutUnprepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    waveOutUnprepareHeader(hwo.param().abi(), pwh, cbwh)
}
#[inline]
pub unsafe fn waveOutWrite<P0>(hwo: P0, pwh: *mut WAVEHDR, cbwh: u32) -> u32
where
    P0: windows_core::Param<HWAVEOUT>,
{
    windows_targets::link!("winmm.dll" "system" fn waveOutWrite(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
    waveOutWrite(hwo.param().abi(), pwh, cbwh)
}
windows_core::imp::define_interface!(IAcousticEchoCancellationControl, IAcousticEchoCancellationControl_Vtbl, 0xf4ae25b5_aaa3_437d_b6b3_dbbe2d0e9549);
impl core::ops::Deref for IAcousticEchoCancellationControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAcousticEchoCancellationControl, windows_core::IUnknown);
impl IAcousticEchoCancellationControl {
    pub unsafe fn SetEchoCancellationRenderEndpoint<P0>(&self, endpointid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetEchoCancellationRenderEndpoint)(windows_core::Interface::as_raw(self), endpointid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAcousticEchoCancellationControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetEchoCancellationRenderEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivateAudioInterfaceAsyncOperation, IActivateAudioInterfaceAsyncOperation_Vtbl, 0x72a22d78_cde4_431d_b8cc_843a71199b6d);
impl core::ops::Deref for IActivateAudioInterfaceAsyncOperation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActivateAudioInterfaceAsyncOperation, windows_core::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    pub unsafe fn GetActivateResult(&self, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetActivateResult)(windows_core::Interface::as_raw(self), activateresult, core::mem::transmute(activatedinterface)).ok()
    }
}
#[repr(C)]
pub struct IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivateResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActivateAudioInterfaceCompletionHandler, IActivateAudioInterfaceCompletionHandler_Vtbl, 0x41d949ab_9862_444a_80f6_c261334da5eb);
impl core::ops::Deref for IActivateAudioInterfaceCompletionHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActivateAudioInterfaceCompletionHandler, windows_core::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    pub unsafe fn ActivateCompleted<P0>(&self, activateoperation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IActivateAudioInterfaceAsyncOperation>,
    {
        (windows_core::Interface::vtable(self).ActivateCompleted)(windows_core::Interface::as_raw(self), activateoperation.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioAmbisonicsControl, IAudioAmbisonicsControl_Vtbl, 0x28724c91_df35_4856_9f76_d6a26413f3df);
impl core::ops::Deref for IAudioAmbisonicsControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioAmbisonicsControl, windows_core::IUnknown);
impl IAudioAmbisonicsControl {
    pub unsafe fn SetData(&self, pambisonicsparams: &[AMBISONICS_PARAMS]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), core::mem::transmute(pambisonicsparams.as_ptr()), pambisonicsparams.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetHeadTracking<P0>(&self, benableheadtracking: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetHeadTracking)(windows_core::Interface::as_raw(self), benableheadtracking.param().abi()).ok()
    }
    pub unsafe fn GetHeadTracking(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHeadTracking)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), x, y, z, w).ok()
    }
}
#[repr(C)]
pub struct IAudioAmbisonicsControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const AMBISONICS_PARAMS, u32) -> windows_core::HRESULT,
    pub SetHeadTracking: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetHeadTracking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioAutoGainControl, IAudioAutoGainControl_Vtbl, 0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
impl core::ops::Deref for IAudioAutoGainControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioAutoGainControl, windows_core::IUnknown);
impl IAudioAutoGainControl {
    pub unsafe fn GetEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, benable: P0, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.param().abi(), core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IAudioAutoGainControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioBass, IAudioBass_Vtbl, 0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
impl core::ops::Deref for IAudioBass {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioBass, windows_core::IUnknown, IPerChannelDbLevel);
impl IAudioBass {}
#[repr(C)]
pub struct IAudioBass_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
windows_core::imp::define_interface!(IAudioCaptureClient, IAudioCaptureClient_Vtbl, 0xc8adbd64_e71e_48a0_a4de_185c395cd317);
impl core::ops::Deref for IAudioCaptureClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioCaptureClient, windows_core::IUnknown);
impl IAudioCaptureClient {
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: Option<*mut u64>, pu64qpcposition: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppdata, pnumframestoread, pdwflags, core::mem::transmute(pu64deviceposition.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pu64qpcposition.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframesread).ok()
    }
    pub unsafe fn GetNextPacketSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioCaptureClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNextPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioChannelConfig, IAudioChannelConfig_Vtbl, 0xbb11c46f_ec28_493c_b88a_5db88062ce98);
impl core::ops::Deref for IAudioChannelConfig {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioChannelConfig, windows_core::IUnknown);
impl IAudioChannelConfig {
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelConfig)(windows_core::Interface::as_raw(self), dwconfig, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetChannelConfig(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioChannelConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioClient, IAudioClient_Vtbl, 0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
impl core::ops::Deref for IAudioClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClient, windows_core::IUnknown);
impl IAudioClient {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, core::mem::transmute(audiosessionguid.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStreamLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: Option<*mut *mut WAVEFORMATEX>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsFormatSupported)(windows_core::Interface::as_raw(self), sharemode, pformat, core::mem::transmute(ppclosestmatch.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn GetMixFormat(&self) -> windows_core::Result<*mut WAVEFORMATEX> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMixFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: Option<*mut i64>, phnsminimumdeviceperiod: Option<*mut i64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDevicePeriod)(windows_core::Interface::as_raw(self), core::mem::transmute(phnsdefaultdeviceperiod.unwrap_or(std::ptr::null_mut())), core::mem::transmute(phnsminimumdeviceperiod.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), eventhandle.param().abi()).ok()
    }
    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAudioClient2, IAudioClient2_Vtbl, 0x726778cd_f60a_4eda_82de_e47610cd78aa);
impl core::ops::Deref for IAudioClient2 {
    type Target = IAudioClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClient2, windows_core::IUnknown, IAudioClient);
impl IAudioClient2 {
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOffloadCapable)(windows_core::Interface::as_raw(self), category, &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClientProperties)(windows_core::Interface::as_raw(self), pproperties).ok()
    }
    pub unsafe fn GetBufferSizeLimits<P0>(&self, pformat: *const WAVEFORMATEX, beventdriven: P0, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetBufferSizeLimits)(windows_core::Interface::as_raw(self), pformat, beventdriven.param().abi(), phnsminbufferduration, phnsmaxbufferduration).ok()
    }
}
#[repr(C)]
pub struct IAudioClient2_Vtbl {
    pub base__: IAudioClient_Vtbl,
    pub IsOffloadCapable: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_STREAM_CATEGORY, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetClientProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const AudioClientProperties) -> windows_core::HRESULT,
    pub GetBufferSizeLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX, super::super::Foundation::BOOL, *mut i64, *mut i64) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).GetSharedModeEnginePeriod)(windows_core::Interface::as_raw(self), pformat, pdefaultperiodinframes, pfundamentalperiodinframes, pminperiodinframes, pmaxperiodinframes).ok()
    }
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentSharedModeEnginePeriod)(windows_core::Interface::as_raw(self), ppformat, pcurrentperiodinframes).ok()
    }
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializeSharedAudioStream)(windows_core::Interface::as_raw(self), streamflags, periodinframes, pformat, core::mem::transmute(audiosessionguid.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IAudioClient3_Vtbl {
    pub base__: IAudioClient2_Vtbl,
    pub GetSharedModeEnginePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentSharedModeEnginePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    pub InitializeSharedAudioStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const WAVEFORMATEX, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioClientDuckingControl, IAudioClientDuckingControl_Vtbl, 0xc789d381_a28c_4168_b28f_d3a837924dc3);
impl core::ops::Deref for IAudioClientDuckingControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClientDuckingControl, windows_core::IUnknown);
impl IAudioClientDuckingControl {
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDuckingOptionsForCurrentStream)(windows_core::Interface::as_raw(self), options).ok()
    }
}
#[repr(C)]
pub struct IAudioClientDuckingControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDuckingOptionsForCurrentStream: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioClock, IAudioClock_Vtbl, 0xcd63314f_3fba_4a1b_812c_ef96358728e7);
impl core::ops::Deref for IAudioClock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClock, windows_core::IUnknown);
impl IAudioClock {
    pub unsafe fn GetFrequency(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), pu64position, core::mem::transmute(pu64qpcposition.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioClock2, IAudioClock2_Vtbl, 0x6f49ff73_6727_49ac_a008_d98cf5e70048);
impl core::ops::Deref for IAudioClock2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClock2, windows_core::IUnknown);
impl IAudioClock2 {
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDevicePosition)(windows_core::Interface::as_raw(self), deviceposition, core::mem::transmute(qpcposition.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IAudioClock2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevicePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioClockAdjustment, IAudioClockAdjustment_Vtbl, 0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
impl core::ops::Deref for IAudioClockAdjustment {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClockAdjustment, windows_core::IUnknown);
impl IAudioClockAdjustment {
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSampleRate)(windows_core::Interface::as_raw(self), flsamplerate).ok()
    }
}
#[repr(C)]
pub struct IAudioClockAdjustment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSampleRate: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEffectsChangedNotificationClient, IAudioEffectsChangedNotificationClient_Vtbl, 0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
impl core::ops::Deref for IAudioEffectsChangedNotificationClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEffectsChangedNotificationClient, windows_core::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    pub unsafe fn OnAudioEffectsChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAudioEffectsChanged)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAudioEffectsChangedNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAudioEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEffectsManager, IAudioEffectsManager_Vtbl, 0x4460b3ae_4b44_4527_8676_7548a8acd260);
impl core::ops::Deref for IAudioEffectsManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEffectsManager, windows_core::IUnknown);
impl IAudioEffectsManager {
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEffectsChangedNotificationClient>,
    {
        (windows_core::Interface::vtable(self).RegisterAudioEffectsChangedNotificationCallback)(windows_core::Interface::as_raw(self), client.param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioEffectsChangedNotificationClient>,
    {
        (windows_core::Interface::vtable(self).UnregisterAudioEffectsChangedNotificationCallback)(windows_core::Interface::as_raw(self), client.param().abi()).ok()
    }
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAudioEffects)(windows_core::Interface::as_raw(self), effects, numeffects).ok()
    }
    pub unsafe fn SetAudioEffectState(&self, effectid: windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAudioEffectState)(windows_core::Interface::as_raw(self), core::mem::transmute(effectid), state).ok()
    }
}
#[repr(C)]
pub struct IAudioEffectsManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAudioEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut AUDIO_EFFECT, *mut u32) -> windows_core::HRESULT,
    pub SetAudioEffectState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, AUDIO_EFFECT_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioFormatEnumerator, IAudioFormatEnumerator_Vtbl, 0xdcdaa858_895a_4a22_a5eb_67bda506096d);
impl core::ops::Deref for IAudioFormatEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioFormatEnumerator, windows_core::IUnknown);
impl IAudioFormatEnumerator {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFormat(&self, index: u32) -> windows_core::Result<*mut WAVEFORMATEX> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioFormatEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut WAVEFORMATEX) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioInputSelector, IAudioInputSelector_Vtbl, 0x4f03dc02_5e6e_4653_8f72_a030c123d598);
impl core::ops::Deref for IAudioInputSelector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioInputSelector, windows_core::IUnknown);
impl IAudioInputSelector {
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IAudioInputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioLoudness, IAudioLoudness_Vtbl, 0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
impl core::ops::Deref for IAudioLoudness {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioLoudness, windows_core::IUnknown);
impl IAudioLoudness {
    pub unsafe fn GetEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, benable: P0, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.param().abi(), core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IAudioLoudness_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioMidrange, IAudioMidrange_Vtbl, 0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
impl core::ops::Deref for IAudioMidrange {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioMidrange, windows_core::IUnknown, IPerChannelDbLevel);
impl IAudioMidrange {}
#[repr(C)]
pub struct IAudioMidrange_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
windows_core::imp::define_interface!(IAudioMute, IAudioMute_Vtbl, 0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
impl core::ops::Deref for IAudioMute {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioMute, windows_core::IUnknown);
impl IAudioMute {
    pub unsafe fn SetMute<P0>(&self, bmuted: P0, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmuted.param().abi(), core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioMute_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioOutputSelector, IAudioOutputSelector_Vtbl, 0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
impl core::ops::Deref for IAudioOutputSelector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioOutputSelector, windows_core::IUnknown);
impl IAudioOutputSelector {
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IAudioOutputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioPeakMeter, IAudioPeakMeter_Vtbl, 0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
impl core::ops::Deref for IAudioPeakMeter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioPeakMeter, windows_core::IUnknown);
impl IAudioPeakMeter {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAudioPeakMeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioRenderClient, IAudioRenderClient_Vtbl, 0xf294acfc_3146_4483_a7bf_addca7c260e2);
impl core::ops::Deref for IAudioRenderClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioRenderClient, windows_core::IUnknown);
impl IAudioRenderClient {
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> windows_core::Result<*mut u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), numframesrequested, &mut result__).map(|| result__)
    }
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframeswritten, dwflags).ok()
    }
}
#[repr(C)]
pub struct IAudioRenderClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSessionControl, IAudioSessionControl_Vtbl, 0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
impl core::ops::Deref for IAudioSessionControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionControl, windows_core::IUnknown);
impl IAudioSessionControl {
    pub unsafe fn GetState(&self) -> windows_core::Result<AudioSessionState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, value: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), value.param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIconPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIconPath<P0>(&self, value: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetIconPath)(windows_core::Interface::as_raw(self), value.param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGroupingParam)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGroupingParam)(windows_core::Interface::as_raw(self), r#override, eventcontext).ok()
    }
    pub unsafe fn RegisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionEvents>,
    {
        (windows_core::Interface::vtable(self).RegisterAudioSessionNotification)(windows_core::Interface::as_raw(self), newnotifications.param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionEvents>,
    {
        (windows_core::Interface::vtable(self).UnregisterAudioSessionNotification)(windows_core::Interface::as_raw(self), newnotifications.param().abi()).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSessionIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSessionInstanceIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProcessId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSystemSoundsSession(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsSystemSoundsSession)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetDuckingPreference<P0>(&self, optout: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDuckingPreference)(windows_core::Interface::as_raw(self), optout.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioSessionControl2_Vtbl {
    pub base__: IAudioSessionControl_Vtbl,
    pub GetSessionIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSessionInstanceIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsSystemSoundsSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuckingPreference: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSessionEnumerator, IAudioSessionEnumerator_Vtbl, 0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
impl core::ops::Deref for IAudioSessionEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionEnumerator, windows_core::IUnknown);
impl IAudioSessionEnumerator {
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSession(&self, sessioncount: i32) -> windows_core::Result<IAudioSessionControl> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSession)(windows_core::Interface::as_raw(self), sessioncount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAudioSessionEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetSession: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSessionEvents, IAudioSessionEvents_Vtbl, 0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
impl core::ops::Deref for IAudioSessionEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionEvents, windows_core::IUnknown);
impl IAudioSessionEvents {
    pub unsafe fn OnDisplayNameChanged<P0>(&self, newdisplayname: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnDisplayNameChanged)(windows_core::Interface::as_raw(self), newdisplayname.param().abi(), eventcontext).ok()
    }
    pub unsafe fn OnIconPathChanged<P0>(&self, newiconpath: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnIconPathChanged)(windows_core::Interface::as_raw(self), newiconpath.param().abi(), eventcontext).ok()
    }
    pub unsafe fn OnSimpleVolumeChanged<P0>(&self, newvolume: f32, newmute: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnSimpleVolumeChanged)(windows_core::Interface::as_raw(self), newvolume, newmute.param().abi(), eventcontext).ok()
    }
    pub unsafe fn OnChannelVolumeChanged(&self, newchannelvolumearray: &[f32], changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChannelVolumeChanged)(windows_core::Interface::as_raw(self), newchannelvolumearray.len().try_into().unwrap(), core::mem::transmute(newchannelvolumearray.as_ptr()), changedchannel, eventcontext).ok()
    }
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnGroupingParamChanged)(windows_core::Interface::as_raw(self), newgroupingparam, eventcontext).ok()
    }
    pub unsafe fn OnStateChanged(&self, newstate: AudioSessionState) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSessionDisconnected)(windows_core::Interface::as_raw(self), disconnectreason).ok()
    }
}
#[repr(C)]
pub struct IAudioSessionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDisplayNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnIconPathChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnSimpleVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, f32, super::super::Foundation::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnChannelVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnGroupingParamChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, AudioSessionState) -> windows_core::HRESULT,
    pub OnSessionDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, AudioSessionDisconnectReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSessionManager, IAudioSessionManager_Vtbl, 0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
impl core::ops::Deref for IAudioSessionManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionManager, windows_core::IUnknown);
impl IAudioSessionManager {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: Option<*const windows_core::GUID>, streamflags: u32) -> windows_core::Result<IAudioSessionControl> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudioSessionControl)(windows_core::Interface::as_raw(self), core::mem::transmute(audiosessionguid.unwrap_or(std::ptr::null())), streamflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: Option<*const windows_core::GUID>, streamflags: u32) -> windows_core::Result<ISimpleAudioVolume> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSimpleAudioVolume)(windows_core::Interface::as_raw(self), core::mem::transmute(audiosessionguid.unwrap_or(std::ptr::null())), streamflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IAudioSessionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAudioSessionControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSimpleAudioVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSessionEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterSessionNotification<P0>(&self, sessionnotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionNotification>,
    {
        (windows_core::Interface::vtable(self).RegisterSessionNotification)(windows_core::Interface::as_raw(self), sessionnotification.param().abi()).ok()
    }
    pub unsafe fn UnregisterSessionNotification<P0>(&self, sessionnotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionNotification>,
    {
        (windows_core::Interface::vtable(self).UnregisterSessionNotification)(windows_core::Interface::as_raw(self), sessionnotification.param().abi()).ok()
    }
    pub unsafe fn RegisterDuckNotification<P0, P1>(&self, sessionid: P0, ducknotification: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IAudioVolumeDuckNotification>,
    {
        (windows_core::Interface::vtable(self).RegisterDuckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi(), ducknotification.param().abi()).ok()
    }
    pub unsafe fn UnregisterDuckNotification<P0>(&self, ducknotification: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioVolumeDuckNotification>,
    {
        (windows_core::Interface::vtable(self).UnregisterDuckNotification)(windows_core::Interface::as_raw(self), ducknotification.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioSessionManager2_Vtbl {
    pub base__: IAudioSessionManager_Vtbl,
    pub GetSessionEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSessionNotification, IAudioSessionNotification_Vtbl, 0x641dd20b_4d41_49cc_aba3_174b9477bb08);
impl core::ops::Deref for IAudioSessionNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionNotification, windows_core::IUnknown);
impl IAudioSessionNotification {
    pub unsafe fn OnSessionCreated<P0>(&self, newsession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSessionControl>,
    {
        (windows_core::Interface::vtable(self).OnSessionCreated)(windows_core::Interface::as_raw(self), newsession.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioSessionNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSessionCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioStateMonitor, IAudioStateMonitor_Vtbl, 0x63bd8738_e30d_4c77_bf5c_834e87c657e2);
impl core::ops::Deref for IAudioStateMonitor {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioStateMonitor, windows_core::IUnknown);
impl IAudioStateMonitor {
    pub unsafe fn RegisterCallback(&self, callback: PAudioStateMonitorCallback, context: Option<*const core::ffi::c_void>) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterCallback)(windows_core::Interface::as_raw(self), callback, core::mem::transmute(context.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
    pub unsafe fn UnregisterCallback(&self, registration: i64) {
        (windows_core::Interface::vtable(self).UnregisterCallback)(windows_core::Interface::as_raw(self), registration)
    }
    pub unsafe fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        (windows_core::Interface::vtable(self).GetSoundLevel)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IAudioStateMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, PAudioStateMonitorCallback, *const core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub UnregisterCallback: unsafe extern "system" fn(*mut core::ffi::c_void, i64),
    pub GetSoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> AudioStateMonitorSoundLevel,
}
windows_core::imp::define_interface!(IAudioStreamVolume, IAudioStreamVolume_Vtbl, 0x93014887_242d_4068_8a15_cf5e93b90fe3);
impl core::ops::Deref for IAudioStreamVolume {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioStreamVolume, windows_core::IUnknown);
impl IAudioStreamVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel).ok()
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())).ok()
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IAudioStreamVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSystemEffectsPropertyChangeNotificationClient, IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl, 0x20049d40_56d5_400e_a2ef_385599feed49);
impl core::ops::Deref for IAudioSystemEffectsPropertyChangeNotificationClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsPropertyChangeNotificationClient, windows_core::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnPropertyChanged)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(key)).ok()
    }
}
#[repr(C)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OnPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OnPropertyChanged: usize,
}
windows_core::imp::define_interface!(IAudioSystemEffectsPropertyStore, IAudioSystemEffectsPropertyStore_Vtbl, 0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
impl core::ops::Deref for IAudioSystemEffectsPropertyStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsPropertyStore, windows_core::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenDefaultPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenUserPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenVolatilePropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ResetUserPropertyStore(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetUserPropertyStore)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResetVolatilePropertyStore(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetVolatilePropertyStore)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RegisterPropertyChangeNotification<P0>(&self, callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        (windows_core::Interface::vtable(self).RegisterPropertyChangeNotification)(windows_core::Interface::as_raw(self), callback.param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertyChangeNotification<P0>(&self, callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        (windows_core::Interface::vtable(self).UnregisterPropertyChangeNotification)(windows_core::Interface::as_raw(self), callback.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAudioTreble, IAudioTreble_Vtbl, 0x0a717812_694e_4907_b74b_bafa5cfdca7b);
impl core::ops::Deref for IAudioTreble {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioTreble, windows_core::IUnknown, IPerChannelDbLevel);
impl IAudioTreble {}
#[repr(C)]
pub struct IAudioTreble_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
windows_core::imp::define_interface!(IAudioViewManagerService, IAudioViewManagerService_Vtbl, 0xa7a7ef10_1f49_45e0_ad35_612057cc8f74);
impl core::ops::Deref for IAudioViewManagerService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioViewManagerService, windows_core::IUnknown);
impl IAudioViewManagerService {
    pub unsafe fn SetAudioStreamWindow<P0>(&self, hwnd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetAudioStreamWindow)(windows_core::Interface::as_raw(self), hwnd.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioViewManagerService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAudioStreamWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioVolumeDuckNotification, IAudioVolumeDuckNotification_Vtbl, 0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
impl core::ops::Deref for IAudioVolumeDuckNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioVolumeDuckNotification, windows_core::IUnknown);
impl IAudioVolumeDuckNotification {
    pub unsafe fn OnVolumeDuckNotification<P0>(&self, sessionid: P0, countcommunicationsessions: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnVolumeDuckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi(), countcommunicationsessions).ok()
    }
    pub unsafe fn OnVolumeUnduckNotification<P0>(&self, sessionid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnVolumeUnduckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioVolumeDuckNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnVolumeDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnVolumeUnduckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioVolumeLevel, IAudioVolumeLevel_Vtbl, 0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
impl core::ops::Deref for IAudioVolumeLevel {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioVolumeLevel, windows_core::IUnknown, IPerChannelDbLevel);
impl IAudioVolumeLevel {}
#[repr(C)]
pub struct IAudioVolumeLevel_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
windows_core::imp::define_interface!(IChannelAudioVolume, IChannelAudioVolume_Vtbl, 0x1c158861_b533_4b30_b1cf_e853e51c59b8);
impl core::ops::Deref for IChannelAudioVolume {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChannelAudioVolume, windows_core::IUnknown);
impl IChannelAudioVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel, eventcontext).ok()
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32], eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr()), eventcontext).ok()
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IChannelAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnector, IConnector_Vtbl, 0x9c2c4058_23f5_41de_877a_df3af236a09e);
impl core::ops::Deref for IConnector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConnector, windows_core::IUnknown);
impl IConnector {
    pub unsafe fn GetType(&self) -> windows_core::Result<ConnectorType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDataFlow(&self) -> windows_core::Result<DataFlow> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ConnectTo<P0>(&self, pconnectto: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IConnector>,
    {
        (windows_core::Interface::vtable(self).ConnectTo)(windows_core::Interface::as_raw(self), pconnectto.param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectedTo(&self) -> windows_core::Result<IConnector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectorIdConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceIdConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IConnector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConnectorType) -> windows_core::HRESULT,
    pub GetDataFlow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataFlow) -> windows_core::HRESULT,
    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnectorIdConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDeviceIdConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IControlChangeNotify, IControlChangeNotify_Vtbl, 0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
impl core::ops::Deref for IControlChangeNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IControlChangeNotify, windows_core::IUnknown);
impl IControlChangeNotify {
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), dwsenderprocessid, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IControlChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IControlInterface, IControlInterface_Vtbl, 0x45d37c3f_5140_444a_ae24_400789f3cbf3);
impl core::ops::Deref for IControlInterface {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IControlInterface, windows_core::IUnknown);
impl IControlInterface {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IControlInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceSpecificProperty, IDeviceSpecificProperty_Vtbl, 0x3b22bcbf_2586_4af0_8583_205d391b807c);
impl core::ops::Deref for IDeviceSpecificProperty {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDeviceSpecificProperty, windows_core::IUnknown);
impl IDeviceSpecificProperty {
    pub unsafe fn GetType(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pvvalue, pcbvalue).ok()
    }
    pub unsafe fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), pvvalue, cbvalue, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Get4BRange)(windows_core::Interface::as_raw(self), plmin, plmax, plstepping).ok()
    }
}
#[repr(C)]
pub struct IDeviceSpecificProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Get4BRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeviceTopology, IDeviceTopology_Vtbl, 0x2a07407e_6497_4a18_9787_32f79bd0d98f);
impl core::ops::Deref for IDeviceTopology {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDeviceTopology, windows_core::IUnknown);
impl IDeviceTopology {
    pub unsafe fn GetConnectorCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnector)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSubunitCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubunitCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSubunit(&self, nindex: u32) -> windows_core::Result<ISubunit> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubunit)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPartById(&self, nid: u32) -> windows_core::Result<IPart> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartById)(windows_core::Interface::as_raw(self), nid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSignalPath<P0, P1, P2>(&self, pipartfrom: P0, pipartto: P1, brejectmixedpaths: P2) -> windows_core::Result<IPartsList>
    where
        P0: windows_core::Param<IPart>,
        P1: windows_core::Param<IPart>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSignalPath)(windows_core::Interface::as_raw(self), pipartfrom.param().abi(), pipartto.param().abi(), brejectmixedpaths.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDeviceTopology_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubunitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSubunit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMDevice, IMMDevice_Vtbl, 0xd666063f_1587_4e43_81f1_b948e807363f);
impl core::ops::Deref for IMMDevice {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMDevice, windows_core::IUnknown);
impl IMMDevice {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Activate<T>(&self, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: Option<*const windows_core::PROPVARIANT>) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &T::IID, dwclsctx, core::mem::transmute(pactivationparams.unwrap_or(std::ptr::null())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::STGM) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<DEVICE_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMMDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::super::System::Com::CLSCTX, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Activate: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Com::STGM, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DEVICE_STATE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMDeviceActivator, IMMDeviceActivator_Vtbl, 0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
impl core::ops::Deref for IMMDeviceActivator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMDeviceActivator, windows_core::IUnknown);
impl IMMDeviceActivator {
    pub unsafe fn Activate<P0>(&self, iid: *const windows_core::GUID, pdevice: P0, pactivationparams: Option<*const windows_core::PROPVARIANT>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMMDevice>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), iid, pdevice.param().abi(), core::mem::transmute(pactivationparams.unwrap_or(std::ptr::null())), ppinterface).ok()
    }
}
#[repr(C)]
pub struct IMMDeviceActivator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMDeviceCollection, IMMDeviceCollection_Vtbl, 0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
impl core::ops::Deref for IMMDeviceCollection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMDeviceCollection, windows_core::IUnknown);
impl IMMDeviceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), ndevice, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMMDeviceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMDeviceEnumerator, IMMDeviceEnumerator_Vtbl, 0xa95664d2_9614_4f35_a746_de8db63617e6);
impl core::ops::Deref for IMMDeviceEnumerator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMDeviceEnumerator, windows_core::IUnknown);
impl IMMDeviceEnumerator {
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: DEVICE_STATE) -> windows_core::Result<IMMDeviceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumAudioEndpoints)(windows_core::Interface::as_raw(self), dataflow, dwstatemask, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultAudioEndpoint)(windows_core::Interface::as_raw(self), dataflow, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDevice<P0>(&self, pwstrid: P0) -> windows_core::Result<IMMDevice>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), pwstrid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMMNotificationClient>,
    {
        (windows_core::Interface::vtable(self).RegisterEndpointNotificationCallback)(windows_core::Interface::as_raw(self), pclient.param().abi()).ok()
    }
    pub unsafe fn UnregisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMMNotificationClient>,
    {
        (windows_core::Interface::vtable(self).UnregisterEndpointNotificationCallback)(windows_core::Interface::as_raw(self), pclient.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMMDeviceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumAudioEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, DEVICE_STATE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEndpointNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterEndpointNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMEndpoint, IMMEndpoint_Vtbl, 0x1be09788_6894_4089_8586_9a2a6c265ac5);
impl core::ops::Deref for IMMEndpoint {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMEndpoint, windows_core::IUnknown);
impl IMMEndpoint {
    pub unsafe fn GetDataFlow(&self) -> windows_core::Result<EDataFlow> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMMEndpoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataFlow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EDataFlow) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMMNotificationClient, IMMNotificationClient_Vtbl, 0x7991eec9_7e89_4d85_8390_6c703cec60c0);
impl core::ops::Deref for IMMNotificationClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMMNotificationClient, windows_core::IUnknown);
impl IMMNotificationClient {
    pub unsafe fn OnDeviceStateChanged<P0>(&self, pwstrdeviceid: P0, dwnewstate: DEVICE_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnDeviceStateChanged)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi(), dwnewstate).ok()
    }
    pub unsafe fn OnDeviceAdded<P0>(&self, pwstrdeviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnDeviceAdded)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi()).ok()
    }
    pub unsafe fn OnDeviceRemoved<P0>(&self, pwstrdeviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnDeviceRemoved)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi()).ok()
    }
    pub unsafe fn OnDefaultDeviceChanged<P0>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnDefaultDeviceChanged)(windows_core::Interface::as_raw(self), flow, role, pwstrdefaultdeviceid.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyValueChanged<P0>(&self, pwstrdeviceid: P0, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnPropertyValueChanged)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi(), core::mem::transmute(key)).ok()
    }
}
#[repr(C)]
pub struct IMMNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDeviceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DEVICE_STATE) -> windows_core::HRESULT,
    pub OnDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDefaultDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OnPropertyValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OnPropertyValueChanged: usize,
}
windows_core::imp::define_interface!(IMessageFilter, IMessageFilter_Vtbl, 0x00000016_0000_0000_c000_000000000046);
impl core::ops::Deref for IMessageFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMessageFilter, windows_core::IUnknown);
impl IMessageFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleInComingCall<P0>(&self, dwcalltype: u32, htaskcaller: P0, dwtickcount: u32, lpinterfaceinfo: Option<*const super::super::System::Com::INTERFACEINFO>) -> u32
    where
        P0: windows_core::Param<super::HTASK>,
    {
        (windows_core::Interface::vtable(self).HandleInComingCall)(windows_core::Interface::as_raw(self), dwcalltype, htaskcaller.param().abi(), dwtickcount, core::mem::transmute(lpinterfaceinfo.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn RetryRejectedCall<P0>(&self, htaskcallee: P0, dwtickcount: u32, dwrejecttype: u32) -> u32
    where
        P0: windows_core::Param<super::HTASK>,
    {
        (windows_core::Interface::vtable(self).RetryRejectedCall)(windows_core::Interface::as_raw(self), htaskcallee.param().abi(), dwtickcount, dwrejecttype)
    }
    pub unsafe fn MessagePending<P0>(&self, htaskcallee: P0, dwtickcount: u32, dwpendingtype: u32) -> u32
    where
        P0: windows_core::Param<super::HTASK>,
    {
        (windows_core::Interface::vtable(self).MessagePending)(windows_core::Interface::as_raw(self), htaskcallee.param().abi(), dwtickcount, dwpendingtype)
    }
}
#[repr(C)]
pub struct IMessageFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleInComingCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::HTASK, u32, *const super::super::System::Com::INTERFACEINFO) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleInComingCall: usize,
    pub RetryRejectedCall: unsafe extern "system" fn(*mut core::ffi::c_void, super::HTASK, u32, u32) -> u32,
    pub MessagePending: unsafe extern "system" fn(*mut core::ffi::c_void, super::HTASK, u32, u32) -> u32,
}
windows_core::imp::define_interface!(IPart, IPart_Vtbl, 0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
impl core::ops::Deref for IPart {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPart, windows_core::IUnknown);
impl IPart {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLocalId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetGlobalId(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGlobalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPartType(&self) -> windows_core::Result<PartType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPartType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSubType(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetControlInterfaceCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetControlInterfaceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> windows_core::Result<IControlInterface> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetControlInterface)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumPartsIncoming(&self) -> windows_core::Result<IPartsList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumPartsIncoming)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumPartsOutgoing(&self) -> windows_core::Result<IPartsList> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumPartsOutgoing)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTopologyObject(&self) -> windows_core::Result<IDeviceTopology> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTopologyObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), dwclscontext, refiid, core::mem::transmute(ppvobject.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn RegisterControlChangeCallback<P0>(&self, riid: *const windows_core::GUID, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IControlChangeNotify>,
    {
        (windows_core::Interface::vtable(self).RegisterControlChangeCallback)(windows_core::Interface::as_raw(self), riid, pnotify.param().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeCallback<P0>(&self, pnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IControlChangeNotify>,
    {
        (windows_core::Interface::vtable(self).UnregisterControlChangeCallback)(windows_core::Interface::as_raw(self), pnotify.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IPartsList, IPartsList_Vtbl, 0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
impl core::ops::Deref for IPartsList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPartsList, windows_core::IUnknown);
impl IPartsList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPart(&self, nindex: u32) -> windows_core::Result<IPart> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPart)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPartsList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPerChannelDbLevel, IPerChannelDbLevel_Vtbl, 0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
impl core::ops::Deref for IPerChannelDbLevel {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPerChannelDbLevel, windows_core::IUnknown);
impl IPerChannelDbLevel {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLevelRange)(windows_core::Interface::as_raw(self), nchannel, pfminleveldb, pfmaxleveldb, pfstepping).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLevelUniform)(windows_core::Interface::as_raw(self), fleveldb, core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLevelAllChannels)(windows_core::Interface::as_raw(self), core::mem::transmute(alevelsdb.as_ptr()), alevelsdb.len().try_into().unwrap(), core::mem::transmute(pguideventcontext.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IPerChannelDbLevel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLevelRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetLevelUniform: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetLevelAllChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleAudioVolume, ISimpleAudioVolume_Vtbl, 0x87ce5498_68d6_44e5_9215_6da47ef883d8);
impl core::ops::Deref for ISimpleAudioVolume {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimpleAudioVolume, windows_core::IUnknown);
impl ISimpleAudioVolume {
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMasterVolume)(windows_core::Interface::as_raw(self), flevel, eventcontext).ok()
    }
    pub unsafe fn GetMasterVolume(&self) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMasterVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMute<P0>(&self, bmute: P0, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.param().abi(), eventcontext).ok()
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISimpleAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioClient, ISpatialAudioClient_Vtbl, 0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
impl core::ops::Deref for ISpatialAudioClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioClient, windows_core::IUnknown);
impl ISpatialAudioClient {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStaticObjectPosition)(windows_core::Interface::as_raw(self), r#type, x, y, z).ok()
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> windows_core::Result<AudioObjectType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNativeStaticObjectTypeMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxDynamicObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> windows_core::Result<IAudioFormatEnumerator> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedAudioObjectFormatEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxFrameCount)(windows_core::Interface::as_raw(self), objectformat, &mut result__).map(|| result__)
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsAudioObjectFormatSupported)(windows_core::Interface::as_raw(self), objectformat).ok()
    }
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const windows_core::GUID, auxiliaryinfo: Option<*const windows_core::PROPVARIANT>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsSpatialAudioStreamAvailable)(windows_core::Interface::as_raw(self), streamuuid, core::mem::transmute(auxiliaryinfo.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn ActivateSpatialAudioStream<T>(&self, activationparams: *const windows_core::PROPVARIANT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioStream)(windows_core::Interface::as_raw(self), core::mem::transmute(activationparams), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialAudioClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStaticObjectPosition: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetNativeStaticObjectTypeMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioObjectType) -> windows_core::HRESULT,
    pub GetMaxDynamicObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedAudioObjectFormatEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    pub IsAudioObjectFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const WAVEFORMATEX) -> windows_core::HRESULT,
    pub IsSpatialAudioStreamAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT,
    pub ActivateSpatialAudioStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioClient2, ISpatialAudioClient2_Vtbl, 0xcaabe452_a66a_4bee_a93e_e320463f6a53);
impl core::ops::Deref for ISpatialAudioClient2 {
    type Target = ISpatialAudioClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioClient2, windows_core::IUnknown, ISpatialAudioClient);
impl ISpatialAudioClient2 {
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsOffloadCapable)(windows_core::Interface::as_raw(self), category, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxFrameCountForCategory<P0>(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: P0, objectformat: *const WAVEFORMATEX) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxFrameCountForCategory)(windows_core::Interface::as_raw(self), category, offloadenabled.param().abi(), objectformat, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpatialAudioClient2_Vtbl {
    pub base__: ISpatialAudioClient_Vtbl,
    pub IsOffloadCapable: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_STREAM_CATEGORY, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetMaxFrameCountForCategory: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_STREAM_CATEGORY, super::super::Foundation::BOOL, *const WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioMetadataClient, ISpatialAudioMetadataClient_Vtbl, 0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
impl core::ops::Deref for ISpatialAudioMetadataClient {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataClient, windows_core::IUnknown);
impl ISpatialAudioMetadataClient {
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: Option<*mut Option<ISpatialAudioMetadataItemsBuffer>>, metadataitems: *mut Option<ISpatialAudioMetadataItems>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataItems)(windows_core::Interface::as_raw(self), maxitemcount, framecount, core::mem::transmute(metadataitemsbuffer.unwrap_or(std::ptr::null_mut())), core::mem::transmute(metadataitems)).ok()
    }
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpatialAudioMetadataItemsBufferLength)(windows_core::Interface::as_raw(self), maxitemcount, &mut result__).map(|| result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> windows_core::Result<ISpatialAudioMetadataWriter> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataWriter)(windows_core::Interface::as_raw(self), overflowmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> windows_core::Result<ISpatialAudioMetadataCopier> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataCopier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> windows_core::Result<ISpatialAudioMetadataReader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioMetadataReader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialAudioMetadataClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateSpatialAudioMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpatialAudioMetadataItemsBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut u32) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialAudioMetadataWriterOverflowMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataCopier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioMetadataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioMetadataCopier, ISpatialAudioMetadataCopier_Vtbl, 0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
impl core::ops::Deref for ISpatialAudioMetadataCopier {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataCopier, windows_core::IUnknown);
impl ISpatialAudioMetadataCopier {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()).ok()
    }
    pub unsafe fn CopyMetadataForFrames<P0>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: P0) -> windows_core::Result<u16>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyMetadataForFrames)(windows_core::Interface::as_raw(self), copyframecount, copymode, dstmetadataitems.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioMetadataCopier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyMetadataForFrames: unsafe extern "system" fn(*mut core::ffi::c_void, u16, SpatialAudioMetadataCopyMode, *mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioMetadataItems, ISpatialAudioMetadataItems_Vtbl, 0xbcd7c78f_3098_4f22_b547_a2f25a381269);
impl core::ops::Deref for ISpatialAudioMetadataItems {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataItems, windows_core::IUnknown);
impl ISpatialAudioMetadataItems {
    pub unsafe fn GetFrameCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetItemCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxItemCount(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMaxValueBufferLength(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMaxValueBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInfo(&self) -> windows_core::Result<SpatialAudioMetadataItemsInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpatialAudioMetadataItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetMaxValueBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SpatialAudioMetadataItemsInfo) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioMetadataItemsBuffer, ISpatialAudioMetadataItemsBuffer_Vtbl, 0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
impl core::ops::Deref for ISpatialAudioMetadataItemsBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataItemsBuffer, windows_core::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    pub unsafe fn AttachToBuffer(&self, buffer: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachToBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: &mut [u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AttachToPopulatedBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn DetachBuffer(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AttachToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub AttachToPopulatedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioMetadataReader, ISpatialAudioMetadataReader_Vtbl, 0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
impl core::ops::Deref for ISpatialAudioMetadataReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataReader, windows_core::IUnknown);
impl ISpatialAudioMetadataReader {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()).ok()
    }
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadNextItem)(windows_core::Interface::as_raw(self), commandcount, frameoffset).ok()
    }
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReadNextItemCommand)(windows_core::Interface::as_raw(self), commandid, valuebuffer, maxvaluebufferlength, valuebufferlength).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioMetadataReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u16) -> windows_core::HRESULT,
    pub ReadNextItemCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioMetadataWriter, ISpatialAudioMetadataWriter_Vtbl, 0x1b17ca01_2955_444d_a430_537dc589a844);
impl core::ops::Deref for ISpatialAudioMetadataWriter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioMetadataWriter, windows_core::IUnknown);
impl ISpatialAudioMetadataWriter {
    pub unsafe fn Open<P0>(&self, metadataitems: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioMetadataItems>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), metadataitems.param().abi()).ok()
    }
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteNextItem)(windows_core::Interface::as_raw(self), frameoffset).ok()
    }
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: Option<*const core::ffi::c_void>, valuebufferlength: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteNextItemCommand)(windows_core::Interface::as_raw(self), commandid, core::mem::transmute(valuebuffer.unwrap_or(std::ptr::null())), valuebufferlength).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioMetadataWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteNextItem: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteNextItemCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z).ok()
    }
    pub unsafe fn SetVolume(&self, volume: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioObject_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioObjectBase, ISpatialAudioObjectBase_Vtbl, 0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
impl core::ops::Deref for ISpatialAudioObjectBase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectBase, windows_core::IUnknown);
impl ISpatialAudioObjectBase {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer, bufferlength).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEndOfStream)(windows_core::Interface::as_raw(self), framecount).ok()
    }
    pub unsafe fn IsActive(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> windows_core::Result<AudioObjectType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudioObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAudioObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioObjectType) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z).ok()
    }
    pub unsafe fn SetGain(&self, gain: f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGain)(windows_core::Interface::as_raw(self), gain).ok()
    }
    pub unsafe fn SetOrientation(&self, orientation: *const *const f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOrientation)(windows_core::Interface::as_raw(self), orientation).ok()
    }
    pub unsafe fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnvironment)(windows_core::Interface::as_raw(self), environment).ok()
    }
    pub unsafe fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDistanceDecay)(windows_core::Interface::as_raw(self), distancedecay).ok()
    }
    pub unsafe fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDirectivity)(windows_core::Interface::as_raw(self), directivity).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectForHrtf_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *const *const f32) -> windows_core::HRESULT,
    pub SetEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, SpatialAudioHrtfEnvironmentType) -> windows_core::HRESULT,
    pub SetDistanceDecay: unsafe extern "system" fn(*mut core::ffi::c_void, *const SpatialAudioHrtfDistanceDecay) -> windows_core::HRESULT,
    pub SetDirectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *const SpatialAudioHrtfDirectivityUnion) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).WriteNextMetadataCommand)(windows_core::Interface::as_raw(self), commandid, core::mem::transmute(valuebuffer.unwrap_or(std::ptr::null())), valuebufferlength).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub WriteNextMetadataCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSpatialAudioMetadataItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectForMetadataItems_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub GetSpatialAudioMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioObject)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectRenderStream_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObject: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamBase, ISpatialAudioObjectRenderStreamBase_Vtbl, 0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
impl core::ops::Deref for ISpatialAudioObjectRenderStreamBase {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamBase, windows_core::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAvailableDynamicObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginUpdatingAudioObjects)(windows_core::Interface::as_raw(self), availabledynamicobjectcount, framecountperbuffer).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndUpdatingAudioObjects)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForHrtf)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForHrtf: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataCommands)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObjectForMetadataItems> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataItems)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForMetadataCommands: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActivateSpatialAudioObjectForMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamNotify, ISpatialAudioObjectRenderStreamNotify_Vtbl, 0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
impl core::ops::Deref for ISpatialAudioObjectRenderStreamNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamNotify, windows_core::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    pub unsafe fn OnAvailableDynamicObjectCountChange<P0>(&self, sender: P0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISpatialAudioObjectRenderStreamBase>,
    {
        (windows_core::Interface::vtable(self).OnAvailableDynamicObjectCountChange)(windows_core::Interface::as_raw(self), sender.param().abi(), hnscompliancedeadlinetime, availabledynamicobjectcountchange).ok()
    }
}
#[repr(C)]
pub struct ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAvailableDynamicObjectCountChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISubunit, ISubunit_Vtbl, 0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
impl core::ops::Deref for ISubunit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISubunit, windows_core::IUnknown);
impl ISubunit {}
#[repr(C)]
pub struct ISubunit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
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
pub const ACMERR_BASE: u32 = 512u32;
pub const ACMERR_BUSY: u32 = 513u32;
pub const ACMERR_CANCELED: u32 = 515u32;
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
pub const ACMERR_UNPREPARED: u32 = 514u32;
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
pub const ACMHELPMSGCONTEXTHELP: windows_core::PCWSTR = windows_core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPA: windows_core::PCSTR = windows_core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPW: windows_core::PCWSTR = windows_core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTMENU: windows_core::PCWSTR = windows_core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUA: windows_core::PCSTR = windows_core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUW: windows_core::PCWSTR = windows_core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGSTRING: windows_core::PCWSTR = windows_core::w!("acmchoose_help");
pub const ACMHELPMSGSTRINGA: windows_core::PCSTR = windows_core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("acmchoose_help");
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
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
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
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4i32);
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2i32);
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0i32);
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1i32);
pub const AUDCLNT_S_BUFFER_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0x8890001_u32 as _);
pub const AUDCLNT_S_POSITION_STALLED: windows_core::HRESULT = windows_core::HRESULT(0x8890003_u32 as _);
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x8890002_u32 as _);
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(0i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(1i32);
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0i32);
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(1i32);
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(0i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(3i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(1i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(2i32);
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
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
pub const Connector: PartType = PartType(0i32);
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
pub const DEVICE_STATE_ACTIVE: DEVICE_STATE = DEVICE_STATE(1u32);
pub const DEVICE_STATE_DISABLED: DEVICE_STATE = DEVICE_STATE(2u32);
pub const DEVICE_STATE_NOTPRESENT: DEVICE_STATE = DEVICE_STATE(4u32);
pub const DEVICE_STATE_UNPLUGGED: DEVICE_STATE = DEVICE_STATE(8u32);
pub const DEVINTERFACE_AUDIO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: windows_core::GUID = windows_core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: windows_core::GUID = windows_core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: windows_core::GUID = windows_core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = AudioSessionDisconnectReason(0i32);
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = AudioSessionDisconnectReason(5i32);
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = AudioSessionDisconnectReason(2i32);
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = AudioSessionDisconnectReason(1i32);
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = AudioSessionDisconnectReason(4i32);
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = AudioSessionDisconnectReason(3i32);
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
pub const ERole_enum_count: ERole = ERole(3i32);
pub const EVENTCONTEXT_VOLUMESLIDER: windows_core::GUID = windows_core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
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
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
pub const In: DataFlow = DataFlow(0i32);
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
pub const MIDIPATCHSIZE: u32 = 128u32;
pub const MIDIPROP_GET: i32 = 1073741824i32;
pub const MIDIPROP_SET: i32 = -2147483648i32;
pub const MIDIPROP_TEMPO: i32 = 2i32;
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
pub const MIDISTRM_ERROR: i32 = -2i32;
pub const MIDI_CACHE_ALL: u32 = 1u32;
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
pub const MIDI_CACHE_QUERY: u32 = 3u32;
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
pub const MIDI_UNCACHE: u32 = 4u32;
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
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3 };
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(1i32);
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(0i32);
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
pub const SND_ALIAS: SND_FLAGS = SND_FLAGS(65536u32);
pub const SND_ALIAS_ID: SND_FLAGS = SND_FLAGS(1114112u32);
pub const SND_ALIAS_START: u32 = 0u32;
pub const SND_APPLICATION: SND_FLAGS = SND_FLAGS(128u32);
pub const SND_ASYNC: SND_FLAGS = SND_FLAGS(1u32);
pub const SND_FILENAME: SND_FLAGS = SND_FLAGS(131072u32);
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
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(1i32);
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(2i32);
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(0i32);
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(1i32);
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(0i32);
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(4i32);
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(2i32);
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(1i32);
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(3i32);
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(0i32);
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(1i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(3i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(2i32);
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(0i32);
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(0i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(2i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(1i32);
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
pub const Subunit: PartType = PartType(1i32);
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: windows_core::PCWSTR = windows_core::w!("VAD\\Process_Loopback");
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
pub const WAVECAPS_PITCH: u32 = 1u32;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
pub const WAVECAPS_SYNC: u32 = 16u32;
pub const WAVECAPS_VOLUME: u32 = 4u32;
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
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
pub const eAll: EDataFlow = EDataFlow(2i32);
pub const eCapture: EDataFlow = EDataFlow(1i32);
pub const eCommunications: ERole = ERole(2i32);
pub const eConsole: ERole = ERole(0i32);
pub const eMultimedia: ERole = ERole(1i32);
pub const eRender: EDataFlow = EDataFlow(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
impl windows_core::TypeKind for AMBISONICS_CHANNEL_ORDERING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AMBISONICS_CHANNEL_ORDERING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AMBISONICS_CHANNEL_ORDERING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AMBISONICS_NORMALIZATION(pub i32);
impl windows_core::TypeKind for AMBISONICS_NORMALIZATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AMBISONICS_NORMALIZATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AMBISONICS_NORMALIZATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AMBISONICS_TYPE(pub i32);
impl windows_core::TypeKind for AMBISONICS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AMBISONICS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AMBISONICS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDCLNT_SHAREMODE(pub i32);
impl windows_core::TypeKind for AUDCLNT_SHAREMODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDCLNT_SHAREMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDCLNT_SHAREMODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDCLNT_STREAMOPTIONS(pub i32);
impl windows_core::TypeKind for AUDCLNT_STREAMOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDCLNT_STREAMOPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDCLNT_STREAMOPTIONS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
impl windows_core::TypeKind for AUDIOCLIENT_ACTIVATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIOCLIENT_ACTIVATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIOCLIENT_ACTIVATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIO_DUCKING_OPTIONS(pub i32);
impl windows_core::TypeKind for AUDIO_DUCKING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIO_DUCKING_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIO_DUCKING_OPTIONS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIO_EFFECT_STATE(pub i32);
impl windows_core::TypeKind for AUDIO_EFFECT_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIO_EFFECT_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIO_EFFECT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIO_STREAM_CATEGORY(pub i32);
impl windows_core::TypeKind for AUDIO_STREAM_CATEGORY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIO_STREAM_CATEGORY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIO_STREAM_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE(pub i32);
impl windows_core::TypeKind for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioObjectType(pub i32);
impl windows_core::TypeKind for AudioObjectType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioObjectType").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioSessionDisconnectReason(pub i32);
impl windows_core::TypeKind for AudioSessionDisconnectReason {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioSessionDisconnectReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioSessionDisconnectReason").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioSessionState(pub i32);
impl windows_core::TypeKind for AudioSessionState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioSessionState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioSessionState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AudioStateMonitorSoundLevel(pub i32);
impl windows_core::TypeKind for AudioStateMonitorSoundLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AudioStateMonitorSoundLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AudioStateMonitorSoundLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
impl windows_core::TypeKind for ConnectorType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConnectorType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConnectorType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEVICE_STATE(pub u32);
impl windows_core::TypeKind for DEVICE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEVICE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEVICE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DataFlow(pub i32);
impl windows_core::TypeKind for DataFlow {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DataFlow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DataFlow").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EDataFlow(pub i32);
impl windows_core::TypeKind for EDataFlow {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EDataFlow {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EDataFlow").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ERole(pub i32);
impl windows_core::TypeKind for ERole {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ERole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ERole").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EndpointFormFactor(pub i32);
impl windows_core::TypeKind for EndpointFormFactor {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EndpointFormFactor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EndpointFormFactor").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
impl windows_core::TypeKind for MIDI_WAVE_OPEN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MIDI_WAVE_OPEN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MIDI_WAVE_OPEN_TYPE").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
impl windows_core::TypeKind for MIXERLINE_COMPONENTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MIXERLINE_COMPONENTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MIXERLINE_COMPONENTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
impl windows_core::TypeKind for PROCESS_LOOPBACK_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROCESS_LOOPBACK_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROCESS_LOOPBACK_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PartType(pub i32);
impl windows_core::TypeKind for PartType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PartType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PartType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SND_FLAGS(pub u32);
impl windows_core::TypeKind for SND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SND_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SND_FLAGS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub i32);
impl windows_core::TypeKind for SPATIAL_AUDIO_STREAM_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SPATIAL_AUDIO_STREAM_OPTIONS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
impl windows_core::TypeKind for SpatialAudioHrtfDirectivityType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAudioHrtfDirectivityType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDirectivityType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
impl windows_core::TypeKind for SpatialAudioHrtfDistanceDecayType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAudioHrtfDistanceDecayType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDistanceDecayType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
impl windows_core::TypeKind for SpatialAudioHrtfEnvironmentType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAudioHrtfEnvironmentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfEnvironmentType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAudioMetadataCopyMode(pub i32);
impl windows_core::TypeKind for SpatialAudioMetadataCopyMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAudioMetadataCopyMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataCopyMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
impl windows_core::TypeKind for SpatialAudioMetadataWriterOverflowMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SpatialAudioMetadataWriterOverflowMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataWriterOverflowMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
impl windows_core::TypeKind for _AUDCLNT_BUFFERFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for _AUDCLNT_BUFFERFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("_AUDCLNT_BUFFERFLAGS").field(&self.0).finish()
    }
}
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
impl windows_core::TypeKind for ACMDRIVERDETAILSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for ACMDRIVERDETAILSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMDRVFORMATSUGGEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
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
impl windows_core::TypeKind for ACMDRVOPENDESCA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMDRVOPENDESCA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
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
impl windows_core::TypeKind for ACMDRVOPENDESCW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMDRVOPENDESCW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for ACMDRVSTREAMHEADER {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for ACMDRVSTREAMINSTANCE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl windows_core::TypeKind for ACMDRVSTREAMSIZE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMDRVSTREAMSIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFILTERCHOOSEA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFILTERCHOOSEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFILTERDETAILSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for ACMFILTERDETAILSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFILTERTAGDETAILSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for ACMFILTERTAGDETAILSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFORMATCHOOSEA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFORMATCHOOSEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFORMATDETAILSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMFORMATTAGDETAILSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for ACMFORMATTAGDETAILSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACMFORMATTAGDETAILSW {
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
impl windows_core::TypeKind for ACMSTREAMHEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for ACMSTREAMHEADER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for AMBISONICS_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl windows_core::TypeKind for AUDIOCLIENT_ACTIVATION_PARAMS {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
impl windows_core::TypeKind for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIO_EFFECT {
    pub id: windows_core::GUID,
    pub canSetState: super::super::Foundation::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
impl windows_core::TypeKind for AUDIO_EFFECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIO_EFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: windows_core::GUID,
    pub bMuted: super::super::Foundation::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
impl windows_core::TypeKind for AUDIO_VOLUME_NOTIFICATION_DATA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for AUXCAPS2A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for AUXCAPS2W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for AUXCAPSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for AUXCAPSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: windows_core::GUID,
}
impl windows_core::TypeKind for AudioClient3ActivationParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for AudioClient3ActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: super::super::Foundation::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
impl windows_core::TypeKind for AudioClientProperties {
    type TypeKind = windows_core::CopyType;
}
impl Default for AudioClientProperties {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: core::mem::ManuallyDrop<Option<IMMDevice>>,
    pub pPnpInterface: core::mem::ManuallyDrop<Option<IMMDevice>>,
    pub pPnpDevnode: core::mem::ManuallyDrop<Option<IMMDevice>>,
}
impl Clone for AudioExtensionParams {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AudioExtensionParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: windows_core::GUID,
    pub dwAudioStreamFlags: u32,
}
impl windows_core::TypeKind for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DeviceTopology: windows_core::GUID = windows_core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl windows_core::TypeKind for ECHOWAVEFILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for ECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
            _ = acmDriverClose(*self, 0);
        }
    }
}
impl Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HACMDRIVER {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for HACMDRIVERID {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for HACMOBJ {
    type TypeKind = windows_core::CopyType;
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
            _ = acmStreamClose(*self, 0);
        }
    }
}
impl Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HACMSTREAM {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for HMIDI {
    type TypeKind = windows_core::CopyType;
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
            _ = midiInClose(*self);
        }
    }
}
impl Default for HMIDIIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HMIDIIN {
    type TypeKind = windows_core::CopyType;
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
            _ = midiOutClose(*self);
        }
    }
}
impl Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HMIDIOUT {
    type TypeKind = windows_core::CopyType;
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
            _ = midiStreamClose(*self);
        }
    }
}
impl Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HMIDISTRM {
    type TypeKind = windows_core::CopyType;
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
            _ = mixerClose(*self);
        }
    }
}
impl Default for HMIXER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HMIXER {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for HMIXEROBJ {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for HWAVE {
    type TypeKind = windows_core::CopyType;
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
            _ = waveInClose(*self);
        }
    }
}
impl Default for HWAVEIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HWAVEIN {
    type TypeKind = windows_core::CopyType;
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
            _ = waveOutClose(*self);
        }
    }
}
impl Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HWAVEOUT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl windows_core::TypeKind for MIDIEVENT {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIHDR {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIINCAPS2A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIINCAPS2W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIINCAPSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIINCAPSW {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIOUTCAPS2A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIOUTCAPS2W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIOUTCAPSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIDIOUTCAPSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl windows_core::TypeKind for MIDIPROPTEMPO {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl windows_core::TypeKind for MIDIPROPTIMEDIV {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl windows_core::TypeKind for MIDISTRMBUFFVER {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIDISTRMBUFFVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
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
impl windows_core::TypeKind for MIXERCAPS2A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCAPS2W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCAPSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCAPSW {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCONTROLA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCONTROLA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl windows_core::TypeKind for MIXERCONTROLA_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl windows_core::TypeKind for MIXERCONTROLA_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl windows_core::TypeKind for MIXERCONTROLA_1 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCONTROLDETAILS {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCONTROLDETAILS_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl windows_core::TypeKind for MIXERCONTROLDETAILS_BOOLEAN {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [i8; 64],
}
impl windows_core::TypeKind for MIXERCONTROLDETAILS_LISTTEXTA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCONTROLDETAILS_LISTTEXTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl windows_core::TypeKind for MIXERCONTROLDETAILS_SIGNED {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl windows_core::TypeKind for MIXERCONTROLDETAILS_UNSIGNED {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for MIXERCONTROLW {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERCONTROLW_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl windows_core::TypeKind for MIXERCONTROLW_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLW_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl windows_core::TypeKind for MIXERCONTROLW_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLW_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl windows_core::TypeKind for MIXERCONTROLW_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for MIXERLINEA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINEA_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINECONTROLSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINECONTROLSA_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINECONTROLSW {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINECONTROLSW_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINEW {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MIXERLINEW_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MMDeviceEnumerator: windows_core::GUID = windows_core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl windows_core::TypeKind for PCMWAVEFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: windows_core::GUID,
    pub appId: windows_core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
impl windows_core::TypeKind for SpatialAudioClientActivationParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioClientActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for SpatialAudioHrtfActivationParams {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for SpatialAudioHrtfActivationParams2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioHrtfActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
impl windows_core::TypeKind for SpatialAudioHrtfDirectivity {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioHrtfDirectivity {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
impl windows_core::TypeKind for SpatialAudioHrtfDirectivityCardioid {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioHrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
impl windows_core::TypeKind for SpatialAudioHrtfDirectivityCone {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioHrtfDirectivityCone {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SpatialAudioHrtfDirectivityUnion {
    pub Cone: SpatialAudioHrtfDirectivityCone,
    pub Cardiod: SpatialAudioHrtfDirectivityCardioid,
    pub Omni: SpatialAudioHrtfDirectivity,
}
impl windows_core::TypeKind for SpatialAudioHrtfDirectivityUnion {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
impl windows_core::TypeKind for SpatialAudioHrtfDistanceDecay {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioHrtfDistanceDecay {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
impl windows_core::TypeKind for SpatialAudioMetadataItemsInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioMetadataItemsInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for SpatialAudioObjectRenderStreamActivationParams {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for SpatialAudioObjectRenderStreamActivationParams2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: windows_core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const windows_core::PROPVARIANT,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
}
impl windows_core::TypeKind for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: windows_core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const windows_core::PROPVARIANT,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
impl windows_core::TypeKind for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl windows_core::TypeKind for VOLUMEWAVEFILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for VOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl windows_core::TypeKind for WAVEFILTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl windows_core::TypeKind for WAVEFORMAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAVEFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl windows_core::TypeKind for WAVEFORMATEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAVEFORMATEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: windows_core::GUID,
}
impl windows_core::TypeKind for WAVEFORMATEXTENSIBLE {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEFORMATEXTENSIBLE_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEHDR {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEINCAPS2A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEINCAPS2W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEINCAPSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEINCAPSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for WAVEOUTCAPS2A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEOUTCAPS2W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEOUTCAPSA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WAVEOUTCAPSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for tACMFORMATDETAILSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ACMDRIVERENUMCB = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFILTERCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type ACMFILTERCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type ACMFILTERENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFILTERENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFILTERTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFILTERTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFORMATCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type ACMFORMATCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
pub type ACMFORMATENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFORMATENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFORMATTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type ACMFORMATTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
pub type LPACMDRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type PAudioStateMonitorCallback = Option<unsafe extern "system" fn(audiostatemonitor: Option<IAudioStateMonitor>, context: *const core::ffi::c_void)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
