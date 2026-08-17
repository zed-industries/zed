#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_targets::link!("mmdevapi.dll" "system" fn ActivateAudioInterfaceAsync(deviceinterfacepath : windows_sys::core::PCWSTR, riid : *const windows_sys::core::GUID, activationparams : *const super::super::System::Com::StructuredStorage:: PROPVARIANT, completionhandler : * mut core::ffi::c_void, activationoperation : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoRegisterMessageFilter(lpmessagefilter : * mut core::ffi::c_void, lplpmessagefilter : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitor(audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategory(category : AUDIO_STREAM_CATEGORY, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category : AUDIO_STREAM_CATEGORY, deviceid : windows_sys::core::PCWSTR, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category : AUDIO_STREAM_CATEGORY, role : ERole, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitor(audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategory(category : AUDIO_STREAM_CATEGORY, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category : AUDIO_STREAM_CATEGORY, deviceid : windows_sys::core::PCWSTR, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("windows.media.mediacontrol.dll" "system" fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category : AUDIO_STREAM_CATEGORY, role : ERole, audiostatemonitor : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("winmm.dll" "system" fn PlaySoundA(pszsound : windows_sys::core::PCSTR, hmod : super::super::Foundation:: HMODULE, fdwsound : SND_FLAGS) -> windows_sys::core::BOOL);
windows_targets::link!("winmm.dll" "system" fn PlaySoundW(pszsound : windows_sys::core::PCWSTR, hmod : super::super::Foundation:: HMODULE, fdwsound : SND_FLAGS) -> windows_sys::core::BOOL);
windows_targets::link!("msacm32.dll" "system" fn acmDriverAddA(phadid : *mut HACMDRIVERID, hinstmodule : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM, dwpriority : u32, fdwadd : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverAddW(phadid : *mut HACMDRIVERID, hinstmodule : super::super::Foundation:: HINSTANCE, lparam : super::super::Foundation:: LPARAM, dwpriority : u32, fdwadd : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverClose(had : HACMDRIVER, fdwclose : u32) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("msacm32.dll" "system" fn acmDriverDetailsA(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSA, fdwdetails : u32) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("msacm32.dll" "system" fn acmDriverDetailsW(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSW, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverEnum(fncallback : ACMDRIVERENUMCB, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverID(hao : HACMOBJ, phadid : *mut HACMDRIVERID, fdwdriverid : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverMessage(had : HACMDRIVER, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> super::super::Foundation:: LRESULT);
windows_targets::link!("msacm32.dll" "system" fn acmDriverOpen(phad : *mut HACMDRIVER, hadid : HACMDRIVERID, fdwopen : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverPriority(hadid : HACMDRIVERID, dwpriority : u32, fdwpriority : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmDriverRemove(hadid : HACMDRIVERID, fdwremove : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterChooseA(pafltrc : *mut ACMFILTERCHOOSEA) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterChooseW(pafltrc : *mut ACMFILTERCHOOSEW) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterDetailsA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterDetailsW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterEnumA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fncallback : ACMFILTERENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterEnumW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fncallback : ACMFILTERENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterTagEnumA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fncallback : ACMFILTERTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFilterTagEnumW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fncallback : ACMFILTERTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatChooseA(pafmtc : *mut ACMFORMATCHOOSEA) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatChooseW(pafmtc : *mut ACMFORMATCHOOSEW) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatDetailsA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatDetailsW(had : HACMDRIVER, pafd : *mut tACMFORMATDETAILSW, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatEnumA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fncallback : ACMFORMATENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatEnumW(had : HACMDRIVER, pafd : *mut tACMFORMATDETAILSW, fncallback : ACMFORMATENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatSuggest(had : HACMDRIVER, pwfxsrc : *mut WAVEFORMATEX, pwfxdst : *mut WAVEFORMATEX, cbwfxdst : u32, fdwsuggest : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fdwdetails : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatTagEnumA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fncallback : ACMFORMATTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmFormatTagEnumW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fncallback : ACMFORMATTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmGetVersion() -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmMetrics(hao : HACMOBJ, umetric : u32, pmetric : *mut core::ffi::c_void) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamClose(has : HACMSTREAM, fdwclose : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamConvert(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwconvert : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamMessage(has : HACMSTREAM, umsg : u32, lparam1 : super::super::Foundation:: LPARAM, lparam2 : super::super::Foundation:: LPARAM) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamOpen(phas : *mut HACMSTREAM, had : HACMDRIVER, pwfxsrc : *mut WAVEFORMATEX, pwfxdst : *mut WAVEFORMATEX, pwfltr : *mut WAVEFILTER, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamPrepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwprepare : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamReset(has : HACMSTREAM, fdwreset : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamSize(has : HACMSTREAM, cbinput : u32, pdwoutputbytes : *mut u32, fdwsize : u32) -> u32);
windows_targets::link!("msacm32.dll" "system" fn acmStreamUnprepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwunprepare : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn auxGetDevCapsA(udeviceid : usize, pac : *mut AUXCAPSA, cbac : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn auxGetDevCapsW(udeviceid : usize, pac : *mut AUXCAPSW, cbac : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn auxGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn auxGetVolume(udeviceid : u32, pdwvolume : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn auxOutMessage(udeviceid : u32, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn auxSetVolume(udeviceid : u32, dwvolume : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiConnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiDisconnect(hmi : HMIDI, hmo : HMIDIOUT, preserved : *const core::ffi::c_void) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInAddBuffer(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInClose(hmi : HMIDIIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInGetDevCapsA(udeviceid : usize, pmic : *mut MIDIINCAPSA, cbmic : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInGetDevCapsW(udeviceid : usize, pmic : *mut MIDIINCAPSW, cbmic : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInGetErrorTextA(mmrerror : u32, psztext : windows_sys::core::PSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInGetErrorTextW(mmrerror : u32, psztext : windows_sys::core::PWSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInGetID(hmi : HMIDIIN, pudeviceid : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInMessage(hmi : HMIDIIN, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInOpen(phmi : *mut HMIDIIN, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInPrepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInReset(hmi : HMIDIIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInStart(hmi : HMIDIIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInStop(hmi : HMIDIIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiInUnprepareHeader(hmi : HMIDIIN, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutCacheDrumPatches(hmo : HMIDIOUT, upatch : u32, pwkya : *const u16, fucache : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutCachePatches(hmo : HMIDIOUT, ubank : u32, pwpa : *const u16, fucache : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutClose(hmo : HMIDIOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetDevCapsA(udeviceid : usize, pmoc : *mut MIDIOUTCAPSA, cbmoc : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetDevCapsW(udeviceid : usize, pmoc : *mut MIDIOUTCAPSW, cbmoc : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetErrorTextA(mmrerror : u32, psztext : windows_sys::core::PSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetErrorTextW(mmrerror : u32, psztext : windows_sys::core::PWSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetID(hmo : HMIDIOUT, pudeviceid : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutGetVolume(hmo : HMIDIOUT, pdwvolume : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutLongMsg(hmo : HMIDIOUT, pmh : *const MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutMessage(hmo : HMIDIOUT, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutOpen(phmo : *mut HMIDIOUT, udeviceid : u32, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutPrepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutReset(hmo : HMIDIOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutSetVolume(hmo : HMIDIOUT, dwvolume : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutShortMsg(hmo : HMIDIOUT, dwmsg : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiOutUnprepareHeader(hmo : HMIDIOUT, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamClose(hms : HMIDISTRM) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamOpen(phms : *mut HMIDISTRM, pudeviceid : *mut u32, cmidi : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamOut(hms : HMIDISTRM, pmh : *mut MIDIHDR, cbmh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamPause(hms : HMIDISTRM) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamPosition(hms : HMIDISTRM, lpmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamProperty(hms : HMIDISTRM, lppropdata : *mut u8, dwproperty : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamRestart(hms : HMIDISTRM) -> u32);
windows_targets::link!("winmm.dll" "system" fn midiStreamStop(hms : HMIDISTRM) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerClose(hmx : HMIXER) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetControlDetailsA(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetControlDetailsW(hmxobj : HMIXEROBJ, pmxcd : *mut MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetDevCapsA(umxid : usize, pmxcaps : *mut MIXERCAPSA, cbmxcaps : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetDevCapsW(umxid : usize, pmxcaps : *mut MIXERCAPSW, cbmxcaps : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetID(hmxobj : HMIXEROBJ, pumxid : *mut u32, fdwid : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetLineControlsA(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSA, fdwcontrols : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetLineControlsW(hmxobj : HMIXEROBJ, pmxlc : *mut MIXERLINECONTROLSW, fdwcontrols : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetLineInfoA(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEA, fdwinfo : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetLineInfoW(hmxobj : HMIXEROBJ, pmxl : *mut MIXERLINEW, fdwinfo : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerMessage(hmx : HMIXER, umsg : u32, dwparam1 : usize, dwparam2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerOpen(phmx : *mut HMIXER, umxid : u32, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn mixerSetControlDetails(hmxobj : HMIXEROBJ, pmxcd : *const MIXERCONTROLDETAILS, fdwdetails : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn sndPlaySoundA(pszsound : windows_sys::core::PCSTR, fusound : u32) -> windows_sys::core::BOOL);
windows_targets::link!("winmm.dll" "system" fn sndPlaySoundW(pszsound : windows_sys::core::PCWSTR, fusound : u32) -> windows_sys::core::BOOL);
windows_targets::link!("winmm.dll" "system" fn waveInAddBuffer(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInClose(hwi : HWAVEIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetDevCapsA(udeviceid : usize, pwic : *mut WAVEINCAPSA, cbwic : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetDevCapsW(udeviceid : usize, pwic : *mut WAVEINCAPSW, cbwic : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetErrorTextA(mmrerror : u32, psztext : windows_sys::core::PSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetErrorTextW(mmrerror : u32, psztext : windows_sys::core::PWSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetID(hwi : HWAVEIN, pudeviceid : *const u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInGetPosition(hwi : HWAVEIN, pmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInMessage(hwi : HWAVEIN, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInOpen(phwi : *mut HWAVEIN, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInPrepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInReset(hwi : HWAVEIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInStart(hwi : HWAVEIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInStop(hwi : HWAVEIN) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveInUnprepareHeader(hwi : HWAVEIN, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutBreakLoop(hwo : HWAVEOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutClose(hwo : HWAVEOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetDevCapsA(udeviceid : usize, pwoc : *mut WAVEOUTCAPSA, cbwoc : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetDevCapsW(udeviceid : usize, pwoc : *mut WAVEOUTCAPSW, cbwoc : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetErrorTextA(mmrerror : u32, psztext : windows_sys::core::PSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetErrorTextW(mmrerror : u32, psztext : windows_sys::core::PWSTR, cchtext : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetID(hwo : HWAVEOUT, pudeviceid : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetNumDevs() -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetPitch(hwo : HWAVEOUT, pdwpitch : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetPlaybackRate(hwo : HWAVEOUT, pdwrate : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetPosition(hwo : HWAVEOUT, pmmt : *mut super:: MMTIME, cbmmt : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutGetVolume(hwo : HWAVEOUT, pdwvolume : *mut u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutMessage(hwo : HWAVEOUT, umsg : u32, dw1 : usize, dw2 : usize) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutOpen(phwo : *mut HWAVEOUT, udeviceid : u32, pwfx : *const WAVEFORMATEX, dwcallback : usize, dwinstance : usize, fdwopen : MIDI_WAVE_OPEN_TYPE) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutPause(hwo : HWAVEOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutPrepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutReset(hwo : HWAVEOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutRestart(hwo : HWAVEOUT) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutSetPitch(hwo : HWAVEOUT, dwpitch : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutSetPlaybackRate(hwo : HWAVEOUT, dwrate : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutSetVolume(hwo : HWAVEOUT, dwvolume : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutUnprepareHeader(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
windows_targets::link!("winmm.dll" "system" fn waveOutWrite(hwo : HWAVEOUT, pwh : *mut WAVEHDR, cbwh : u32) -> u32);
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
pub type ACMDRIVERENUMCB = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
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
#[derive(Clone, Copy)]
pub struct ACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: windows_sys::core::PCSTR,
    pub pszAliasName: windows_sys::core::PCSTR,
    pub dnDevNode: u32,
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
    pub pszSectionName: windows_sys::core::PCWSTR,
    pub pszAliasName: windows_sys::core::PCWSTR,
    pub dnDevNode: u32,
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
    pub pszTitle: windows_sys::core::PCSTR,
    pub szFilterTag: [i8; 48],
    pub szFilter: [i8; 128],
    pub pszName: windows_sys::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCSTR,
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
    pub pszTitle: windows_sys::core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: windows_sys::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCWSTR,
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
pub type ACMFILTERENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub type ACMFILTERENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
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
pub type ACMFILTERTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub type ACMFILTERTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_sys::core::PCSTR,
    pub szFormatTag: [i8; 48],
    pub szFormat: [i8; 128],
    pub pszName: windows_sys::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCSTR,
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
    pub pszTitle: windows_sys::core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: windows_sys::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCWSTR,
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
pub type ACMFORMATENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub type ACMFORMATENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
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
pub type ACMFORMATTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub type ACMFORMATTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub const ACMHELPMSGCONTEXTHELP: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTMENU: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_help");
pub const ACMHELPMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_help");
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
pub type AMBISONICS_CHANNEL_ORDERING = i32;
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = 0i32;
pub type AMBISONICS_NORMALIZATION = i32;
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = 1i32;
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
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
pub type AMBISONICS_TYPE = i32;
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = 0i32;
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = 1i32;
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = 2i32;
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = 4i32;
pub const AUDCLNT_E_ALREADY_INITIALIZED: windows_sys::core::HRESULT = 0x88890002_u32 as _;
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: windows_sys::core::HRESULT = 0x88890013_u32 as _;
pub const AUDCLNT_E_BUFFER_ERROR: windows_sys::core::HRESULT = 0x88890018_u32 as _;
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: windows_sys::core::HRESULT = 0x8889000B_u32 as _;
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: windows_sys::core::HRESULT = 0x88890016_u32 as _;
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: windows_sys::core::HRESULT = 0x88890019_u32 as _;
pub const AUDCLNT_E_BUFFER_TOO_LARGE: windows_sys::core::HRESULT = 0x88890006_u32 as _;
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: windows_sys::core::HRESULT = 0x88890017_u32 as _;
pub const AUDCLNT_E_DEVICE_INVALIDATED: windows_sys::core::HRESULT = 0x88890004_u32 as _;
pub const AUDCLNT_E_DEVICE_IN_USE: windows_sys::core::HRESULT = 0x8889000A_u32 as _;
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x88890041_u32 as _;
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: windows_sys::core::HRESULT = 0x88890042_u32 as _;
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: windows_sys::core::HRESULT = 0x8889000F_u32 as _;
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: windows_sys::core::HRESULT = 0x88890022_u32 as _;
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: windows_sys::core::HRESULT = 0x88890029_u32 as _;
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: windows_sys::core::HRESULT = 0x88890028_u32 as _;
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: windows_sys::core::HRESULT = 0x88890011_u32 as _;
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: windows_sys::core::HRESULT = 0x88890014_u32 as _;
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: windows_sys::core::HRESULT = 0x8889000E_u32 as _;
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: windows_sys::core::HRESULT = 0x88890012_u32 as _;
pub const AUDCLNT_E_HEADTRACKING_ENABLED: windows_sys::core::HRESULT = 0x88890030_u32 as _;
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: windows_sys::core::HRESULT = 0x88890040_u32 as _;
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: windows_sys::core::HRESULT = 0x88890015_u32 as _;
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: windows_sys::core::HRESULT = 0x88890020_u32 as _;
pub const AUDCLNT_E_INVALID_SIZE: windows_sys::core::HRESULT = 0x88890009_u32 as _;
pub const AUDCLNT_E_INVALID_STREAM_FLAG: windows_sys::core::HRESULT = 0x88890021_u32 as _;
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: windows_sys::core::HRESULT = 0x88890025_u32 as _;
pub const AUDCLNT_E_NOT_INITIALIZED: windows_sys::core::HRESULT = 0x88890001_u32 as _;
pub const AUDCLNT_E_NOT_STOPPED: windows_sys::core::HRESULT = 0x88890005_u32 as _;
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: windows_sys::core::HRESULT = 0x88890024_u32 as _;
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: windows_sys::core::HRESULT = 0x88890023_u32 as _;
pub const AUDCLNT_E_OUT_OF_ORDER: windows_sys::core::HRESULT = 0x88890007_u32 as _;
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: windows_sys::core::HRESULT = 0x88890027_u32 as _;
pub const AUDCLNT_E_RESOURCES_INVALIDATED: windows_sys::core::HRESULT = 0x88890026_u32 as _;
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: windows_sys::core::HRESULT = 0x88890010_u32 as _;
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: windows_sys::core::HRESULT = 0x8889000C_u32 as _;
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: windows_sys::core::HRESULT = 0x88890008_u32 as _;
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: windows_sys::core::HRESULT = 0x88890003_u32 as _;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
pub type AUDCLNT_SHAREMODE = i32;
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = 1i32;
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = 0i32;
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
pub type AUDCLNT_STREAMOPTIONS = i32;
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = 4i32;
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = 2i32;
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = 0i32;
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = 1i32;
pub const AUDCLNT_S_BUFFER_EMPTY: windows_sys::core::HRESULT = 0x8890001_u32 as _;
pub const AUDCLNT_S_POSITION_STALLED: windows_sys::core::HRESULT = 0x8890003_u32 as _;
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: windows_sys::core::HRESULT = 0x8890002_u32 as _;
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
pub type AUDIOCLIENT_ACTIVATION_TYPE = i32;
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = 0i32;
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
pub type AUDIO_DUCKING_OPTIONS = i32;
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = 0i32;
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AUDIO_EFFECT {
    pub id: windows_sys::core::GUID,
    pub canSetState: windows_sys::core::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
pub type AUDIO_EFFECT_STATE = i32;
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = 0i32;
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = 1i32;
pub type AUDIO_STREAM_CATEGORY = i32;
pub type AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = i32;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 0i32;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 3i32;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 1i32;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: windows_sys::core::GUID,
    pub bMuted: windows_sys::core::BOOL,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = 4i32;
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = 3i32;
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = 12i32;
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = 1i32;
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = 8i32;
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = 6i32;
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = 7i32;
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = 11i32;
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = 10i32;
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = 0i32;
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = 5i32;
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = 9i32;
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = 13i32;
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = 14i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: windows_sys::core::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: *mut core::ffi::c_void,
    pub pPnpInterface: *mut core::ffi::c_void,
    pub pPnpDevnode: *mut core::ffi::c_void,
}
impl Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AudioObjectType = i32;
pub const AudioObjectType_BackCenter: AudioObjectType = 131072i32;
pub const AudioObjectType_BackLeft: AudioObjectType = 128i32;
pub const AudioObjectType_BackRight: AudioObjectType = 256i32;
pub const AudioObjectType_BottomBackLeft: AudioObjectType = 32768i32;
pub const AudioObjectType_BottomBackRight: AudioObjectType = 65536i32;
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = 8192i32;
pub const AudioObjectType_BottomFrontRight: AudioObjectType = 16384i32;
pub const AudioObjectType_Dynamic: AudioObjectType = 1i32;
pub const AudioObjectType_FrontCenter: AudioObjectType = 8i32;
pub const AudioObjectType_FrontLeft: AudioObjectType = 2i32;
pub const AudioObjectType_FrontRight: AudioObjectType = 4i32;
pub const AudioObjectType_LowFrequency: AudioObjectType = 16i32;
pub const AudioObjectType_None: AudioObjectType = 0i32;
pub const AudioObjectType_SideLeft: AudioObjectType = 32i32;
pub const AudioObjectType_SideRight: AudioObjectType = 64i32;
pub const AudioObjectType_TopBackLeft: AudioObjectType = 2048i32;
pub const AudioObjectType_TopBackRight: AudioObjectType = 4096i32;
pub const AudioObjectType_TopFrontLeft: AudioObjectType = 512i32;
pub const AudioObjectType_TopFrontRight: AudioObjectType = 1024i32;
pub type AudioSessionDisconnectReason = i32;
pub type AudioSessionState = i32;
pub const AudioSessionStateActive: AudioSessionState = 1i32;
pub const AudioSessionStateExpired: AudioSessionState = 2i32;
pub const AudioSessionStateInactive: AudioSessionState = 0i32;
pub type AudioStateMonitorSoundLevel = i32;
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = 327680u32;
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = 196608u32;
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = 0u32;
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = 131072u32;
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = 131072u32;
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = 458752u32;
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = 65536u32;
pub const Connector: PartType = 0i32;
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
pub type DEVICE_STATE = u32;
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
pub const DEVICE_STATE_ACTIVE: DEVICE_STATE = 1u32;
pub const DEVICE_STATE_DISABLED: DEVICE_STATE = 2u32;
pub const DEVICE_STATE_NOTPRESENT: DEVICE_STATE = 4u32;
pub const DEVICE_STATE_UNPLUGGED: DEVICE_STATE = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: windows_sys::core::GUID,
    pub dwAudioStreamFlags: u32,
}
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
pub type DataFlow = i32;
pub const DeviceTopology: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
pub const DigitalAudioDisplayDevice: EndpointFormFactor = 9i32;
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = 0i32;
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = 5i32;
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = 2i32;
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = 1i32;
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = 4i32;
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = 3i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
pub type EDataFlow = i32;
pub const EDataFlow_enum_count: EDataFlow = 3i32;
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
pub type ERole = i32;
pub const ERole_enum_count: ERole = 3i32;
pub const EVENTCONTEXT_VOLUMESLIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
pub type EndpointFormFactor = i32;
pub const EndpointFormFactor_enum_count: EndpointFormFactor = 11i32;
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
pub const Full: AudioStateMonitorSoundLevel = 2i32;
pub type HACMDRIVER = *mut core::ffi::c_void;
pub type HACMDRIVERID = *mut core::ffi::c_void;
pub type HACMOBJ = *mut core::ffi::c_void;
pub type HACMSTREAM = *mut core::ffi::c_void;
pub type HMIDI = *mut core::ffi::c_void;
pub type HMIDIIN = *mut core::ffi::c_void;
pub type HMIDIOUT = *mut core::ffi::c_void;
pub type HMIDISTRM = *mut core::ffi::c_void;
pub type HMIXER = *mut core::ffi::c_void;
pub type HMIXEROBJ = *mut core::ffi::c_void;
pub type HWAVE = *mut core::ffi::c_void;
pub type HWAVEIN = *mut core::ffi::c_void;
pub type HWAVEOUT = *mut core::ffi::c_void;
pub const Handset: EndpointFormFactor = 6i32;
pub const Headphones: EndpointFormFactor = 3i32;
pub const Headset: EndpointFormFactor = 5i32;
pub const In: DataFlow = 0i32;
pub type LPACMDRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub const LineLevel: EndpointFormFactor = 2i32;
pub const Low: AudioStateMonitorSoundLevel = 1i32;
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
    pub lpData: windows_sys::core::PSTR,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = 32u32;
pub const MIDI_UNCACHE: u32 = 4u32;
pub type MIDI_WAVE_OPEN_TYPE = u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [i8; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
pub type MIXERLINE_COMPONENTTYPE = u32;
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = 1u32;
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = 5u32;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = 2u32;
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = 3u32;
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = 4u32;
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = 6u32;
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = 0u32;
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = 8u32;
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = 7u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = 4106u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = 4105u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = 4101u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = 4097u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = 4098u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = 4099u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = 4103u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = 4100u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = 4102u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = 4096u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = 4104u32;
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
pub const MMDeviceEnumerator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
pub const MOD_FMSYNTH: u32 = 4u32;
pub const MOD_MAPPER: u32 = 5u32;
pub const MOD_MIDIPORT: u32 = 1u32;
pub const MOD_SQSYNTH: u32 = 3u32;
pub const MOD_SWSYNTH: u32 = 7u32;
pub const MOD_SYNTH: u32 = 2u32;
pub const MOD_WAVETABLE: u32 = 6u32;
pub const Microphone: EndpointFormFactor = 4i32;
pub const Muted: AudioStateMonitorSoundLevel = 0i32;
pub const Out: DataFlow = 1i32;
pub type PAudioStateMonitorCallback = Option<unsafe extern "system" fn(audiostatemonitor: *mut core::ffi::c_void, context: *const core::ffi::c_void)>;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0 };
pub const PKEY_AudioEndpointLogo_IconPath: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1 };
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1 };
pub const PKEY_AudioEndpointSettings_MenuText: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0 };
pub const PKEY_AudioEndpoint_Association: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2 };
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1 };
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9 };
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5 };
pub const PKEY_AudioEndpoint_FormFactor: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0 };
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6 };
pub const PKEY_AudioEndpoint_GUID: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4 };
pub const PKEY_AudioEndpoint_JackSubType: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8 };
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3 };
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7 };
pub const PKEY_AudioEngine_DeviceFormat: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0 };
pub const PKEY_AudioEngine_OEMFormat: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3 };
pub type PROCESS_LOOPBACK_MODE = i32;
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = 1i32;
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = 0i32;
pub type PartType = i32;
pub const RemoteNetworkDevice: EndpointFormFactor = 0i32;
pub const SND_ALIAS: SND_FLAGS = 65536u32;
pub const SND_ALIAS_ID: SND_FLAGS = 1114112u32;
pub const SND_ALIAS_START: u32 = 0u32;
pub const SND_APPLICATION: SND_FLAGS = 128u32;
pub const SND_ASYNC: SND_FLAGS = 1u32;
pub const SND_FILENAME: SND_FLAGS = 131072u32;
pub type SND_FLAGS = u32;
pub const SND_LOOP: SND_FLAGS = 8u32;
pub const SND_MEMORY: SND_FLAGS = 4u32;
pub const SND_NODEFAULT: SND_FLAGS = 2u32;
pub const SND_NOSTOP: SND_FLAGS = 16u32;
pub const SND_NOWAIT: SND_FLAGS = 8192u32;
pub const SND_PURGE: SND_FLAGS = 64u32;
pub const SND_RESOURCE: SND_FLAGS = 262148u32;
pub const SND_RING: i32 = 1048576i32;
pub const SND_SENTRY: SND_FLAGS = 524288u32;
pub const SND_SYNC: SND_FLAGS = 0u32;
pub const SND_SYSTEM: SND_FLAGS = 2097152u32;
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
pub type SPATIAL_AUDIO_STREAM_OPTIONS = i32;
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = 0i32;
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = 1i32;
pub const SPDIF: EndpointFormFactor = 8i32;
pub const SPTLAUDCLNT_E_DESTROYED: windows_sys::core::HRESULT = 0x88890100_u32 as _;
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: windows_sys::core::HRESULT = 0x88890105_u32 as _;
pub const SPTLAUDCLNT_E_INTERNAL: windows_sys::core::HRESULT = 0x8889010D_u32 as _;
pub const SPTLAUDCLNT_E_INVALID_LICENSE: windows_sys::core::HRESULT = 0x88890108_u32 as _;
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: windows_sys::core::HRESULT = 0x88890106_u32 as _;
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: windows_sys::core::HRESULT = 0x88890103_u32 as _;
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: windows_sys::core::HRESULT = 0x8889010C_u32 as _;
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: windows_sys::core::HRESULT = 0x88890101_u32 as _;
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: windows_sys::core::HRESULT = 0x88890104_u32 as _;
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: windows_sys::core::HRESULT = 0x88890102_u32 as _;
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x8889010B_u32 as _;
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: windows_sys::core::HRESULT = 0x88890107_u32 as _;
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: windows_sys::core::HRESULT = 0x8889010A_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: windows_sys::core::HRESULT = 0x88890214_u32 as _;
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: windows_sys::core::HRESULT = 0x88890207_u32 as _;
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: windows_sys::core::HRESULT = 0x88890208_u32 as _;
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: windows_sys::core::HRESULT = 0x88890224_u32 as _;
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: windows_sys::core::HRESULT = 0x88890222_u32 as _;
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: windows_sys::core::HRESULT = 0x88890200_u32 as _;
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: windows_sys::core::HRESULT = 0x88890215_u32 as _;
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: windows_sys::core::HRESULT = 0x88890223_u32 as _;
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x88890209_u32 as _;
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: windows_sys::core::HRESULT = 0x88890218_u32 as _;
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: windows_sys::core::HRESULT = 0x88890202_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: windows_sys::core::HRESULT = 0x88890213_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: windows_sys::core::HRESULT = 0x88890225_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: windows_sys::core::HRESULT = 0x88890211_u32 as _;
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: windows_sys::core::HRESULT = 0x88890219_u32 as _;
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: windows_sys::core::HRESULT = 0x88890205_u32 as _;
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: windows_sys::core::HRESULT = 0x88890203_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: windows_sys::core::HRESULT = 0x88890216_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: windows_sys::core::HRESULT = 0x88890220_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: windows_sys::core::HRESULT = 0x88890210_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: windows_sys::core::HRESULT = 0x88890212_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: windows_sys::core::HRESULT = 0x88890221_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: windows_sys::core::HRESULT = 0x88890206_u32 as _;
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: windows_sys::core::HRESULT = 0x88890217_u32 as _;
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: windows_sys::core::HRESULT = 0x88890201_u32 as _;
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: windows_sys::core::HRESULT = 0x88890204_u32 as _;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: windows_sys::core::GUID,
    pub appId: windows_sys::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
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
#[derive(Clone, Copy)]
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
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
pub type SpatialAudioHrtfDirectivityType = i32;
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
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = 1i32;
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = 2i32;
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = 0i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
pub type SpatialAudioHrtfDistanceDecayType = i32;
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = 1i32;
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = 0i32;
pub type SpatialAudioHrtfEnvironmentType = i32;
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = 4i32;
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = 2i32;
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = 1i32;
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = 3i32;
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = 0i32;
pub type SpatialAudioMetadataCopyMode = i32;
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = 1i32;
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = 3i32;
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = 2i32;
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = 0i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
pub type SpatialAudioMetadataWriterOverflowMode = i32;
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = 0i32;
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = 2i32;
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = 1i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
}
impl Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: *mut core::ffi::c_void,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
impl Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: windows_sys::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: windows_sys::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: *mut core::ffi::c_void,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Speakers: EndpointFormFactor = 1i32;
pub const Subunit: PartType = 1i32;
pub const UnknownDigitalPassthrough: EndpointFormFactor = 7i32;
pub const UnknownFormFactor: EndpointFormFactor = 10i32;
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: windows_sys::core::PCWSTR = windows_sys::core::w!("VAD\\Process_Loopback");
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
    pub SubFormat: windows_sys::core::GUID,
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
    pub lpData: windows_sys::core::PSTR,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
    pub ManufacturerGuid: windows_sys::core::GUID,
    pub ProductGuid: windows_sys::core::GUID,
    pub NameGuid: windows_sys::core::GUID,
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
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = 2u32;
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
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = 8u32;
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = 9u32;
pub const WAVE_FORMAT_PCM: u32 = 1u32;
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = 1u32;
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = 4u32;
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = 16u32;
pub const WAVE_MAPPER: u32 = 4294967295u32;
pub const WHDR_BEGINLOOP: u32 = 4u32;
pub const WHDR_DONE: u32 = 1u32;
pub const WHDR_ENDLOOP: u32 = 8u32;
pub const WHDR_INQUEUE: u32 = 16u32;
pub const WHDR_PREPARED: u32 = 2u32;
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
pub type _AUDCLNT_BUFFERFLAGS = i32;
pub const eAll: EDataFlow = 2i32;
pub const eCapture: EDataFlow = 1i32;
pub const eCommunications: ERole = 2i32;
pub const eConsole: ERole = 0i32;
pub const eMultimedia: ERole = 1i32;
pub const eRender: EDataFlow = 0i32;
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
