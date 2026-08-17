#[inline]
pub unsafe fn ImmAssociateContext<P0, P1>(param0: P0, param1: P1) -> HIMC
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmAssociateContext(param0 : super::super::super::Foundation:: HWND, param1 : HIMC) -> HIMC);
    ImmAssociateContext(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn ImmAssociateContextEx<P0, P1>(param0: P0, param1: P1, param2: u32) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmAssociateContextEx(param0 : super::super::super::Foundation:: HWND, param1 : HIMC, param2 : u32) -> super::super::super::Foundation:: BOOL);
    ImmAssociateContextEx(param0.param().abi(), param1.param().abi(), param2)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmConfigureIMEA<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmConfigureIMEA(param0 : super::KeyboardAndMouse:: HKL, param1 : super::super::super::Foundation:: HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> super::super::super::Foundation:: BOOL);
    ImmConfigureIMEA(param0.param().abi(), param1.param().abi(), param2, param3)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmConfigureIMEW<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmConfigureIMEW(param0 : super::KeyboardAndMouse:: HKL, param1 : super::super::super::Foundation:: HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> super::super::super::Foundation:: BOOL);
    ImmConfigureIMEW(param0.param().abi(), param1.param().abi(), param2, param3)
}
#[inline]
pub unsafe fn ImmCreateContext() -> HIMC {
    windows_targets::link!("imm32.dll" "system" fn ImmCreateContext() -> HIMC);
    ImmCreateContext()
}
#[inline]
pub unsafe fn ImmCreateIMCC(param0: u32) -> HIMCC {
    windows_targets::link!("imm32.dll" "system" fn ImmCreateIMCC(param0 : u32) -> HIMCC);
    ImmCreateIMCC(param0)
}
#[inline]
pub unsafe fn ImmCreateSoftKeyboard<P0>(param0: u32, param1: P0, param2: i32, param3: i32) -> super::super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmCreateSoftKeyboard(param0 : u32, param1 : super::super::super::Foundation:: HWND, param2 : i32, param3 : i32) -> super::super::super::Foundation:: HWND);
    ImmCreateSoftKeyboard(param0, param1.param().abi(), param2, param3)
}
#[inline]
pub unsafe fn ImmDestroyContext<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmDestroyContext(param0 : HIMC) -> super::super::super::Foundation:: BOOL);
    ImmDestroyContext(param0.param().abi())
}
#[inline]
pub unsafe fn ImmDestroyIMCC<P0>(param0: P0) -> HIMCC
where
    P0: windows_core::Param<HIMCC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmDestroyIMCC(param0 : HIMCC) -> HIMCC);
    ImmDestroyIMCC(param0.param().abi())
}
#[inline]
pub unsafe fn ImmDestroySoftKeyboard<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmDestroySoftKeyboard(param0 : super::super::super::Foundation:: HWND) -> super::super::super::Foundation:: BOOL);
    ImmDestroySoftKeyboard(param0.param().abi())
}
#[inline]
pub unsafe fn ImmDisableIME(param0: u32) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("imm32.dll" "system" fn ImmDisableIME(param0 : u32) -> super::super::super::Foundation:: BOOL);
    ImmDisableIME(param0)
}
#[inline]
pub unsafe fn ImmDisableLegacyIME() -> super::super::super::Foundation::BOOL {
    windows_targets::link!("imm32.dll" "system" fn ImmDisableLegacyIME() -> super::super::super::Foundation:: BOOL);
    ImmDisableLegacyIME()
}
#[inline]
pub unsafe fn ImmDisableTextFrameService(idthread: u32) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("imm32.dll" "system" fn ImmDisableTextFrameService(idthread : u32) -> super::super::super::Foundation:: BOOL);
    ImmDisableTextFrameService(idthread)
}
#[inline]
pub unsafe fn ImmEnumInputContext<P0>(idthread: u32, lpfn: IMCENUMPROC, lparam: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::LPARAM>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmEnumInputContext(idthread : u32, lpfn : IMCENUMPROC, lparam : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: BOOL);
    ImmEnumInputContext(idthread, lpfn, lparam.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<P0, P1, P2>(param0: P0, param1: REGISTERWORDENUMPROCA, lpszreading: P1, param3: u32, lpszregister: P2, param5: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmEnumRegisterWordA(param0 : super::KeyboardAndMouse:: HKL, param1 : REGISTERWORDENUMPROCA, lpszreading : windows_core::PCSTR, param3 : u32, lpszregister : windows_core::PCSTR, param5 : *mut core::ffi::c_void) -> u32);
    ImmEnumRegisterWordA(param0.param().abi(), param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<P0, P1, P2>(param0: P0, param1: REGISTERWORDENUMPROCW, lpszreading: P1, param3: u32, lpszregister: P2, param5: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmEnumRegisterWordW(param0 : super::KeyboardAndMouse:: HKL, param1 : REGISTERWORDENUMPROCW, lpszreading : windows_core::PCWSTR, param3 : u32, lpszregister : windows_core::PCWSTR, param5 : *mut core::ffi::c_void) -> u32);
    ImmEnumRegisterWordW(param0.param().abi(), param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEscapeA<P0, P1>(param0: P0, param1: P1, param2: IME_ESCAPE, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmEscapeA(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, param2 : IME_ESCAPE, param3 : *mut core::ffi::c_void) -> super::super::super::Foundation:: LRESULT);
    ImmEscapeA(param0.param().abi(), param1.param().abi(), param2, param3)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEscapeW<P0, P1>(param0: P0, param1: P1, param2: IME_ESCAPE, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::LRESULT
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmEscapeW(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, param2 : IME_ESCAPE, param3 : *mut core::ffi::c_void) -> super::super::super::Foundation:: LRESULT);
    ImmEscapeW(param0.param().abi(), param1.param().abi(), param2, param3)
}
#[inline]
pub unsafe fn ImmGenerateMessage<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGenerateMessage(param0 : HIMC) -> super::super::super::Foundation:: BOOL);
    ImmGenerateMessage(param0.param().abi())
}
#[inline]
pub unsafe fn ImmGetCandidateListA<P0>(param0: P0, deindex: u32, lpcandlist: Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListA(param0 : HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    ImmGetCandidateListA(param0.param().abi(), deindex, core::mem::transmute(lpcandlist.unwrap_or(std::ptr::null_mut())), dwbuflen)
}
#[inline]
pub unsafe fn ImmGetCandidateListCountA<P0>(param0: P0, lpdwlistcount: *mut u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListCountA(param0 : HIMC, lpdwlistcount : *mut u32) -> u32);
    ImmGetCandidateListCountA(param0.param().abi(), lpdwlistcount)
}
#[inline]
pub unsafe fn ImmGetCandidateListCountW<P0>(param0: P0, lpdwlistcount: *mut u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListCountW(param0 : HIMC, lpdwlistcount : *mut u32) -> u32);
    ImmGetCandidateListCountW(param0.param().abi(), lpdwlistcount)
}
#[inline]
pub unsafe fn ImmGetCandidateListW<P0>(param0: P0, deindex: u32, lpcandlist: Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateListW(param0 : HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    ImmGetCandidateListW(param0.param().abi(), deindex, core::mem::transmute(lpcandlist.unwrap_or(std::ptr::null_mut())), dwbuflen)
}
#[inline]
pub unsafe fn ImmGetCandidateWindow<P0>(param0: P0, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCandidateWindow(param0 : HIMC, param1 : u32, lpcandidate : *mut CANDIDATEFORM) -> super::super::super::Foundation:: BOOL);
    ImmGetCandidateWindow(param0.param().abi(), param1, lpcandidate)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontA<P0>(param0: P0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionFontA(param0 : HIMC, lplf : *mut super::super::super::Graphics::Gdi:: LOGFONTA) -> super::super::super::Foundation:: BOOL);
    ImmGetCompositionFontA(param0.param().abi(), lplf)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontW<P0>(param0: P0, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionFontW(param0 : HIMC, lplf : *mut super::super::super::Graphics::Gdi:: LOGFONTW) -> super::super::super::Foundation:: BOOL);
    ImmGetCompositionFontW(param0.param().abi(), lplf)
}
#[inline]
pub unsafe fn ImmGetCompositionStringA<P0>(param0: P0, param1: IME_COMPOSITION_STRING, lpbuf: Option<*mut core::ffi::c_void>, dwbuflen: u32) -> i32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionStringA(param0 : HIMC, param1 : IME_COMPOSITION_STRING, lpbuf : *mut core::ffi::c_void, dwbuflen : u32) -> i32);
    ImmGetCompositionStringA(param0.param().abi(), param1, core::mem::transmute(lpbuf.unwrap_or(std::ptr::null_mut())), dwbuflen)
}
#[inline]
pub unsafe fn ImmGetCompositionStringW<P0>(param0: P0, param1: IME_COMPOSITION_STRING, lpbuf: Option<*mut core::ffi::c_void>, dwbuflen: u32) -> i32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionStringW(param0 : HIMC, param1 : IME_COMPOSITION_STRING, lpbuf : *mut core::ffi::c_void, dwbuflen : u32) -> i32);
    ImmGetCompositionStringW(param0.param().abi(), param1, core::mem::transmute(lpbuf.unwrap_or(std::ptr::null_mut())), dwbuflen)
}
#[inline]
pub unsafe fn ImmGetCompositionWindow<P0>(param0: P0, lpcompform: *mut COMPOSITIONFORM) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetCompositionWindow(param0 : HIMC, lpcompform : *mut COMPOSITIONFORM) -> super::super::super::Foundation:: BOOL);
    ImmGetCompositionWindow(param0.param().abi(), lpcompform)
}
#[inline]
pub unsafe fn ImmGetContext<P0>(param0: P0) -> HIMC
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetContext(param0 : super::super::super::Foundation:: HWND) -> HIMC);
    ImmGetContext(param0.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetConversionListA<P0, P1, P2>(param0: P0, param1: P1, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<HIMC>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetConversionListA(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, lpsrc : windows_core::PCSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : GET_CONVERSION_LIST_FLAG) -> u32);
    ImmGetConversionListA(param0.param().abi(), param1.param().abi(), lpsrc.param().abi(), lpdst, dwbuflen, uflag)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetConversionListW<P0, P1, P2>(param0: P0, param1: P1, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<HIMC>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetConversionListW(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, lpsrc : windows_core::PCWSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : GET_CONVERSION_LIST_FLAG) -> u32);
    ImmGetConversionListW(param0.param().abi(), param1.param().abi(), lpsrc.param().abi(), lpdst, dwbuflen, uflag)
}
#[inline]
pub unsafe fn ImmGetConversionStatus<P0>(param0: P0, lpfdwconversion: Option<*mut IME_CONVERSION_MODE>, lpfdwsentence: Option<*mut IME_SENTENCE_MODE>) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetConversionStatus(param0 : HIMC, lpfdwconversion : *mut IME_CONVERSION_MODE, lpfdwsentence : *mut IME_SENTENCE_MODE) -> super::super::super::Foundation:: BOOL);
    ImmGetConversionStatus(param0.param().abi(), core::mem::transmute(lpfdwconversion.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpfdwsentence.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd<P0>(param0: P0) -> super::super::super::Foundation::HWND
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetDefaultIMEWnd(param0 : super::super::super::Foundation:: HWND) -> super::super::super::Foundation:: HWND);
    ImmGetDefaultIMEWnd(param0.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetDescriptionA<P0>(param0: P0, lpszdescription: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetDescriptionA(param0 : super::KeyboardAndMouse:: HKL, lpszdescription : windows_core::PSTR, ubuflen : u32) -> u32);
    ImmGetDescriptionA(param0.param().abi(), core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetDescriptionW<P0>(param0: P0, lpszdescription: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetDescriptionW(param0 : super::KeyboardAndMouse:: HKL, lpszdescription : windows_core::PWSTR, ubuflen : u32) -> u32);
    ImmGetDescriptionW(param0.param().abi(), core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn ImmGetGuideLineA<P0>(param0: P0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetGuideLineA(param0 : HIMC, dwindex : GET_GUIDE_LINE_TYPE, lpbuf : windows_core::PSTR, dwbuflen : u32) -> u32);
    ImmGetGuideLineA(param0.param().abi(), dwindex, core::mem::transmute(lpbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn ImmGetGuideLineW<P0>(param0: P0, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: windows_core::PWSTR, dwbuflen: u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetGuideLineW(param0 : HIMC, dwindex : GET_GUIDE_LINE_TYPE, lpbuf : windows_core::PWSTR, dwbuflen : u32) -> u32);
    ImmGetGuideLineW(param0.param().abi(), dwindex, core::mem::transmute(lpbuf), dwbuflen)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut super::KeyboardAndMouse::HKL) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("imm32.dll" "system" fn ImmGetHotKey(param0 : u32, lpumodifiers : *mut u32, lpuvkey : *mut u32, phkl : *mut super::KeyboardAndMouse:: HKL) -> super::super::super::Foundation:: BOOL);
    ImmGetHotKey(param0, lpumodifiers, lpuvkey, phkl)
}
#[inline]
pub unsafe fn ImmGetIMCCLockCount<P0>(param0: P0) -> u32
where
    P0: windows_core::Param<HIMCC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetIMCCLockCount(param0 : HIMCC) -> u32);
    ImmGetIMCCLockCount(param0.param().abi())
}
#[inline]
pub unsafe fn ImmGetIMCCSize<P0>(param0: P0) -> u32
where
    P0: windows_core::Param<HIMCC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetIMCCSize(param0 : HIMCC) -> u32);
    ImmGetIMCCSize(param0.param().abi())
}
#[inline]
pub unsafe fn ImmGetIMCLockCount<P0>(param0: P0) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetIMCLockCount(param0 : HIMC) -> u32);
    ImmGetIMCLockCount(param0.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetIMEFileNameA<P0>(param0: P0, lpszfilename: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetIMEFileNameA(param0 : super::KeyboardAndMouse:: HKL, lpszfilename : windows_core::PSTR, ubuflen : u32) -> u32);
    ImmGetIMEFileNameA(param0.param().abi(), core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetIMEFileNameW<P0>(param0: P0, lpszfilename: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetIMEFileNameW(param0 : super::KeyboardAndMouse:: HKL, lpszfilename : windows_core::PWSTR, ubuflen : u32) -> u32);
    ImmGetIMEFileNameW(param0.param().abi(), core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetImeMenuItemsA<P0>(param0: P0, param1: u32, param2: u32, lpimeparentmenu: Option<*mut IMEMENUITEMINFOA>, lpimemenu: Option<*mut IMEMENUITEMINFOA>, dwsize: u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetImeMenuItemsA(param0 : HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOA, lpimemenu : *mut IMEMENUITEMINFOA, dwsize : u32) -> u32);
    ImmGetImeMenuItemsA(param0.param().abi(), param1, param2, core::mem::transmute(lpimeparentmenu.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpimemenu.unwrap_or(std::ptr::null_mut())), dwsize)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetImeMenuItemsW<P0>(param0: P0, param1: u32, param2: u32, lpimeparentmenu: Option<*mut IMEMENUITEMINFOW>, lpimemenu: Option<*mut IMEMENUITEMINFOW>, dwsize: u32) -> u32
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetImeMenuItemsW(param0 : HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOW, lpimemenu : *mut IMEMENUITEMINFOW, dwsize : u32) -> u32);
    ImmGetImeMenuItemsW(param0.param().abi(), param1, param2, core::mem::transmute(lpimeparentmenu.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpimemenu.unwrap_or(std::ptr::null_mut())), dwsize)
}
#[inline]
pub unsafe fn ImmGetOpenStatus<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetOpenStatus(param0 : HIMC) -> super::super::super::Foundation:: BOOL);
    ImmGetOpenStatus(param0.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetProperty<P0>(param0: P0, param1: u32) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetProperty(param0 : super::KeyboardAndMouse:: HKL, param1 : u32) -> u32);
    ImmGetProperty(param0.param().abi(), param1)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA<P0>(param0: P0, lpstylebuf: &mut [STYLEBUFA]) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleA(param0 : super::KeyboardAndMouse:: HKL, nitem : u32, lpstylebuf : *mut STYLEBUFA) -> u32);
    ImmGetRegisterWordStyleA(param0.param().abi(), lpstylebuf.len().try_into().unwrap(), core::mem::transmute(lpstylebuf.as_ptr()))
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW<P0>(param0: P0, lpstylebuf: &mut [STYLEBUFW]) -> u32
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleW(param0 : super::KeyboardAndMouse:: HKL, nitem : u32, lpstylebuf : *mut STYLEBUFW) -> u32);
    ImmGetRegisterWordStyleW(param0.param().abi(), lpstylebuf.len().try_into().unwrap(), core::mem::transmute(lpstylebuf.as_ptr()))
}
#[inline]
pub unsafe fn ImmGetStatusWindowPos<P0>(param0: P0, lpptpos: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetStatusWindowPos(param0 : HIMC, lpptpos : *mut super::super::super::Foundation:: POINT) -> super::super::super::Foundation:: BOOL);
    ImmGetStatusWindowPos(param0.param().abi(), lpptpos)
}
#[inline]
pub unsafe fn ImmGetVirtualKey<P0>(param0: P0) -> u32
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmGetVirtualKey(param0 : super::super::super::Foundation:: HWND) -> u32);
    ImmGetVirtualKey(param0.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmInstallIMEA<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::KeyboardAndMouse::HKL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmInstallIMEA(lpszimefilename : windows_core::PCSTR, lpszlayouttext : windows_core::PCSTR) -> super::KeyboardAndMouse:: HKL);
    ImmInstallIMEA(lpszimefilename.param().abi(), lpszlayouttext.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmInstallIMEW<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::KeyboardAndMouse::HKL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmInstallIMEW(lpszimefilename : windows_core::PCWSTR, lpszlayouttext : windows_core::PCWSTR) -> super::KeyboardAndMouse:: HKL);
    ImmInstallIMEW(lpszimefilename.param().abi(), lpszlayouttext.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmIsIME<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmIsIME(param0 : super::KeyboardAndMouse:: HKL) -> super::super::super::Foundation:: BOOL);
    ImmIsIME(param0.param().abi())
}
#[inline]
pub unsafe fn ImmIsUIMessageA<P0, P1, P2>(param0: P0, param1: u32, param2: P1, param3: P2) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmIsUIMessageA(param0 : super::super::super::Foundation:: HWND, param1 : u32, param2 : super::super::super::Foundation:: WPARAM, param3 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: BOOL);
    ImmIsUIMessageA(param0.param().abi(), param1, param2.param().abi(), param3.param().abi())
}
#[inline]
pub unsafe fn ImmIsUIMessageW<P0, P1, P2>(param0: P0, param1: u32, param2: P1, param3: P2) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmIsUIMessageW(param0 : super::super::super::Foundation:: HWND, param1 : u32, param2 : super::super::super::Foundation:: WPARAM, param3 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: BOOL);
    ImmIsUIMessageW(param0.param().abi(), param1, param2.param().abi(), param3.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmLockIMC<P0>(param0: P0) -> *mut INPUTCONTEXT
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmLockIMC(param0 : HIMC) -> *mut INPUTCONTEXT);
    ImmLockIMC(param0.param().abi())
}
#[inline]
pub unsafe fn ImmLockIMCC<P0>(param0: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<HIMCC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmLockIMCC(param0 : HIMCC) -> *mut core::ffi::c_void);
    ImmLockIMCC(param0.param().abi())
}
#[inline]
pub unsafe fn ImmNotifyIME<P0>(param0: P0, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmNotifyIME(param0 : HIMC, dwaction : NOTIFY_IME_ACTION, dwindex : NOTIFY_IME_INDEX, dwvalue : u32) -> super::super::super::Foundation:: BOOL);
    ImmNotifyIME(param0.param().abi(), dwaction, dwindex, dwvalue)
}
#[inline]
pub unsafe fn ImmReSizeIMCC<P0>(param0: P0, param1: u32) -> HIMCC
where
    P0: windows_core::Param<HIMCC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmReSizeIMCC(param0 : HIMCC, param1 : u32) -> HIMCC);
    ImmReSizeIMCC(param0.param().abi(), param1)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmRegisterWordA<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmRegisterWordA(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszregister : windows_core::PCSTR) -> super::super::super::Foundation:: BOOL);
    ImmRegisterWordA(param0.param().abi(), lpszreading.param().abi(), param2, lpszregister.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmRegisterWordW<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmRegisterWordW(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszregister : windows_core::PCWSTR) -> super::super::super::Foundation:: BOOL);
    ImmRegisterWordW(param0.param().abi(), lpszreading.param().abi(), param2, lpszregister.param().abi())
}
#[inline]
pub unsafe fn ImmReleaseContext<P0, P1>(param0: P0, param1: P1) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
    P1: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmReleaseContext(param0 : super::super::super::Foundation:: HWND, param1 : HIMC) -> super::super::super::Foundation:: BOOL);
    ImmReleaseContext(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn ImmRequestMessageA<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> super::super::super::Foundation::LRESULT
where
    P0: windows_core::Param<HIMC>,
    P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmRequestMessageA(param0 : HIMC, param1 : super::super::super::Foundation:: WPARAM, param2 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: LRESULT);
    ImmRequestMessageA(param0.param().abi(), param1.param().abi(), param2.param().abi())
}
#[inline]
pub unsafe fn ImmRequestMessageW<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> super::super::super::Foundation::LRESULT
where
    P0: windows_core::Param<HIMC>,
    P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
    P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmRequestMessageW(param0 : HIMC, param1 : super::super::super::Foundation:: WPARAM, param2 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: LRESULT);
    ImmRequestMessageW(param0.param().abi(), param1.param().abi(), param2.param().abi())
}
#[inline]
pub unsafe fn ImmSetCandidateWindow<P0>(param0: P0, lpcandidate: *const CANDIDATEFORM) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetCandidateWindow(param0 : HIMC, lpcandidate : *const CANDIDATEFORM) -> super::super::super::Foundation:: BOOL);
    ImmSetCandidateWindow(param0.param().abi(), lpcandidate)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontA<P0>(param0: P0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionFontA(param0 : HIMC, lplf : *const super::super::super::Graphics::Gdi:: LOGFONTA) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionFontA(param0.param().abi(), lplf)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontW<P0>(param0: P0, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionFontW(param0 : HIMC, lplf : *const super::super::super::Graphics::Gdi:: LOGFONTW) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionFontW(param0.param().abi(), lplf)
}
#[inline]
pub unsafe fn ImmSetCompositionStringA<P0>(param0: P0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: Option<*const core::ffi::c_void>, dwcomplen: u32, lpread: Option<*const core::ffi::c_void>, dwreadlen: u32) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionStringA(param0 : HIMC, dwindex : SET_COMPOSITION_STRING_TYPE, lpcomp : *const core::ffi::c_void, dwcomplen : u32, lpread : *const core::ffi::c_void, dwreadlen : u32) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionStringA(param0.param().abi(), dwindex, core::mem::transmute(lpcomp.unwrap_or(std::ptr::null())), dwcomplen, core::mem::transmute(lpread.unwrap_or(std::ptr::null())), dwreadlen)
}
#[inline]
pub unsafe fn ImmSetCompositionStringW<P0>(param0: P0, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: Option<*const core::ffi::c_void>, dwcomplen: u32, lpread: Option<*const core::ffi::c_void>, dwreadlen: u32) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionStringW(param0 : HIMC, dwindex : SET_COMPOSITION_STRING_TYPE, lpcomp : *const core::ffi::c_void, dwcomplen : u32, lpread : *const core::ffi::c_void, dwreadlen : u32) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionStringW(param0.param().abi(), dwindex, core::mem::transmute(lpcomp.unwrap_or(std::ptr::null())), dwcomplen, core::mem::transmute(lpread.unwrap_or(std::ptr::null())), dwreadlen)
}
#[inline]
pub unsafe fn ImmSetCompositionWindow<P0>(param0: P0, lpcompform: *const COMPOSITIONFORM) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetCompositionWindow(param0 : HIMC, lpcompform : *const COMPOSITIONFORM) -> super::super::super::Foundation:: BOOL);
    ImmSetCompositionWindow(param0.param().abi(), lpcompform)
}
#[inline]
pub unsafe fn ImmSetConversionStatus<P0>(param0: P0, param1: IME_CONVERSION_MODE, param2: IME_SENTENCE_MODE) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetConversionStatus(param0 : HIMC, param1 : IME_CONVERSION_MODE, param2 : IME_SENTENCE_MODE) -> super::super::super::Foundation:: BOOL);
    ImmSetConversionStatus(param0.param().abi(), param1, param2)
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmSetHotKey<P0>(param0: u32, param1: u32, param2: u32, param3: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetHotKey(param0 : u32, param1 : u32, param2 : u32, param3 : super::KeyboardAndMouse:: HKL) -> super::super::super::Foundation:: BOOL);
    ImmSetHotKey(param0, param1, param2, param3.param().abi())
}
#[inline]
pub unsafe fn ImmSetOpenStatus<P0, P1>(param0: P0, param1: P1) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
    P1: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetOpenStatus(param0 : HIMC, param1 : super::super::super::Foundation:: BOOL) -> super::super::super::Foundation:: BOOL);
    ImmSetOpenStatus(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn ImmSetStatusWindowPos<P0>(param0: P0, lpptpos: *const super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSetStatusWindowPos(param0 : HIMC, lpptpos : *const super::super::super::Foundation:: POINT) -> super::super::super::Foundation:: BOOL);
    ImmSetStatusWindowPos(param0.param().abi(), lpptpos)
}
#[inline]
pub unsafe fn ImmShowSoftKeyboard<P0>(param0: P0, param1: i32) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmShowSoftKeyboard(param0 : super::super::super::Foundation:: HWND, param1 : i32) -> super::super::super::Foundation:: BOOL);
    ImmShowSoftKeyboard(param0.param().abi(), param1)
}
#[inline]
pub unsafe fn ImmSimulateHotKey<P0>(param0: P0, param1: IME_HOTKEY_IDENTIFIER) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmSimulateHotKey(param0 : super::super::super::Foundation:: HWND, param1 : IME_HOTKEY_IDENTIFIER) -> super::super::super::Foundation:: BOOL);
    ImmSimulateHotKey(param0.param().abi(), param1)
}
#[inline]
pub unsafe fn ImmUnlockIMC<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmUnlockIMC(param0 : HIMC) -> super::super::super::Foundation:: BOOL);
    ImmUnlockIMC(param0.param().abi())
}
#[inline]
pub unsafe fn ImmUnlockIMCC<P0>(param0: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<HIMCC>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmUnlockIMCC(param0 : HIMCC) -> super::super::super::Foundation:: BOOL);
    ImmUnlockIMCC(param0.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmUnregisterWordA<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszunregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmUnregisterWordA(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszunregister : windows_core::PCSTR) -> super::super::super::Foundation:: BOOL);
    ImmUnregisterWordA(param0.param().abi(), lpszreading.param().abi(), param2, lpszunregister.param().abi())
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmUnregisterWordW<P0, P1, P2>(param0: P0, lpszreading: P1, param2: u32, lpszunregister: P2) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("imm32.dll" "system" fn ImmUnregisterWordW(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszunregister : windows_core::PCWSTR) -> super::super::super::Foundation:: BOOL);
    ImmUnregisterWordW(param0.param().abi(), lpszreading.param().abi(), param2, lpszunregister.param().abi())
}
windows_core::imp::define_interface!(IActiveIME, IActiveIME_Vtbl, 0x6fe20962_d077_11d0_8fe7_00aa006bcc59);
impl core::ops::Deref for IActiveIME {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveIME, windows_core::IUnknown);
impl IActiveIME {
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: windows_core::PWSTR, pdwprivate: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Inquire)(windows_core::Interface::as_raw(self), dwsysteminfoflags, pimeinfo, core::mem::transmute(szwndclass), pdwprivate).ok()
    }
    pub unsafe fn ConversionList<P0, P1>(&self, himc: P0, szsource: P1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ConversionList)(windows_core::Interface::as_raw(self), himc.param().abi(), szsource.param().abi(), uflag, ubuflen, pdest, pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn Configure<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pregisterword: *const REGISTERWORDW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), hkl.param().abi(), hwnd.param().abi(), dwmode, pregisterword).ok()
    }
    pub unsafe fn Destroy(&self, ureserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self), ureserved).ok()
    }
    pub unsafe fn Escape<P0>(&self, himc: P0, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), himc.param().abi(), uescape, pdata, plresult).ok()
    }
    pub unsafe fn SetActiveContext<P0, P1>(&self, himc: P0, fflag: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetActiveContext)(windows_core::Interface::as_raw(self), himc.param().abi(), fflag.param().abi()).ok()
    }
    pub unsafe fn ProcessKey<P0>(&self, himc: P0, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).ProcessKey)(windows_core::Interface::as_raw(self), himc.param().abi(), uvirkey, lparam, pbkeystate).ok()
    }
    pub unsafe fn Notify<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), himc.param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    pub unsafe fn Select<P0, P1>(&self, himc: P0, fselect: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), himc.param().abi(), fselect.param().abi()).ok()
    }
    pub unsafe fn SetCompositionString<P0>(&self, himc: P0, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionString)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    pub unsafe fn ToAsciiEx<P0>(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: P0, pdwtransbuf: *mut u32, pusize: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).ToAsciiEx)(windows_core::Interface::as_raw(self), uvirkey, uscancode, pbkeystate, fustate, himc.param().abi(), pdwtransbuf, pusize).ok()
    }
    pub unsafe fn RegisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szstring: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterWord)(windows_core::Interface::as_raw(self), szreading.param().abi(), dwstyle, szstring.param().abi()).ok()
    }
    pub unsafe fn UnregisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szstring: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterWord)(windows_core::Interface::as_raw(self), szreading.param().abi(), dwstyle, szstring.param().abi()).ok()
    }
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRegisterWordStyle)(windows_core::Interface::as_raw(self), nitem, pstylebuf, pubufsize).ok()
    }
    pub unsafe fn EnumRegisterWord<P0, P1>(&self, szreading: P0, dwstyle: u32, szregister: P1, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRegisterWord)(windows_core::Interface::as_raw(self), szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCodePageA(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodePageA)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLangId(&self) -> windows_core::Result<u16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IActiveIME_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Inquire: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut IMEINFO, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ConversionList: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, windows_core::PCWSTR, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub Configure: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, super::super::super::Foundation::HWND, u32, *const REGISTERWORDW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    Configure: usize,
    pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *mut core::ffi::c_void, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub SetActiveContext: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ProcessKey: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *const u8) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, u32) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetCompositionString: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ToAsciiEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRegisterWordStyle: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STYLEBUFW, *mut u32) -> windows_core::HRESULT,
    pub EnumRegisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCodePageA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLangId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveIME2, IActiveIME2_Vtbl, 0xe1c4bf0e_2d53_11d2_93e1_0060b067b86e);
impl core::ops::Deref for IActiveIME2 {
    type Target = IActiveIME;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveIME2, windows_core::IUnknown, IActiveIME);
impl IActiveIME2 {
    pub unsafe fn Sleep(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Sleep)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unsleep<P0>(&self, fdead: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Unsleep)(windows_core::Interface::as_raw(self), fdead.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IActiveIME2_Vtbl {
    pub base__: IActiveIME_Vtbl,
    pub Sleep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unsleep: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveIMMApp, IActiveIMMApp_Vtbl, 0x08c0e040_62d1_11d1_9326_0060b067b86e);
impl core::ops::Deref for IActiveIMMApp {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveIMMApp, windows_core::IUnknown);
impl IActiveIMMApp {
    pub unsafe fn AssociateContext<P0, P1>(&self, hwnd: P0, hime: P1) -> windows_core::Result<HIMC>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<HIMC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AssociateContext)(windows_core::Interface::as_raw(self), hwnd.param().abi(), hime.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEA<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ConfigureIMEA)(windows_core::Interface::as_raw(self), hkl.param().abi(), hwnd.param().abi(), dwmode, pdata).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEW<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ConfigureIMEW)(windows_core::Interface::as_raw(self), hkl.param().abi(), hwnd.param().abi(), dwmode, pdata).ok()
    }
    pub unsafe fn CreateContext(&self) -> windows_core::Result<HIMC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DestroyContext<P0>(&self, hime: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).DestroyContext)(windows_core::Interface::as_raw(self), hime.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordA>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRegisterWordA)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRegisterWordW)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeA<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).EscapeA)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), uescape, pdata, plresult).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeW<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).EscapeW)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), uescape, pdata, plresult).ok()
    }
    pub unsafe fn GetCandidateListA<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    pub unsafe fn GetCandidateListW<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    pub unsafe fn GetCandidateListCountA<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListCountA)(windows_core::Interface::as_raw(self), himc.param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    pub unsafe fn GetCandidateListCountW<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListCountW)(windows_core::Interface::as_raw(self), himc.param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    pub unsafe fn GetCandidateWindow<P0>(&self, himc: P0, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcandidate).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontA<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionFontA)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontW<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionFontW)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    pub unsafe fn GetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionStringA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    pub unsafe fn GetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionStringW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    pub unsafe fn GetCompositionWindow<P0>(&self, himc: P0, pcompform: *mut COMPOSITIONFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), pcompform).ok()
    }
    pub unsafe fn GetContext<P0>(&self, hwnd: P0) -> windows_core::Result<HIMC>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListA<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConversionListA)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), psrc.param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListW<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetConversionListW)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), psrc.param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    pub unsafe fn GetConversionStatus<P0>(&self, himc: P0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetConversionStatus)(windows_core::Interface::as_raw(self), himc.param().abi(), pfdwconversion, pfdwsentence).ok()
    }
    pub unsafe fn GetDefaultIMEWnd<P0>(&self, hwnd: P0) -> windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultIMEWnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionA<P0>(&self, hkl: P0, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetDescriptionA)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szdescription), pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionW<P0>(&self, hkl: P0, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetDescriptionW)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szdescription), pucopied).ok()
    }
    pub unsafe fn GetGuideLineA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetGuideLineA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult).ok()
    }
    pub unsafe fn GetGuideLineW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetGuideLineW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameA<P0>(&self, hkl: P0, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetIMEFileNameA)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szfilename), pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameW<P0>(&self, hkl: P0, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetIMEFileNameW)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szfilename), pucopied).ok()
    }
    pub unsafe fn GetOpenStatus<P0>(&self, himc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetOpenStatus)(windows_core::Interface::as_raw(self), himc.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetProperty<P0>(&self, hkl: P0, fdwindex: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), hkl.param().abi(), fdwindex, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleA<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetRegisterWordStyleA)(windows_core::Interface::as_raw(self), hkl.param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleW<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetRegisterWordStyleW)(windows_core::Interface::as_raw(self), hkl.param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    pub unsafe fn GetStatusWindowPos<P0>(&self, himc: P0) -> windows_core::Result<super::super::super::Foundation::POINT>
    where
        P0: windows_core::Param<HIMC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatusWindowPos)(windows_core::Interface::as_raw(self), himc.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVirtualKey<P0>(&self, hwnd: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVirtualKey)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEA<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallIMEA)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEW<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallIMEW)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn IsIME<P0>(&self, hkl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).IsIME)(windows_core::Interface::as_raw(self), hkl.param().abi()).ok()
    }
    pub unsafe fn IsUIMessageA<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).IsUIMessageA)(windows_core::Interface::as_raw(self), hwndime.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn IsUIMessageW<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).IsUIMessageW)(windows_core::Interface::as_raw(self), hwndime.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn NotifyIME<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).NotifyIME)(windows_core::Interface::as_raw(self), himc.param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterWordA)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterWordW)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi()).ok()
    }
    pub unsafe fn ReleaseContext<P0, P1>(&self, hwnd: P0, himc: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).ReleaseContext)(windows_core::Interface::as_raw(self), hwnd.param().abi(), himc.param().abi()).ok()
    }
    pub unsafe fn SetCandidateWindow<P0>(&self, himc: P0, pcandidate: *const CANDIDATEFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCandidateWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), pcandidate).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontA<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionFontA)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontW<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionFontW)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    pub unsafe fn SetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionStringA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    pub unsafe fn SetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionStringW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    pub unsafe fn SetCompositionWindow<P0>(&self, himc: P0, pcompform: *const COMPOSITIONFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), pcompform).ok()
    }
    pub unsafe fn SetConversionStatus<P0>(&self, himc: P0, fdwconversion: u32, fdwsentence: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetConversionStatus)(windows_core::Interface::as_raw(self), himc.param().abi(), fdwconversion, fdwsentence).ok()
    }
    pub unsafe fn SetOpenStatus<P0, P1>(&self, himc: P0, fopen: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOpenStatus)(windows_core::Interface::as_raw(self), himc.param().abi(), fopen.param().abi()).ok()
    }
    pub unsafe fn SetStatusWindowPos<P0>(&self, himc: P0, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetStatusWindowPos)(windows_core::Interface::as_raw(self), himc.param().abi(), pptpos).ok()
    }
    pub unsafe fn SimulateHotKey<P0>(&self, hwnd: P0, dwhotkeyid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SimulateHotKey)(windows_core::Interface::as_raw(self), hwnd.param().abi(), dwhotkeyid).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterWordA)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szunregister.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterWordW)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szunregister.param().abi()).ok()
    }
    pub unsafe fn Activate<P0>(&self, frestorelayout: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), frestorelayout.param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDefWindowProc<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnDefWindowProc)(windows_core::Interface::as_raw(self), hwnd.param().abi(), msg, wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FilterClientWindows)(windows_core::Interface::as_raw(self), aaclasslist, usize).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetCodePageA<P0>(&self, hkl: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodePageA)(windows_core::Interface::as_raw(self), hkl.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetLangId<P0>(&self, hkl: P0) -> windows_core::Result<u16>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), hkl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AssociateContextEx<P0, P1>(&self, hwnd: P0, himc: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).AssociateContextEx)(windows_core::Interface::as_raw(self), hwnd.param().abi(), himc.param().abi(), dwflags).ok()
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableIME)(windows_core::Interface::as_raw(self), idthread).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsA<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetImeMenuItemsA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsW<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetImeMenuItemsW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> windows_core::Result<IEnumInputContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumInputContext)(windows_core::Interface::as_raw(self), idthread, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IActiveIMMApp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssociateContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, HIMC, *mut HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub ConfigureIMEA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, super::super::super::Foundation::HWND, u32, *const REGISTERWORDA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    ConfigureIMEA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub ConfigureIMEW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, super::super::super::Foundation::HWND, u32, *const REGISTERWORDW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    ConfigureIMEW: usize,
    pub CreateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HIMC) -> windows_core::HRESULT,
    pub DestroyContext: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EnumRegisterWordA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCSTR, u32, windows_core::PCSTR, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EnumRegisterWordA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EnumRegisterWordW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCWSTR, u32, windows_core::PCWSTR, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EnumRegisterWordW: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EscapeA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, u32, *mut core::ffi::c_void, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EscapeA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EscapeW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, u32, *mut core::ffi::c_void, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EscapeW: usize,
    pub GetCandidateListA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateListW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateListCountA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateListCountW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *mut CANDIDATEFORM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetCompositionFontA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetCompositionFontA: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetCompositionFontW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetCompositionFontW: usize,
    pub GetCompositionStringA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCompositionStringW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCompositionWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut COMPOSITIONFORM) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetConversionListA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, windows_core::PCSTR, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetConversionListA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetConversionListW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, windows_core::PCWSTR, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetConversionListW: usize,
    pub GetConversionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetDefaultIMEWnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetDescriptionA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetDescriptionA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetDescriptionW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetDescriptionW: usize,
    pub GetGuideLineA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub GetGuideLineW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetIMEFileNameA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetIMEFileNameA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetIMEFileNameW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetIMEFileNameW: usize,
    pub GetOpenStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetRegisterWordStyleA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, *mut STYLEBUFA, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetRegisterWordStyleA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetRegisterWordStyleW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, *mut STYLEBUFW, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetRegisterWordStyleW: usize,
    pub GetStatusWindowPos: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub GetVirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub InstallIMEA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    InstallIMEA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub InstallIMEW: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    InstallIMEW: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub IsIME: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    IsIME: usize,
    pub IsUIMessageA: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub IsUIMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub NotifyIME: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub RegisterWordA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCSTR, u32, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    RegisterWordA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub RegisterWordW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    RegisterWordW: usize,
    pub ReleaseContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, HIMC) -> windows_core::HRESULT,
    pub SetCandidateWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const CANDIDATEFORM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetCompositionFontA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetCompositionFontA: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetCompositionFontW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetCompositionFontW: usize,
    pub SetCompositionStringA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetCompositionStringW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetCompositionWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const COMPOSITIONFORM) -> windows_core::HRESULT,
    pub SetConversionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32) -> windows_core::HRESULT,
    pub SetOpenStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetStatusWindowPos: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const super::super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub SimulateHotKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub UnregisterWordA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCSTR, u32, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    UnregisterWordA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub UnregisterWordW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    UnregisterWordW: usize,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDefWindowProc: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub FilterClientWindows: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetCodePageA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetCodePageA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetLangId: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetLangId: usize,
    pub AssociateContextEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, HIMC, u32) -> windows_core::HRESULT,
    pub DisableIME: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetImeMenuItemsA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *const IMEMENUITEMINFOA, *mut IMEMENUITEMINFOA, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetImeMenuItemsA: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetImeMenuItemsW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *const IMEMENUITEMINFOW, *mut IMEMENUITEMINFOW, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetImeMenuItemsW: usize,
    pub EnumInputContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveIMMIME, IActiveIMMIME_Vtbl, 0x08c03411_f96b_11d0_a475_00aa006bcc59);
impl core::ops::Deref for IActiveIMMIME {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveIMMIME, windows_core::IUnknown);
impl IActiveIMMIME {
    pub unsafe fn AssociateContext<P0, P1>(&self, hwnd: P0, hime: P1) -> windows_core::Result<HIMC>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<HIMC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AssociateContext)(windows_core::Interface::as_raw(self), hwnd.param().abi(), hime.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEA<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ConfigureIMEA)(windows_core::Interface::as_raw(self), hkl.param().abi(), hwnd.param().abi(), dwmode, pdata).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEW<P0, P1>(&self, hkl: P0, hwnd: P1, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ConfigureIMEW)(windows_core::Interface::as_raw(self), hkl.param().abi(), hwnd.param().abi(), dwmode, pdata).ok()
    }
    pub unsafe fn CreateContext(&self) -> windows_core::Result<HIMC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DestroyContext<P0>(&self, hime: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).DestroyContext)(windows_core::Interface::as_raw(self), hime.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordA>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRegisterWordA)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRegisterWordW)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeA<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).EscapeA)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), uescape, pdata, plresult).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeW<P0, P1>(&self, hkl: P0, himc: P1, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).EscapeW)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), uescape, pdata, plresult).ok()
    }
    pub unsafe fn GetCandidateListA<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    pub unsafe fn GetCandidateListW<P0>(&self, himc: P0, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, ubuflen, pcandlist, pucopied).ok()
    }
    pub unsafe fn GetCandidateListCountA<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListCountA)(windows_core::Interface::as_raw(self), himc.param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    pub unsafe fn GetCandidateListCountW<P0>(&self, himc: P0, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateListCountW)(windows_core::Interface::as_raw(self), himc.param().abi(), pdwlistsize, pdwbuflen).ok()
    }
    pub unsafe fn GetCandidateWindow<P0>(&self, himc: P0, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCandidateWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcandidate).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontA<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionFontA)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontW<P0>(&self, himc: P0, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionFontW)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    pub unsafe fn GetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionStringA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    pub unsafe fn GetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionStringW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, plcopied, pbuf).ok()
    }
    pub unsafe fn GetCompositionWindow<P0>(&self, himc: P0, pcompform: *mut COMPOSITIONFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetCompositionWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), pcompform).ok()
    }
    pub unsafe fn GetContext<P0>(&self, hwnd: P0) -> windows_core::Result<HIMC>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListA<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).GetConversionListA)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), psrc.param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListW<P0, P1, P2>(&self, hkl: P0, himc: P1, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<HIMC>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetConversionListW)(windows_core::Interface::as_raw(self), hkl.param().abi(), himc.param().abi(), psrc.param().abi(), ubuflen, uflag, pdst, pucopied).ok()
    }
    pub unsafe fn GetConversionStatus<P0>(&self, himc: P0, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetConversionStatus)(windows_core::Interface::as_raw(self), himc.param().abi(), pfdwconversion, pfdwsentence).ok()
    }
    pub unsafe fn GetDefaultIMEWnd<P0>(&self, hwnd: P0) -> windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultIMEWnd)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionA<P0>(&self, hkl: P0, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetDescriptionA)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szdescription), pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionW<P0>(&self, hkl: P0, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetDescriptionW)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szdescription), pucopied).ok()
    }
    pub unsafe fn GetGuideLineA<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetGuideLineA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult).ok()
    }
    pub unsafe fn GetGuideLineW<P0>(&self, himc: P0, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetGuideLineW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameA<P0>(&self, hkl: P0, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetIMEFileNameA)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szfilename), pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameW<P0>(&self, hkl: P0, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetIMEFileNameW)(windows_core::Interface::as_raw(self), hkl.param().abi(), ubuflen, core::mem::transmute(szfilename), pucopied).ok()
    }
    pub unsafe fn GetOpenStatus<P0>(&self, himc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetOpenStatus)(windows_core::Interface::as_raw(self), himc.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetProperty<P0>(&self, hkl: P0, fdwindex: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), hkl.param().abi(), fdwindex, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleA<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetRegisterWordStyleA)(windows_core::Interface::as_raw(self), hkl.param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleW<P0>(&self, hkl: P0, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).GetRegisterWordStyleW)(windows_core::Interface::as_raw(self), hkl.param().abi(), nitem, pstylebuf, pucopied).ok()
    }
    pub unsafe fn GetStatusWindowPos<P0>(&self, himc: P0) -> windows_core::Result<super::super::super::Foundation::POINT>
    where
        P0: windows_core::Param<HIMC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatusWindowPos)(windows_core::Interface::as_raw(self), himc.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVirtualKey<P0>(&self, hwnd: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVirtualKey)(windows_core::Interface::as_raw(self), hwnd.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEA<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallIMEA)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEW<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallIMEW)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn IsIME<P0>(&self, hkl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).IsIME)(windows_core::Interface::as_raw(self), hkl.param().abi()).ok()
    }
    pub unsafe fn IsUIMessageA<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).IsUIMessageA)(windows_core::Interface::as_raw(self), hwndime.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn IsUIMessageW<P0, P1, P2>(&self, hwndime: P0, msg: u32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).IsUIMessageW)(windows_core::Interface::as_raw(self), hwndime.param().abi(), msg, wparam.param().abi(), lparam.param().abi()).ok()
    }
    pub unsafe fn NotifyIME<P0>(&self, himc: P0, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).NotifyIME)(windows_core::Interface::as_raw(self), himc.param().abi(), dwaction, dwindex, dwvalue).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterWordA)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterWordW)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szregister.param().abi()).ok()
    }
    pub unsafe fn ReleaseContext<P0, P1>(&self, hwnd: P0, himc: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).ReleaseContext)(windows_core::Interface::as_raw(self), hwnd.param().abi(), himc.param().abi()).ok()
    }
    pub unsafe fn SetCandidateWindow<P0>(&self, himc: P0, pcandidate: *const CANDIDATEFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCandidateWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), pcandidate).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontA<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionFontA)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontW<P0>(&self, himc: P0, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionFontW)(windows_core::Interface::as_raw(self), himc.param().abi(), plf).ok()
    }
    pub unsafe fn SetCompositionStringA<P0>(&self, himc: P0, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionStringA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    pub unsafe fn SetCompositionStringW<P0>(&self, himc: P0, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionStringW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwindex, pcomp, dwcomplen, pread, dwreadlen).ok()
    }
    pub unsafe fn SetCompositionWindow<P0>(&self, himc: P0, pcompform: *const COMPOSITIONFORM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetCompositionWindow)(windows_core::Interface::as_raw(self), himc.param().abi(), pcompform).ok()
    }
    pub unsafe fn SetConversionStatus<P0>(&self, himc: P0, fdwconversion: u32, fdwsentence: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetConversionStatus)(windows_core::Interface::as_raw(self), himc.param().abi(), fdwconversion, fdwsentence).ok()
    }
    pub unsafe fn SetOpenStatus<P0, P1>(&self, himc: P0, fopen: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOpenStatus)(windows_core::Interface::as_raw(self), himc.param().abi(), fopen.param().abi()).ok()
    }
    pub unsafe fn SetStatusWindowPos<P0>(&self, himc: P0, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).SetStatusWindowPos)(windows_core::Interface::as_raw(self), himc.param().abi(), pptpos).ok()
    }
    pub unsafe fn SimulateHotKey<P0>(&self, hwnd: P0, dwhotkeyid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SimulateHotKey)(windows_core::Interface::as_raw(self), hwnd.param().abi(), dwhotkeyid).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordA<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterWordA)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szunregister.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordW<P0, P1, P2>(&self, hkl: P0, szreading: P1, dwstyle: u32, szunregister: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterWordW)(windows_core::Interface::as_raw(self), hkl.param().abi(), szreading.param().abi(), dwstyle, szunregister.param().abi()).ok()
    }
    pub unsafe fn GenerateMessage<P0>(&self, himc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GenerateMessage)(windows_core::Interface::as_raw(self), himc.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn LockIMC<P0>(&self, himc: P0) -> windows_core::Result<*mut INPUTCONTEXT>
    where
        P0: windows_core::Param<HIMC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LockIMC)(windows_core::Interface::as_raw(self), himc.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn UnlockIMC<P0>(&self, himc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).UnlockIMC)(windows_core::Interface::as_raw(self), himc.param().abi()).ok()
    }
    pub unsafe fn GetIMCLockCount<P0>(&self, himc: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<HIMC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIMCLockCount)(windows_core::Interface::as_raw(self), himc.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CreateIMCC(&self, dwsize: u32) -> windows_core::Result<HIMCC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateIMCC)(windows_core::Interface::as_raw(self), dwsize, &mut result__).map(|| result__)
    }
    pub unsafe fn DestroyIMCC<P0>(&self, himcc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMCC>,
    {
        (windows_core::Interface::vtable(self).DestroyIMCC)(windows_core::Interface::as_raw(self), himcc.param().abi()).ok()
    }
    pub unsafe fn LockIMCC<P0>(&self, himcc: P0, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMCC>,
    {
        (windows_core::Interface::vtable(self).LockIMCC)(windows_core::Interface::as_raw(self), himcc.param().abi(), ppv).ok()
    }
    pub unsafe fn UnlockIMCC<P0>(&self, himcc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMCC>,
    {
        (windows_core::Interface::vtable(self).UnlockIMCC)(windows_core::Interface::as_raw(self), himcc.param().abi()).ok()
    }
    pub unsafe fn ReSizeIMCC<P0>(&self, himcc: P0, dwsize: u32) -> windows_core::Result<HIMCC>
    where
        P0: windows_core::Param<HIMCC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReSizeIMCC)(windows_core::Interface::as_raw(self), himcc.param().abi(), dwsize, &mut result__).map(|| result__)
    }
    pub unsafe fn GetIMCCSize<P0>(&self, himcc: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<HIMCC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIMCCSize)(windows_core::Interface::as_raw(self), himcc.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIMCCLockCount<P0>(&self, himcc: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<HIMCC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIMCCLockCount)(windows_core::Interface::as_raw(self), himcc.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHotKey)(windows_core::Interface::as_raw(self), dwhotkeyid, pumodifiers, puvkey, phkl).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn SetHotKey<P0>(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        (windows_core::Interface::vtable(self).SetHotKey)(windows_core::Interface::as_raw(self), dwhotkeyid, umodifiers, uvkey, hkl.param().abi()).ok()
    }
    pub unsafe fn CreateSoftKeyboard<P0>(&self, utype: u32, howner: P0, x: i32, y: i32) -> windows_core::Result<super::super::super::Foundation::HWND>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSoftKeyboard)(windows_core::Interface::as_raw(self), utype, howner.param().abi(), x, y, &mut result__).map(|| result__)
    }
    pub unsafe fn DestroySoftKeyboard<P0>(&self, hsoftkbdwnd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DestroySoftKeyboard)(windows_core::Interface::as_raw(self), hsoftkbdwnd.param().abi()).ok()
    }
    pub unsafe fn ShowSoftKeyboard<P0>(&self, hsoftkbdwnd: P0, ncmdshow: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ShowSoftKeyboard)(windows_core::Interface::as_raw(self), hsoftkbdwnd.param().abi(), ncmdshow).ok()
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetCodePageA<P0>(&self, hkl: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCodePageA)(windows_core::Interface::as_raw(self), hkl.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetLangId<P0>(&self, hkl: P0) -> windows_core::Result<u16>
    where
        P0: windows_core::Param<super::KeyboardAndMouse::HKL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), hkl.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).KeybdEvent)(windows_core::Interface::as_raw(self), lgidime, bvk, bscan, dwflags, dwextrainfo).ok()
    }
    pub unsafe fn LockModal(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockModal)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UnlockModal(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockModal)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AssociateContextEx<P0, P1>(&self, hwnd: P0, himc: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).AssociateContextEx)(windows_core::Interface::as_raw(self), hwnd.param().abi(), himc.param().abi(), dwflags).ok()
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableIME)(windows_core::Interface::as_raw(self), idthread).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsA<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetImeMenuItemsA)(windows_core::Interface::as_raw(self), himc.param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsW<P0>(&self, himc: P0, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<HIMC>,
    {
        (windows_core::Interface::vtable(self).GetImeMenuItemsW)(windows_core::Interface::as_raw(self), himc.param().abi(), dwflags, dwtype, pimeparentmenu, pimemenu, dwsize, pdwresult).ok()
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> windows_core::Result<IEnumInputContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumInputContext)(windows_core::Interface::as_raw(self), idthread, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestMessageA<P0, P1, P2>(&self, himc: P0, wparam: P1, lparam: P2) -> windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestMessageA)(windows_core::Interface::as_raw(self), himc.param().abi(), wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestMessageW<P0, P1, P2>(&self, himc: P0, wparam: P1, lparam: P2) -> windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<HIMC>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestMessageW)(windows_core::Interface::as_raw(self), himc.param().abi(), wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SendIMCA<P0, P1, P2>(&self, hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SendIMCA)(windows_core::Interface::as_raw(self), hwnd.param().abi(), umsg, wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SendIMCW<P0, P1, P2>(&self, hwnd: P0, umsg: u32, wparam: P1, lparam: P2) -> windows_core::Result<super::super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SendIMCW)(windows_core::Interface::as_raw(self), hwnd.param().abi(), umsg, wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IsSleeping(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsSleeping)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IActiveIMMIME_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AssociateContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, HIMC, *mut HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub ConfigureIMEA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, super::super::super::Foundation::HWND, u32, *const REGISTERWORDA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    ConfigureIMEA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub ConfigureIMEW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, super::super::super::Foundation::HWND, u32, *const REGISTERWORDW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    ConfigureIMEW: usize,
    pub CreateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HIMC) -> windows_core::HRESULT,
    pub DestroyContext: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EnumRegisterWordA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCSTR, u32, windows_core::PCSTR, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EnumRegisterWordA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EnumRegisterWordW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCWSTR, u32, windows_core::PCWSTR, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EnumRegisterWordW: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EscapeA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, u32, *mut core::ffi::c_void, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EscapeA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub EscapeW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, u32, *mut core::ffi::c_void, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    EscapeW: usize,
    pub GetCandidateListA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateListW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateListCountA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateListCountW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetCandidateWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *mut CANDIDATEFORM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetCompositionFontA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetCompositionFontA: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetCompositionFontW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetCompositionFontW: usize,
    pub GetCompositionStringA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCompositionStringW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *mut i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCompositionWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut COMPOSITIONFORM) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetConversionListA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, windows_core::PCSTR, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetConversionListA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetConversionListW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, HIMC, windows_core::PCWSTR, u32, u32, *mut CANDIDATELIST, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetConversionListW: usize,
    pub GetConversionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetDefaultIMEWnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetDescriptionA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetDescriptionA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetDescriptionW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetDescriptionW: usize,
    pub GetGuideLineA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    pub GetGuideLineW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetIMEFileNameA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetIMEFileNameA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetIMEFileNameW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetIMEFileNameW: usize,
    pub GetOpenStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetRegisterWordStyleA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, *mut STYLEBUFA, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetRegisterWordStyleA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetRegisterWordStyleW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, u32, *mut STYLEBUFW, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetRegisterWordStyleW: usize,
    pub GetStatusWindowPos: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub GetVirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub InstallIMEA: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::PCSTR, *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    InstallIMEA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub InstallIMEW: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    InstallIMEW: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub IsIME: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    IsIME: usize,
    pub IsUIMessageA: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub IsUIMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub NotifyIME: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub RegisterWordA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCSTR, u32, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    RegisterWordA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub RegisterWordW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    RegisterWordW: usize,
    pub ReleaseContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, HIMC) -> windows_core::HRESULT,
    pub SetCandidateWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const CANDIDATEFORM) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetCompositionFontA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetCompositionFontA: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetCompositionFontW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetCompositionFontW: usize,
    pub SetCompositionStringA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetCompositionStringW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetCompositionWindow: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const COMPOSITIONFORM) -> windows_core::HRESULT,
    pub SetConversionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32) -> windows_core::HRESULT,
    pub SetOpenStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetStatusWindowPos: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *const super::super::super::Foundation::POINT) -> windows_core::HRESULT,
    pub SimulateHotKey: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub UnregisterWordA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCSTR, u32, windows_core::PCSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    UnregisterWordA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub UnregisterWordW: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    UnregisterWordW: usize,
    pub GenerateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub LockIMC: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut *mut INPUTCONTEXT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    LockIMC: usize,
    pub UnlockIMC: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC) -> windows_core::HRESULT,
    pub GetIMCLockCount: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, *mut u32) -> windows_core::HRESULT,
    pub CreateIMCC: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut HIMCC) -> windows_core::HRESULT,
    pub DestroyIMCC: unsafe extern "system" fn(*mut core::ffi::c_void, HIMCC) -> windows_core::HRESULT,
    pub LockIMCC: unsafe extern "system" fn(*mut core::ffi::c_void, HIMCC, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockIMCC: unsafe extern "system" fn(*mut core::ffi::c_void, HIMCC) -> windows_core::HRESULT,
    pub ReSizeIMCC: unsafe extern "system" fn(*mut core::ffi::c_void, HIMCC, u32, *mut HIMCC) -> windows_core::HRESULT,
    pub GetIMCCSize: unsafe extern "system" fn(*mut core::ffi::c_void, HIMCC, *mut u32) -> windows_core::HRESULT,
    pub GetIMCCLockCount: unsafe extern "system" fn(*mut core::ffi::c_void, HIMCC, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetHotKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetHotKey: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub SetHotKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::KeyboardAndMouse::HKL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    SetHotKey: usize,
    pub CreateSoftKeyboard: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::super::Foundation::HWND, i32, i32, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub DestroySoftKeyboard: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ShowSoftKeyboard: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetCodePageA: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetCodePageA: usize,
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub GetLangId: unsafe extern "system" fn(*mut core::ffi::c_void, super::KeyboardAndMouse::HKL, *mut u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Input_KeyboardAndMouse"))]
    GetLangId: usize,
    pub KeybdEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u8, u8, u32, u32) -> windows_core::HRESULT,
    pub LockModal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockModal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AssociateContextEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, HIMC, u32) -> windows_core::HRESULT,
    pub DisableIME: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetImeMenuItemsA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *const IMEMENUITEMINFOA, *mut IMEMENUITEMINFOA, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetImeMenuItemsA: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetImeMenuItemsW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *const IMEMENUITEMINFOW, *mut IMEMENUITEMINFOW, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetImeMenuItemsW: usize,
    pub EnumInputContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestMessageA: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub RequestMessageW: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub SendIMCA: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub SendIMCW: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, u32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM, *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub IsSleeping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveIMMMessagePumpOwner, IActiveIMMMessagePumpOwner_Vtbl, 0xb5cf2cfa_8aeb_11d1_9364_0060b067b86e);
impl core::ops::Deref for IActiveIMMMessagePumpOwner {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveIMMMessagePumpOwner, windows_core::IUnknown);
impl IActiveIMMMessagePumpOwner {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn End(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTranslateMessage)(windows_core::Interface::as_raw(self), pmsg).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Resume(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
}
#[repr(C)]
pub struct IActiveIMMMessagePumpOwner_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub OnTranslateMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::WindowsAndMessaging::MSG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    OnTranslateMessage: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveIMMRegistrar, IActiveIMMRegistrar_Vtbl, 0xb3458082_bd00_11d1_939b_0060b067b86e);
impl core::ops::Deref for IActiveIMMRegistrar {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveIMMRegistrar, windows_core::IUnknown);
impl IActiveIMMRegistrar {
    pub unsafe fn RegisterIME<P0, P1>(&self, rclsid: *const windows_core::GUID, lgid: u16, psziconfile: P0, pszdesc: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterIME)(windows_core::Interface::as_raw(self), rclsid, lgid, psziconfile.param().abi(), pszdesc.param().abi()).ok()
    }
    pub unsafe fn UnregisterIME(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnregisterIME)(windows_core::Interface::as_raw(self), rclsid).ok()
    }
}
#[repr(C)]
pub struct IActiveIMMRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterIME: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterIME: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumInputContext, IEnumInputContext_Vtbl, 0x09b5eab0_f997_11d1_93d4_0060b067b86e);
impl core::ops::Deref for IEnumInputContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumInputContext, windows_core::IUnknown);
impl IEnumInputContext {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumInputContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ulcount: u32, rginputcontext: *mut HIMC, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rginputcontext, pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumInputContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut HIMC, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumRegisterWordA, IEnumRegisterWordA_Vtbl, 0x08c03412_f96b_11d0_a475_00aa006bcc59);
impl core::ops::Deref for IEnumRegisterWordA {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumRegisterWordA, windows_core::IUnknown);
impl IEnumRegisterWordA {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRegisterWordA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rgregisterword, pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumRegisterWordA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut REGISTERWORDA, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumRegisterWordW, IEnumRegisterWordW_Vtbl, 0x4955dd31_b159_11d0_8fcf_00aa006bcc59);
impl core::ops::Deref for IEnumRegisterWordW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumRegisterWordW, windows_core::IUnknown);
impl IEnumRegisterWordW {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRegisterWordW> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rgregisterword, pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok()
    }
}
#[repr(C)]
pub struct IEnumRegisterWordW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut REGISTERWORDW, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFEClassFactory, IFEClassFactory_Vtbl, 0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFEClassFactory {
    type Target = super::super::super::System::Com::IClassFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFEClassFactory, windows_core::IUnknown, super::super::super::System::Com::IClassFactory);
#[cfg(feature = "Win32_System_Com")]
impl IFEClassFactory {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFEClassFactory_Vtbl {
    pub base__: super::super::super::System::Com::IClassFactory_Vtbl,
}
windows_core::imp::define_interface!(IFECommon, IFECommon_Vtbl, 0x019f7151_e6db_11d0_83c3_00c04fddb82e);
impl core::ops::Deref for IFECommon {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFECommon, windows_core::IUnknown);
impl IFECommon {
    pub unsafe fn IsDefaultIME(&self, szname: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsDefaultIME)(windows_core::Interface::as_raw(self), core::mem::transmute(szname.as_ptr()), szname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetDefaultIME(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultIME)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvokeWordRegDialog)(windows_core::Interface::as_raw(self), pimedlg).ok()
    }
    pub unsafe fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InvokeDictToolDialog)(windows_core::Interface::as_raw(self), pimedlg).ok()
    }
}
#[repr(C)]
pub struct IFECommon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDefaultIME: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, i32) -> windows_core::HRESULT,
    pub SetDefaultIME: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeWordRegDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEDLG) -> windows_core::HRESULT,
    pub InvokeDictToolDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEDLG) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFEDictionary, IFEDictionary_Vtbl, 0x019f7153_e6db_11d0_83c3_00c04fddb82e);
impl core::ops::Deref for IFEDictionary {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFEDictionary, windows_core::IUnknown);
impl IFEDictionary {
    pub unsafe fn Open(&self, pchdictpath: Option<&mut [u8; 260]>, pshf: *mut IMESHF) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute(pchdictpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pshf).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHeader(&self, pchdictpath: Option<&mut [u8; 260]>, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHeader)(windows_core::Interface::as_raw(self), core::mem::transmute(pchdictpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pshf, pjfmt, pultype).ok()
    }
    pub unsafe fn DisplayProperty<P0>(&self, hwnd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DisplayProperty)(windows_core::Interface::as_raw(self), hwnd.param().abi()).ok()
    }
    pub unsafe fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPosTable)(windows_core::Interface::as_raw(self), prgpostbl, pcpostbl).ok()
    }
    pub unsafe fn GetWords<P0, P1, P2>(&self, pwchfirst: P0, pwchlast: P1, pwchdisplay: P2, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetWords)(windows_core::Interface::as_raw(self), pwchfirst.param().abi(), pwchlast.param().abi(), pwchdisplay.param().abi(), ulpos, ulselect, ulwordsrc, pchbuffer, cbbuffer, pcwrd).ok()
    }
    pub unsafe fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextWords)(windows_core::Interface::as_raw(self), pchbuffer, cbbuffer, pcwrd).ok()
    }
    pub unsafe fn Create<P0>(&self, pchdictpath: P0, pshf: *mut IMESHF) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pchdictpath.param().abi(), pshf).ok()
    }
    pub unsafe fn SetHeader(&self, pshf: *mut IMESHF) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHeader)(windows_core::Interface::as_raw(self), pshf).ok()
    }
    pub unsafe fn ExistWord(&self, pwrd: *mut IMEWRD) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).ExistWord)(windows_core::Interface::as_raw(self), pwrd)
    }
    pub unsafe fn ExistDependency(&self, pdp: *mut IMEDP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExistDependency)(windows_core::Interface::as_raw(self), pdp).ok()
    }
    pub unsafe fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterWord)(windows_core::Interface::as_raw(self), reg, pwrd).ok()
    }
    pub unsafe fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterDependency)(windows_core::Interface::as_raw(self), reg, pdp).ok()
    }
    pub unsafe fn GetDependencies<P0, P1, P2, P3>(&self, pwchkakarireading: P0, pwchkakaridisplay: P1, ulkakaripos: u32, pwchukereading: P2, pwchukedisplay: P3, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDependencies)(windows_core::Interface::as_raw(self), pwchkakarireading.param().abi(), pwchkakaridisplay.param().abi(), ulkakaripos, pwchukereading.param().abi(), pwchukedisplay.param().abi(), ulukepos, jrel, ulwordsrc, pchbuffer, cbbuffer, pcdp).ok()
    }
    pub unsafe fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NextDependencies)(windows_core::Interface::as_raw(self), pchbuffer, cbbuffer, pcdp).ok()
    }
    pub unsafe fn ConvertFromOldMSIME<P0>(&self, pchdic: P0, pfnlog: PFNLOG, reg: IMEREG) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        (windows_core::Interface::vtable(self).ConvertFromOldMSIME)(windows_core::Interface::as_raw(self), pchdic.param().abi(), pfnlog, reg).ok()
    }
    pub unsafe fn ConvertFromUserToSys(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertFromUserToSys)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IFEDictionary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, *mut IMESHF) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PSTR, *mut IMESHF, *mut IMEFMT, *mut u32) -> windows_core::HRESULT,
    pub DisplayProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetPosTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut POSTBL, *mut i32) -> windows_core::HRESULT,
    pub GetWords: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub NextWords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut IMESHF) -> windows_core::HRESULT,
    pub SetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMESHF) -> windows_core::HRESULT,
    pub ExistWord: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEWRD) -> windows_core::HRESULT,
    pub ExistDependency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEDP) -> windows_core::HRESULT,
    pub RegisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, IMEREG, *mut IMEWRD) -> windows_core::HRESULT,
    pub RegisterDependency: unsafe extern "system" fn(*mut core::ffi::c_void, IMEREG, *mut IMEDP) -> windows_core::HRESULT,
    pub GetDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, windows_core::PCWSTR, windows_core::PCWSTR, u32, IMEREL, u32, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub NextDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub ConvertFromOldMSIME: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, PFNLOG, IMEREG) -> windows_core::HRESULT,
    pub ConvertFromUserToSys: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFELanguage, IFELanguage_Vtbl, 0x019f7152_e6db_11d0_83c3_00c04fddb82e);
impl core::ops::Deref for IFELanguage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFELanguage, windows_core::IUnknown);
impl IFELanguage {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetJMorphResult<P0>(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: P0, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetJMorphResult)(windows_core::Interface::as_raw(self), dwrequest, dwcmode, cwchinput, pwchinput.param().abi(), pfcinfo, ppresult).ok()
    }
    pub unsafe fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConversionModeCaps)(windows_core::Interface::as_raw(self), pdwcaps).ok()
    }
    pub unsafe fn GetPhonetic<P0>(&self, string: P0, start: i32, length: i32, phonetic: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetPhonetic)(windows_core::Interface::as_raw(self), string.param().abi(), start, length, core::mem::transmute(phonetic)).ok()
    }
    pub unsafe fn GetConversion<P0>(&self, string: P0, start: i32, length: i32, result: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetConversion)(windows_core::Interface::as_raw(self), string.param().abi(), start, length, core::mem::transmute(result)).ok()
    }
}
#[repr(C)]
pub struct IFELanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJMorphResult: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32, windows_core::PCWSTR, *mut u32, *mut *mut MORRSLT) -> windows_core::HRESULT,
    pub GetConversionModeCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPhonetic: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetConversion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImePad, IImePad_Vtbl, 0x5d8e643a_c3a9_11d1_afef_00805f0c8b6d);
impl core::ops::Deref for IImePad {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImePad, windows_core::IUnknown);
impl IImePad {
    pub unsafe fn Request<P0, P1, P2>(&self, piimepadapplet: P0, reqid: IME_PAD_REQUEST_FLAGS, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IImePadApplet>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), piimepadapplet.param().abi(), reqid.0 as _, wparam.param().abi(), lparam.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IImePad_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImePadApplet, IImePadApplet_Vtbl, 0x5d8e643b_c3a9_11d1_afef_00805f0c8b6d);
impl core::ops::Deref for IImePadApplet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImePadApplet, windows_core::IUnknown);
impl IImePadApplet {
    pub unsafe fn Initialize<P0>(&self, lpiimepad: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lpiimepad.param().abi()).ok()
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppletConfig)(windows_core::Interface::as_raw(self), lpappletcfg).ok()
    }
    pub unsafe fn CreateUI<P0>(&self, hwndparent: P0, lpimeappletui: *mut IMEAPPLETUI) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).CreateUI)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), lpimeappletui).ok()
    }
    pub unsafe fn Notify<P0, P1, P2>(&self, lpimepad: P0, notify: i32, wparam: P1, lparam: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), lpimepad.param().abi(), notify, wparam.param().abi(), lparam.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IImePadApplet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetAppletConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEAPPLETCFG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetAppletConfig: usize,
    pub CreateUI: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut IMEAPPLETUI) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImePlugInDictDictionaryList, IImePlugInDictDictionaryList_Vtbl, 0x98752974_b0a6_489b_8f6f_bff3769c8eeb);
impl core::ops::Deref for IImePlugInDictDictionaryList {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImePlugInDictDictionaryList, windows_core::IUnknown);
impl IImePlugInDictDictionaryList {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDictionariesInUse)(windows_core::Interface::as_raw(self), prgdictionaryguid, prgdatecreated, prgfencrypted).ok()
    }
    pub unsafe fn DeleteDictionary<P0>(&self, bstrdictionaryguid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteDictionary)(windows_core::Interface::as_raw(self), bstrdictionaryguid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IImePlugInDictDictionaryList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDictionariesInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::super::System::Com::SAFEARRAY, *mut *mut super::super::super::System::Com::SAFEARRAY, *mut *mut super::super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDictionariesInUse: usize,
    pub DeleteDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImeSpecifyApplets, IImeSpecifyApplets_Vtbl, 0x5d8e643c_c3a9_11d1_afef_00805f0c8b6d);
impl core::ops::Deref for IImeSpecifyApplets {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImeSpecifyApplets, windows_core::IUnknown);
impl IImeSpecifyApplets {
    pub unsafe fn GetAppletIIDList(&self, refiid: *const windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAppletIIDList)(windows_core::Interface::as_raw(self), refiid, lpiidlist).ok()
    }
}
#[repr(C)]
pub struct IImeSpecifyApplets_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAppletIIDList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut APPLETIDLIST) -> windows_core::HRESULT,
}
pub const ATTR_CONVERTED: u32 = 2u32;
pub const ATTR_FIXEDCONVERTED: u32 = 5u32;
pub const ATTR_INPUT: u32 = 0u32;
pub const ATTR_INPUT_ERROR: u32 = 4u32;
pub const ATTR_TARGET_CONVERTED: u32 = 1u32;
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3u32;
pub const CATID_MSIME_IImePadApplet: windows_core::GUID = windows_core::GUID::from_u128(0x7566cad1_4ec9_4478_9fe9_8ed766619edf);
pub const CATID_MSIME_IImePadApplet1000: windows_core::GUID = windows_core::GUID::from_u128(0xe081e1d6_2389_43cb_b66f_609f823d9f9c);
pub const CATID_MSIME_IImePadApplet1200: windows_core::GUID = windows_core::GUID::from_u128(0xa47fb5fc_7d15_4223_a789_b781bf9ae667);
pub const CATID_MSIME_IImePadApplet900: windows_core::GUID = windows_core::GUID::from_u128(0xfaae51bf_5e5b_4a1d_8de1_17c1d9e1728d);
pub const CATID_MSIME_IImePadApplet_VER7: windows_core::GUID = windows_core::GUID::from_u128(0x4a0f8e31_c3ee_11d1_afef_00805f0c8b6d);
pub const CATID_MSIME_IImePadApplet_VER80: windows_core::GUID = windows_core::GUID::from_u128(0x56f7a792_fef1_11d3_8463_00c04f7a06e5);
pub const CATID_MSIME_IImePadApplet_VER81: windows_core::GUID = windows_core::GUID::from_u128(0x656520b0_bb88_11d4_84c0_00c04f7a06e5);
pub const CFS_CANDIDATEPOS: u32 = 64u32;
pub const CFS_DEFAULT: u32 = 0u32;
pub const CFS_EXCLUDE: u32 = 128u32;
pub const CFS_FORCE_POSITION: u32 = 32u32;
pub const CFS_POINT: u32 = 2u32;
pub const CFS_RECT: u32 = 1u32;
pub const CHARINFO_APPLETID_MASK: u32 = 4278190080u32;
pub const CHARINFO_CHARID_MASK: u32 = 65535u32;
pub const CHARINFO_FEID_MASK: u32 = 15728640u32;
pub const CLSID_ImePlugInDictDictionaryList_CHS: windows_core::GUID = windows_core::GUID::from_u128(0x7bf0129b_5bef_4de4_9b0b_5edb66ac2fa6);
pub const CLSID_ImePlugInDictDictionaryList_JPN: windows_core::GUID = windows_core::GUID::from_u128(0x4fe2776b_b0f9_4396_b5fc_e9d4cf1ec195);
pub const CLSID_VERSION_DEPENDENT_MSIME_JAPANESE: windows_core::GUID = windows_core::GUID::from_u128(0x6a91029e_aa49_471b_aee7_7d332785660d);
pub const CPS_CANCEL: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(4u32);
pub const CPS_COMPLETE: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(1u32);
pub const CPS_CONVERT: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(2u32);
pub const CPS_REVERT: NOTIFY_IME_INDEX = NOTIFY_IME_INDEX(3u32);
pub const CS_INSERTCHAR: u32 = 8192u32;
pub const CS_NOMOVECARET: u32 = 16384u32;
pub const E_LARGEINPUT: u32 = 51u32;
pub const E_NOCAND: u32 = 48u32;
pub const E_NOTENOUGH_BUFFER: u32 = 49u32;
pub const E_NOTENOUGH_WDD: u32 = 50u32;
pub const FEID_CHINESE_HONGKONG: u32 = 3u32;
pub const FEID_CHINESE_SIMPLIFIED: u32 = 2u32;
pub const FEID_CHINESE_SINGAPORE: u32 = 4u32;
pub const FEID_CHINESE_TRADITIONAL: u32 = 1u32;
pub const FEID_JAPANESE: u32 = 5u32;
pub const FEID_KOREAN: u32 = 6u32;
pub const FEID_KOREAN_JOHAB: u32 = 7u32;
pub const FEID_NONE: u32 = 0u32;
pub const FELANG_CLMN_FIXD: u32 = 32u32;
pub const FELANG_CLMN_FIXR: u32 = 16u32;
pub const FELANG_CLMN_NOPBREAK: u32 = 8u32;
pub const FELANG_CLMN_NOWBREAK: u32 = 2u32;
pub const FELANG_CLMN_PBREAK: u32 = 4u32;
pub const FELANG_CLMN_WBREAK: u32 = 1u32;
pub const FELANG_CMODE_AUTOMATIC: u32 = 134217728u32;
pub const FELANG_CMODE_BESTFIRST: u32 = 16384u32;
pub const FELANG_CMODE_BOPOMOFO: u32 = 64u32;
pub const FELANG_CMODE_CONVERSATION: u32 = 536870912u32;
pub const FELANG_CMODE_FULLWIDTHOUT: u32 = 32u32;
pub const FELANG_CMODE_HALFWIDTHOUT: u32 = 16u32;
pub const FELANG_CMODE_HANGUL: u32 = 128u32;
pub const FELANG_CMODE_HIRAGANAOUT: u32 = 0u32;
pub const FELANG_CMODE_KATAKANAOUT: u32 = 8u32;
pub const FELANG_CMODE_MERGECAND: u32 = 4096u32;
pub const FELANG_CMODE_MONORUBY: u32 = 2u32;
pub const FELANG_CMODE_NAME: u32 = 268435456u32;
pub const FELANG_CMODE_NOINVISIBLECHAR: u32 = 1073741824u32;
pub const FELANG_CMODE_NONE: u32 = 16777216u32;
pub const FELANG_CMODE_NOPRUNING: u32 = 4u32;
pub const FELANG_CMODE_PHRASEPREDICT: u32 = 268435456u32;
pub const FELANG_CMODE_PINYIN: u32 = 256u32;
pub const FELANG_CMODE_PLAURALCLAUSE: u32 = 33554432u32;
pub const FELANG_CMODE_PRECONV: u32 = 512u32;
pub const FELANG_CMODE_RADICAL: u32 = 1024u32;
pub const FELANG_CMODE_ROMAN: u32 = 8192u32;
pub const FELANG_CMODE_SINGLECONVERT: u32 = 67108864u32;
pub const FELANG_CMODE_UNKNOWNREADING: u32 = 2048u32;
pub const FELANG_CMODE_USENOREVWORDS: u32 = 32768u32;
pub const FELANG_INVALD_PO: u32 = 65535u32;
pub const FELANG_REQ_CONV: u32 = 65536u32;
pub const FELANG_REQ_RECONV: u32 = 131072u32;
pub const FELANG_REQ_REV: u32 = 196608u32;
pub const FID_MSIME_KMS_DEL_KEYLIST: u32 = 4u32;
pub const FID_MSIME_KMS_FUNCDESC: u32 = 9u32;
pub const FID_MSIME_KMS_GETMAP: u32 = 6u32;
pub const FID_MSIME_KMS_GETMAPFAST: u32 = 11u32;
pub const FID_MSIME_KMS_GETMAPSEAMLESS: u32 = 10u32;
pub const FID_MSIME_KMS_INIT: u32 = 2u32;
pub const FID_MSIME_KMS_INVOKE: u32 = 7u32;
pub const FID_MSIME_KMS_NOTIFY: u32 = 5u32;
pub const FID_MSIME_KMS_SETMAP: u32 = 8u32;
pub const FID_MSIME_KMS_TERM: u32 = 3u32;
pub const FID_MSIME_KMS_VERSION: u32 = 1u32;
pub const FID_MSIME_VERSION: u32 = 0u32;
pub const FID_RECONVERT_VERSION: u32 = 268435456u32;
pub const GCL_CONVERSION: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(1u32);
pub const GCL_REVERSECONVERSION: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(2u32);
pub const GCL_REVERSE_LENGTH: GET_CONVERSION_LIST_FLAG = GET_CONVERSION_LIST_FLAG(3u32);
pub const GCSEX_CANCELRECONVERT: u32 = 268435456u32;
pub const GCS_COMPATTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(16u32);
pub const GCS_COMPCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(32u32);
pub const GCS_COMPREADATTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(2u32);
pub const GCS_COMPREADCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(4u32);
pub const GCS_COMPREADSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(1u32);
pub const GCS_COMPSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(8u32);
pub const GCS_CURSORPOS: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(128u32);
pub const GCS_DELTASTART: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(256u32);
pub const GCS_RESULTCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(4096u32);
pub const GCS_RESULTREADCLAUSE: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(1024u32);
pub const GCS_RESULTREADSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(512u32);
pub const GCS_RESULTSTR: IME_COMPOSITION_STRING = IME_COMPOSITION_STRING(2048u32);
pub const GGL_INDEX: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(2u32);
pub const GGL_LEVEL: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(1u32);
pub const GGL_PRIVATE: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(4u32);
pub const GGL_STRING: GET_GUIDE_LINE_TYPE = GET_GUIDE_LINE_TYPE(3u32);
pub const GL_ID_CANNOTSAVE: u32 = 17u32;
pub const GL_ID_CHOOSECANDIDATE: u32 = 40u32;
pub const GL_ID_INPUTCODE: u32 = 38u32;
pub const GL_ID_INPUTRADICAL: u32 = 37u32;
pub const GL_ID_INPUTREADING: u32 = 36u32;
pub const GL_ID_INPUTSYMBOL: u32 = 39u32;
pub const GL_ID_NOCONVERT: u32 = 32u32;
pub const GL_ID_NODICTIONARY: u32 = 16u32;
pub const GL_ID_NOMODULE: u32 = 1u32;
pub const GL_ID_PRIVATE_FIRST: u32 = 32768u32;
pub const GL_ID_PRIVATE_LAST: u32 = 65535u32;
pub const GL_ID_READINGCONFLICT: u32 = 35u32;
pub const GL_ID_REVERSECONVERSION: u32 = 41u32;
pub const GL_ID_TOOMANYSTROKE: u32 = 34u32;
pub const GL_ID_TYPINGERROR: u32 = 33u32;
pub const GL_ID_UNKNOWN: u32 = 0u32;
pub const GL_LEVEL_ERROR: u32 = 2u32;
pub const GL_LEVEL_FATAL: u32 = 1u32;
pub const GL_LEVEL_INFORMATION: u32 = 4u32;
pub const GL_LEVEL_NOGUIDELINE: u32 = 0u32;
pub const GL_LEVEL_WARNING: u32 = 3u32;
pub const IACE_CHILDREN: u32 = 1u32;
pub const IACE_DEFAULT: u32 = 16u32;
pub const IACE_IGNORENOCONTEXT: u32 = 32u32;
pub const IFEC_S_ALREADY_DEFAULT: windows_core::HRESULT = windows_core::HRESULT(0x47400_u32 as _);
pub const IFED_ACTIVE_DICT: IMEFMT = IMEFMT(13i32);
pub const IFED_ATOK10: IMEFMT = IMEFMT(15i32);
pub const IFED_ATOK9: IMEFMT = IMEFMT(14i32);
pub const IFED_E_INVALID_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80047301_u32 as _);
pub const IFED_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80047300_u32 as _);
pub const IFED_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80047307_u32 as _);
pub const IFED_E_NOT_USER_DIC: windows_core::HRESULT = windows_core::HRESULT(0x80047306_u32 as _);
pub const IFED_E_NO_ENTRY: windows_core::HRESULT = windows_core::HRESULT(0x80047304_u32 as _);
pub const IFED_E_OPEN_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80047302_u32 as _);
pub const IFED_E_REGISTER_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x8004730B_u32 as _);
pub const IFED_E_REGISTER_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80047305_u32 as _);
pub const IFED_E_REGISTER_ILLEGAL_POS: windows_core::HRESULT = windows_core::HRESULT(0x80047309_u32 as _);
pub const IFED_E_REGISTER_IMPROPER_WORD: windows_core::HRESULT = windows_core::HRESULT(0x8004730A_u32 as _);
pub const IFED_E_USER_COMMENT: windows_core::HRESULT = windows_core::HRESULT(0x80047308_u32 as _);
pub const IFED_E_WRITE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80047303_u32 as _);
pub const IFED_MSIME2_BIN_SYSTEM: IMEFMT = IMEFMT(1i32);
pub const IFED_MSIME2_BIN_USER: IMEFMT = IMEFMT(2i32);
pub const IFED_MSIME2_TEXT_USER: IMEFMT = IMEFMT(3i32);
pub const IFED_MSIME95_BIN_SYSTEM: IMEFMT = IMEFMT(4i32);
pub const IFED_MSIME95_BIN_USER: IMEFMT = IMEFMT(5i32);
pub const IFED_MSIME95_TEXT_USER: IMEFMT = IMEFMT(6i32);
pub const IFED_MSIME97_BIN_SYSTEM: IMEFMT = IMEFMT(7i32);
pub const IFED_MSIME97_BIN_USER: IMEFMT = IMEFMT(8i32);
pub const IFED_MSIME97_TEXT_USER: IMEFMT = IMEFMT(9i32);
pub const IFED_MSIME98_BIN_SYSTEM: IMEFMT = IMEFMT(10i32);
pub const IFED_MSIME98_BIN_USER: IMEFMT = IMEFMT(11i32);
pub const IFED_MSIME98_SYSTEM_CE: IMEFMT = IMEFMT(20i32);
pub const IFED_MSIME98_TEXT_USER: IMEFMT = IMEFMT(12i32);
pub const IFED_MSIME_BIN_SYSTEM: IMEFMT = IMEFMT(21i32);
pub const IFED_MSIME_BIN_USER: IMEFMT = IMEFMT(22i32);
pub const IFED_MSIME_TEXT_USER: IMEFMT = IMEFMT(23i32);
pub const IFED_NEC_AI_: IMEFMT = IMEFMT(16i32);
pub const IFED_PIME2_BIN_STANDARD_SYSTEM: IMEFMT = IMEFMT(26i32);
pub const IFED_PIME2_BIN_SYSTEM: IMEFMT = IMEFMT(25i32);
pub const IFED_PIME2_BIN_USER: IMEFMT = IMEFMT(24i32);
pub const IFED_POS_ADJECTIVE: u32 = 4u32;
pub const IFED_POS_ADJECTIVE_VERB: u32 = 8u32;
pub const IFED_POS_ADNOUN: u32 = 32u32;
pub const IFED_POS_ADVERB: u32 = 16u32;
pub const IFED_POS_AFFIX: u32 = 1536u32;
pub const IFED_POS_ALL: u32 = 131071u32;
pub const IFED_POS_AUXILIARY_VERB: u32 = 32768u32;
pub const IFED_POS_CONJUNCTION: u32 = 64u32;
pub const IFED_POS_DEPENDENT: u32 = 114688u32;
pub const IFED_POS_IDIOMS: u32 = 4096u32;
pub const IFED_POS_INDEPENDENT: u32 = 255u32;
pub const IFED_POS_INFLECTIONALSUFFIX: u32 = 256u32;
pub const IFED_POS_INTERJECTION: u32 = 128u32;
pub const IFED_POS_NONE: u32 = 0u32;
pub const IFED_POS_NOUN: u32 = 1u32;
pub const IFED_POS_PARTICLE: u32 = 16384u32;
pub const IFED_POS_PREFIX: u32 = 512u32;
pub const IFED_POS_SUB_VERB: u32 = 65536u32;
pub const IFED_POS_SUFFIX: u32 = 1024u32;
pub const IFED_POS_SYMBOLS: u32 = 8192u32;
pub const IFED_POS_TANKANJI: u32 = 2048u32;
pub const IFED_POS_VERB: u32 = 2u32;
pub const IFED_REG_ALL: u32 = 7u32;
pub const IFED_REG_AUTO: u32 = 2u32;
pub const IFED_REG_DEL: IMEREG = IMEREG(2i32);
pub const IFED_REG_GRAMMAR: u32 = 4u32;
pub const IFED_REG_HEAD: IMEREG = IMEREG(0i32);
pub const IFED_REG_NONE: u32 = 0u32;
pub const IFED_REG_TAIL: IMEREG = IMEREG(1i32);
pub const IFED_REG_USER: u32 = 1u32;
pub const IFED_REL_ALL: IMEREL = IMEREL(24i32);
pub const IFED_REL_DE: IMEREL = IMEREL(5i32);
pub const IFED_REL_FUKU_YOUGEN: IMEREL = IMEREL(12i32);
pub const IFED_REL_GA: IMEREL = IMEREL(2i32);
pub const IFED_REL_HE: IMEREL = IMEREL(9i32);
pub const IFED_REL_IDEOM: IMEREL = IMEREL(11i32);
pub const IFED_REL_KARA: IMEREL = IMEREL(7i32);
pub const IFED_REL_KEIDOU1_YOUGEN: IMEREL = IMEREL(14i32);
pub const IFED_REL_KEIDOU2_YOUGEN: IMEREL = IMEREL(15i32);
pub const IFED_REL_KEIYOU_TARU_YOUGEN: IMEREL = IMEREL(21i32);
pub const IFED_REL_KEIYOU_TO_YOUGEN: IMEREL = IMEREL(20i32);
pub const IFED_REL_KEIYOU_YOUGEN: IMEREL = IMEREL(13i32);
pub const IFED_REL_MADE: IMEREL = IMEREL(8i32);
pub const IFED_REL_NI: IMEREL = IMEREL(4i32);
pub const IFED_REL_NO: IMEREL = IMEREL(1i32);
pub const IFED_REL_NONE: IMEREL = IMEREL(0i32);
pub const IFED_REL_RENSOU: IMEREL = IMEREL(19i32);
pub const IFED_REL_RENTAI_MEI: IMEREL = IMEREL(18i32);
pub const IFED_REL_TAIGEN: IMEREL = IMEREL(16i32);
pub const IFED_REL_TO: IMEREL = IMEREL(10i32);
pub const IFED_REL_UNKNOWN1: IMEREL = IMEREL(22i32);
pub const IFED_REL_UNKNOWN2: IMEREL = IMEREL(23i32);
pub const IFED_REL_WO: IMEREL = IMEREL(3i32);
pub const IFED_REL_YORI: IMEREL = IMEREL(6i32);
pub const IFED_REL_YOUGEN: IMEREL = IMEREL(17i32);
pub const IFED_SELECT_ALL: u32 = 15u32;
pub const IFED_SELECT_COMMENT: u32 = 8u32;
pub const IFED_SELECT_DISPLAY: u32 = 2u32;
pub const IFED_SELECT_NONE: u32 = 0u32;
pub const IFED_SELECT_POS: u32 = 4u32;
pub const IFED_SELECT_READING: u32 = 1u32;
pub const IFED_S_COMMENT_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x47203_u32 as _);
pub const IFED_S_EMPTY_DICTIONARY: windows_core::HRESULT = windows_core::HRESULT(0x47201_u32 as _);
pub const IFED_S_MORE_ENTRIES: windows_core::HRESULT = windows_core::HRESULT(0x47200_u32 as _);
pub const IFED_S_WORD_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x47202_u32 as _);
pub const IFED_TYPE_ALL: u32 = 31u32;
pub const IFED_TYPE_ENGLISH: u32 = 16u32;
pub const IFED_TYPE_GENERAL: u32 = 1u32;
pub const IFED_TYPE_NAMEPLACE: u32 = 2u32;
pub const IFED_TYPE_NONE: u32 = 0u32;
pub const IFED_TYPE_REVERSE: u32 = 8u32;
pub const IFED_TYPE_SPEECH: u32 = 4u32;
pub const IFED_UCT_MAX: IMEUCT = IMEUCT(4i32);
pub const IFED_UCT_NONE: IMEUCT = IMEUCT(0i32);
pub const IFED_UCT_STRING_SJIS: IMEUCT = IMEUCT(1i32);
pub const IFED_UCT_STRING_UNICODE: IMEUCT = IMEUCT(2i32);
pub const IFED_UCT_USER_DEFINED: IMEUCT = IMEUCT(3i32);
pub const IFED_UNKNOWN: IMEFMT = IMEFMT(0i32);
pub const IFED_VJE_20: IMEFMT = IMEFMT(19i32);
pub const IFED_WX_II: IMEFMT = IMEFMT(17i32);
pub const IFED_WX_III: IMEFMT = IMEFMT(18i32);
pub const IGIMIF_RIGHTMENU: u32 = 1u32;
pub const IGIMII_CMODE: u32 = 1u32;
pub const IGIMII_CONFIGURE: u32 = 4u32;
pub const IGIMII_HELP: u32 = 16u32;
pub const IGIMII_INPUTTOOLS: u32 = 64u32;
pub const IGIMII_OTHER: u32 = 32u32;
pub const IGIMII_SMODE: u32 = 2u32;
pub const IGIMII_TOOLS: u32 = 8u32;
pub const IMC_CLOSESTATUSWINDOW: u32 = 33u32;
pub const IMC_GETCANDIDATEPOS: u32 = 7u32;
pub const IMC_GETCOMPOSITIONFONT: u32 = 9u32;
pub const IMC_GETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMC_GETSOFTKBDFONT: u32 = 17u32;
pub const IMC_GETSOFTKBDPOS: u32 = 19u32;
pub const IMC_GETSOFTKBDSUBTYPE: u32 = 21u32;
pub const IMC_GETSTATUSWINDOWPOS: u32 = 15u32;
pub const IMC_OPENSTATUSWINDOW: u32 = 34u32;
pub const IMC_SETCANDIDATEPOS: u32 = 8u32;
pub const IMC_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMC_SETCOMPOSITIONWINDOW: u32 = 12u32;
pub const IMC_SETCONVERSIONMODE: u32 = 2u32;
pub const IMC_SETOPENSTATUS: u32 = 6u32;
pub const IMC_SETSENTENCEMODE: u32 = 4u32;
pub const IMC_SETSOFTKBDDATA: u32 = 24u32;
pub const IMC_SETSOFTKBDFONT: u32 = 18u32;
pub const IMC_SETSOFTKBDPOS: u32 = 20u32;
pub const IMC_SETSOFTKBDSUBTYPE: u32 = 22u32;
pub const IMC_SETSTATUSWINDOWPOS: u32 = 16u32;
pub const IMEFAREASTINFO_TYPE_COMMENT: u32 = 2u32;
pub const IMEFAREASTINFO_TYPE_COSTTIME: u32 = 3u32;
pub const IMEFAREASTINFO_TYPE_DEFAULT: u32 = 0u32;
pub const IMEFAREASTINFO_TYPE_READING: u32 = 1u32;
pub const IMEKEYCTRLMASK_ALT: u32 = 1u32;
pub const IMEKEYCTRLMASK_CTRL: u32 = 2u32;
pub const IMEKEYCTRLMASK_SHIFT: u32 = 4u32;
pub const IMEKEYCTRL_DOWN: u32 = 0u32;
pub const IMEKEYCTRL_UP: u32 = 1u32;
pub const IMEKMS_2NDLEVEL: u32 = 4u32;
pub const IMEKMS_CANDIDATE: u32 = 6u32;
pub const IMEKMS_COMPOSITION: u32 = 1u32;
pub const IMEKMS_IMEOFF: u32 = 3u32;
pub const IMEKMS_INPTGL: u32 = 5u32;
pub const IMEKMS_NOCOMPOSITION: u32 = 0u32;
pub const IMEKMS_SELECTION: u32 = 2u32;
pub const IMEKMS_TYPECAND: u32 = 7u32;
pub const IMEMENUITEM_STRING_SIZE: u32 = 80u32;
pub const IMEMOUSERET_NOTHANDLED: i32 = -1i32;
pub const IMEMOUSE_LDOWN: u32 = 1u32;
pub const IMEMOUSE_MDOWN: u32 = 4u32;
pub const IMEMOUSE_NONE: u32 = 0u32;
pub const IMEMOUSE_RDOWN: u32 = 2u32;
pub const IMEMOUSE_VERSION: u32 = 255u32;
pub const IMEMOUSE_WDOWN: u32 = 32u32;
pub const IMEMOUSE_WUP: u32 = 16u32;
pub const IMEPADCTRL_CARETBACKSPACE: u32 = 10u32;
pub const IMEPADCTRL_CARETBOTTOM: u32 = 9u32;
pub const IMEPADCTRL_CARETDELETE: u32 = 11u32;
pub const IMEPADCTRL_CARETLEFT: u32 = 6u32;
pub const IMEPADCTRL_CARETRIGHT: u32 = 7u32;
pub const IMEPADCTRL_CARETSET: u32 = 5u32;
pub const IMEPADCTRL_CARETTOP: u32 = 8u32;
pub const IMEPADCTRL_CLEARALL: u32 = 4u32;
pub const IMEPADCTRL_CONVERTALL: u32 = 1u32;
pub const IMEPADCTRL_DETERMINALL: u32 = 2u32;
pub const IMEPADCTRL_DETERMINCHAR: u32 = 3u32;
pub const IMEPADCTRL_INSERTFULLSPACE: u32 = 14u32;
pub const IMEPADCTRL_INSERTHALFSPACE: u32 = 15u32;
pub const IMEPADCTRL_INSERTSPACE: u32 = 13u32;
pub const IMEPADCTRL_OFFIME: u32 = 17u32;
pub const IMEPADCTRL_OFFPRECONVERSION: u32 = 19u32;
pub const IMEPADCTRL_ONIME: u32 = 16u32;
pub const IMEPADCTRL_ONPRECONVERSION: u32 = 18u32;
pub const IMEPADCTRL_PHONETICCANDIDATE: u32 = 20u32;
pub const IMEPADCTRL_PHRASEDELETE: u32 = 12u32;
pub const IMEPADREQ_CHANGESTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4113u32);
pub const IMEPADREQ_CHANGESTRINGCANDIDATEINFO: u32 = 4111u32;
pub const IMEPADREQ_CHANGESTRINGINFO: u32 = 4115u32;
pub const IMEPADREQ_DELETESTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4112u32);
pub const IMEPADREQ_FIRST: u32 = 4096u32;
pub const IMEPADREQ_FORCEIMEPADWINDOWSHOW: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4117u32);
pub const IMEPADREQ_GETAPPLETDATA: u32 = 4106u32;
pub const IMEPADREQ_GETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4121u32);
pub const IMEPADREQ_GETAPPLHWND: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4116u32);
pub const IMEPADREQ_GETCOMPOSITIONSTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4102u32);
pub const IMEPADREQ_GETCOMPOSITIONSTRINGID: u32 = 4109u32;
pub const IMEPADREQ_GETCOMPOSITIONSTRINGINFO: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4108u32);
pub const IMEPADREQ_GETCONVERSIONSTATUS: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4126u32);
pub const IMEPADREQ_GETCURRENTIMEINFO: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4128u32);
pub const IMEPADREQ_GETCURRENTUILANGID: u32 = 4120u32;
pub const IMEPADREQ_GETDEFAULTUILANGID: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4119u32);
pub const IMEPADREQ_GETSELECTEDSTRING: u32 = 4103u32;
pub const IMEPADREQ_GETVERSION: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4127u32);
pub const IMEPADREQ_INSERTITEMCANDIDATE: u32 = 4099u32;
pub const IMEPADREQ_INSERTSTRING: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4097u32);
pub const IMEPADREQ_INSERTSTRINGCANDIDATE: u32 = 4098u32;
pub const IMEPADREQ_INSERTSTRINGCANDIDATEINFO: u32 = 4110u32;
pub const IMEPADREQ_INSERTSTRINGINFO: u32 = 4114u32;
pub const IMEPADREQ_ISAPPLETACTIVE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4123u32);
pub const IMEPADREQ_ISIMEPADWINDOWVISIBLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4124u32);
pub const IMEPADREQ_POSTMODALNOTIFY: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4118u32);
pub const IMEPADREQ_SENDCONTROL: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4100u32);
pub const IMEPADREQ_SENDKEYCONTROL: u32 = 4101u32;
pub const IMEPADREQ_SETAPPLETDATA: u32 = 4105u32;
pub const IMEPADREQ_SETAPPLETMINMAXSIZE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4125u32);
pub const IMEPADREQ_SETAPPLETSIZE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4104u32);
pub const IMEPADREQ_SETAPPLETUISTYLE: IME_PAD_REQUEST_FLAGS = IME_PAD_REQUEST_FLAGS(4122u32);
pub const IMEPADREQ_SETTITLEFONT: u32 = 4107u32;
pub const IMEPN_ACTIVATE: u32 = 257u32;
pub const IMEPN_APPLYCAND: u32 = 267u32;
pub const IMEPN_APPLYCANDEX: u32 = 268u32;
pub const IMEPN_CONFIG: u32 = 264u32;
pub const IMEPN_FIRST: u32 = 256u32;
pub const IMEPN_HELP: u32 = 265u32;
pub const IMEPN_HIDE: u32 = 261u32;
pub const IMEPN_INACTIVATE: u32 = 258u32;
pub const IMEPN_QUERYCAND: u32 = 266u32;
pub const IMEPN_SETTINGCHANGED: u32 = 269u32;
pub const IMEPN_SHOW: u32 = 260u32;
pub const IMEPN_SIZECHANGED: u32 = 263u32;
pub const IMEPN_SIZECHANGING: u32 = 262u32;
pub const IMEPN_USER: u32 = 356u32;
pub const IMEVER_0310: u32 = 196618u32;
pub const IMEVER_0400: u32 = 262144u32;
pub const IME_CAND_CODE: u32 = 2u32;
pub const IME_CAND_MEANING: u32 = 3u32;
pub const IME_CAND_RADICAL: u32 = 4u32;
pub const IME_CAND_READ: u32 = 1u32;
pub const IME_CAND_STROKE: u32 = 5u32;
pub const IME_CAND_UNKNOWN: u32 = 0u32;
pub const IME_CHOTKEY_IME_NONIME_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(16u32);
pub const IME_CHOTKEY_SHAPE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(17u32);
pub const IME_CHOTKEY_SYMBOL_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(18u32);
pub const IME_CMODE_ALPHANUMERIC: IME_CONVERSION_MODE = IME_CONVERSION_MODE(0u32);
pub const IME_CMODE_CHARCODE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(32u32);
pub const IME_CMODE_CHINESE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_EUDC: IME_CONVERSION_MODE = IME_CONVERSION_MODE(512u32);
pub const IME_CMODE_FIXED: IME_CONVERSION_MODE = IME_CONVERSION_MODE(2048u32);
pub const IME_CMODE_FULLSHAPE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(8u32);
pub const IME_CMODE_HANGEUL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_HANGUL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_HANJACONVERT: IME_CONVERSION_MODE = IME_CONVERSION_MODE(64u32);
pub const IME_CMODE_JAPANESE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_KATAKANA: IME_CONVERSION_MODE = IME_CONVERSION_MODE(2u32);
pub const IME_CMODE_LANGUAGE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(3u32);
pub const IME_CMODE_NATIVE: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1u32);
pub const IME_CMODE_NATIVESYMBOL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(128u32);
pub const IME_CMODE_NOCONVERSION: IME_CONVERSION_MODE = IME_CONVERSION_MODE(256u32);
pub const IME_CMODE_RESERVED: IME_CONVERSION_MODE = IME_CONVERSION_MODE(4026531840u32);
pub const IME_CMODE_ROMAN: IME_CONVERSION_MODE = IME_CONVERSION_MODE(16u32);
pub const IME_CMODE_SOFTKBD: IME_CONVERSION_MODE = IME_CONVERSION_MODE(128u32);
pub const IME_CMODE_SYMBOL: IME_CONVERSION_MODE = IME_CONVERSION_MODE(1024u32);
pub const IME_CONFIG_GENERAL: u32 = 1u32;
pub const IME_CONFIG_REGISTERWORD: u32 = 2u32;
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3u32;
pub const IME_ESC_AUTOMATA: IME_ESCAPE = IME_ESCAPE(4105u32);
pub const IME_ESC_GETHELPFILENAME: IME_ESCAPE = IME_ESCAPE(4107u32);
pub const IME_ESC_GET_EUDC_DICTIONARY: IME_ESCAPE = IME_ESCAPE(4099u32);
pub const IME_ESC_HANJA_MODE: IME_ESCAPE = IME_ESCAPE(4104u32);
pub const IME_ESC_IME_NAME: IME_ESCAPE = IME_ESCAPE(4102u32);
pub const IME_ESC_MAX_KEY: IME_ESCAPE = IME_ESCAPE(4101u32);
pub const IME_ESC_PRIVATE_FIRST: IME_ESCAPE = IME_ESCAPE(2048u32);
pub const IME_ESC_PRIVATE_HOTKEY: IME_ESCAPE = IME_ESCAPE(4106u32);
pub const IME_ESC_PRIVATE_LAST: IME_ESCAPE = IME_ESCAPE(4095u32);
pub const IME_ESC_QUERY_SUPPORT: IME_ESCAPE = IME_ESCAPE(3u32);
pub const IME_ESC_RESERVED_FIRST: IME_ESCAPE = IME_ESCAPE(4u32);
pub const IME_ESC_RESERVED_LAST: IME_ESCAPE = IME_ESCAPE(2047u32);
pub const IME_ESC_SEQUENCE_TO_INTERNAL: IME_ESCAPE = IME_ESCAPE(4097u32);
pub const IME_ESC_SET_EUDC_DICTIONARY: IME_ESCAPE = IME_ESCAPE(4100u32);
pub const IME_ESC_STRING_BUFFER_SIZE: u32 = 80u32;
pub const IME_ESC_SYNC_HOTKEY: IME_ESCAPE = IME_ESCAPE(4103u32);
pub const IME_HOTKEY_DSWITCH_FIRST: u32 = 256u32;
pub const IME_HOTKEY_DSWITCH_LAST: u32 = 287u32;
pub const IME_HOTKEY_PRIVATE_FIRST: u32 = 512u32;
pub const IME_HOTKEY_PRIVATE_LAST: u32 = 543u32;
pub const IME_ITHOTKEY_PREVIOUS_COMPOSITION: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(513u32);
pub const IME_ITHOTKEY_RECONVERTSTRING: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(515u32);
pub const IME_ITHOTKEY_RESEND_RESULTSTR: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(512u32);
pub const IME_ITHOTKEY_UISTYLE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(514u32);
pub const IME_JHOTKEY_CLOSE_OPEN: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(48u32);
pub const IME_KHOTKEY_ENGLISH: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(82u32);
pub const IME_KHOTKEY_HANJACONVERT: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(81u32);
pub const IME_KHOTKEY_SHAPE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(80u32);
pub const IME_PROP_ACCEPT_WIDE_VKEY: u32 = 32u32;
pub const IME_PROP_AT_CARET: u32 = 65536u32;
pub const IME_PROP_CANDLIST_START_FROM_1: u32 = 262144u32;
pub const IME_PROP_COMPLETE_ON_UNSELECT: u32 = 1048576u32;
pub const IME_PROP_END_UNLOAD: u32 = 1u32;
pub const IME_PROP_IGNORE_UPKEYS: u32 = 4u32;
pub const IME_PROP_KBD_CHAR_FIRST: u32 = 2u32;
pub const IME_PROP_NEED_ALTKEY: u32 = 8u32;
pub const IME_PROP_NO_KEYS_ON_CLOSE: u32 = 16u32;
pub const IME_PROP_SPECIAL_UI: u32 = 131072u32;
pub const IME_PROP_UNICODE: u32 = 524288u32;
pub const IME_REGWORD_STYLE_EUDC: u32 = 1u32;
pub const IME_REGWORD_STYLE_USER_FIRST: u32 = 2147483648u32;
pub const IME_REGWORD_STYLE_USER_LAST: u32 = 4294967295u32;
pub const IME_SMODE_AUTOMATIC: IME_SENTENCE_MODE = IME_SENTENCE_MODE(4u32);
pub const IME_SMODE_CONVERSATION: IME_SENTENCE_MODE = IME_SENTENCE_MODE(16u32);
pub const IME_SMODE_NONE: IME_SENTENCE_MODE = IME_SENTENCE_MODE(0u32);
pub const IME_SMODE_PHRASEPREDICT: IME_SENTENCE_MODE = IME_SENTENCE_MODE(8u32);
pub const IME_SMODE_PLAURALCLAUSE: IME_SENTENCE_MODE = IME_SENTENCE_MODE(1u32);
pub const IME_SMODE_RESERVED: IME_SENTENCE_MODE = IME_SENTENCE_MODE(61440u32);
pub const IME_SMODE_SINGLECONVERT: IME_SENTENCE_MODE = IME_SENTENCE_MODE(2u32);
pub const IME_SYSINFO_WINLOGON: u32 = 1u32;
pub const IME_THOTKEY_IME_NONIME_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(112u32);
pub const IME_THOTKEY_SHAPE_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(113u32);
pub const IME_THOTKEY_SYMBOL_TOGGLE: IME_HOTKEY_IDENTIFIER = IME_HOTKEY_IDENTIFIER(114u32);
pub const IME_UI_CLASS_NAME_SIZE: u32 = 16u32;
pub const IMFT_RADIOCHECK: u32 = 1u32;
pub const IMFT_SEPARATOR: u32 = 2u32;
pub const IMFT_SUBMENU: u32 = 4u32;
pub const IMMGWLP_IMC: u32 = 0u32;
pub const IMMGWL_IMC: u32 = 0u32;
pub const IMM_ERROR_GENERAL: i32 = -2i32;
pub const IMM_ERROR_NODATA: i32 = -1i32;
pub const IMN_CHANGECANDIDATE: u32 = 3u32;
pub const IMN_CLOSECANDIDATE: u32 = 4u32;
pub const IMN_CLOSESTATUSWINDOW: u32 = 1u32;
pub const IMN_GUIDELINE: u32 = 13u32;
pub const IMN_OPENCANDIDATE: u32 = 5u32;
pub const IMN_OPENSTATUSWINDOW: u32 = 2u32;
pub const IMN_PRIVATE: u32 = 14u32;
pub const IMN_SETCANDIDATEPOS: u32 = 9u32;
pub const IMN_SETCOMPOSITIONFONT: u32 = 10u32;
pub const IMN_SETCOMPOSITIONWINDOW: u32 = 11u32;
pub const IMN_SETCONVERSIONMODE: u32 = 6u32;
pub const IMN_SETOPENSTATUS: u32 = 8u32;
pub const IMN_SETSENTENCEMODE: u32 = 7u32;
pub const IMN_SETSTATUSWINDOWPOS: u32 = 12u32;
pub const IMN_SOFTKBDDESTROYED: u32 = 17u32;
pub const IMR_CANDIDATEWINDOW: u32 = 2u32;
pub const IMR_COMPOSITIONFONT: u32 = 3u32;
pub const IMR_COMPOSITIONWINDOW: u32 = 1u32;
pub const IMR_CONFIRMRECONVERTSTRING: u32 = 5u32;
pub const IMR_DOCUMENTFEED: u32 = 7u32;
pub const IMR_QUERYCHARPOSITION: u32 = 6u32;
pub const IMR_RECONVERTSTRING: u32 = 4u32;
pub const INFOMASK_APPLY_CAND: u32 = 2u32;
pub const INFOMASK_APPLY_CAND_EX: u32 = 4u32;
pub const INFOMASK_BLOCK_CAND: u32 = 262144u32;
pub const INFOMASK_HIDE_CAND: u32 = 131072u32;
pub const INFOMASK_NONE: u32 = 0u32;
pub const INFOMASK_QUERY_CAND: u32 = 1u32;
pub const INFOMASK_STRING_FIX: u32 = 65536u32;
pub const INIT_COMPFORM: u32 = 16u32;
pub const INIT_CONVERSION: u32 = 2u32;
pub const INIT_LOGFONT: u32 = 8u32;
pub const INIT_SENTENCE: u32 = 4u32;
pub const INIT_SOFTKBDPOS: u32 = 32u32;
pub const INIT_STATUSWNDPOS: u32 = 1u32;
pub const IPACFG_CATEGORY: i32 = 262144i32;
pub const IPACFG_HELP: i32 = 2i32;
pub const IPACFG_LANG: i32 = 16i32;
pub const IPACFG_NONE: i32 = 0i32;
pub const IPACFG_PROPERTY: i32 = 1i32;
pub const IPACFG_TITLE: i32 = 65536i32;
pub const IPACFG_TITLEFONTFACE: i32 = 131072i32;
pub const IPACID_CHARLIST: u32 = 9u32;
pub const IPACID_EPWING: u32 = 7u32;
pub const IPACID_HANDWRITING: u32 = 2u32;
pub const IPACID_NONE: u32 = 0u32;
pub const IPACID_OCR: u32 = 8u32;
pub const IPACID_RADICALSEARCH: u32 = 4u32;
pub const IPACID_SOFTKEY: u32 = 1u32;
pub const IPACID_STROKESEARCH: u32 = 3u32;
pub const IPACID_SYMBOLSEARCH: u32 = 5u32;
pub const IPACID_USER: u32 = 256u32;
pub const IPACID_VOICE: u32 = 6u32;
pub const IPAWS_ENABLED: i32 = 1i32;
pub const IPAWS_HORIZONTALFIXED: i32 = 512i32;
pub const IPAWS_MAXHEIGHTFIXED: i32 = 8192i32;
pub const IPAWS_MAXSIZEFIXED: i32 = 12288i32;
pub const IPAWS_MAXWIDTHFIXED: i32 = 4096i32;
pub const IPAWS_MINHEIGHTFIXED: i32 = 131072i32;
pub const IPAWS_MINSIZEFIXED: i32 = 196608i32;
pub const IPAWS_MINWIDTHFIXED: i32 = 65536i32;
pub const IPAWS_SIZEFIXED: i32 = 768i32;
pub const IPAWS_SIZINGNOTIFY: i32 = 4i32;
pub const IPAWS_VERTICALFIXED: i32 = 256i32;
pub const ISC_SHOWUIALL: u32 = 3221225487u32;
pub const ISC_SHOWUIALLCANDIDATEWINDOW: u32 = 15u32;
pub const ISC_SHOWUICANDIDATEWINDOW: u32 = 1u32;
pub const ISC_SHOWUICOMPOSITIONWINDOW: u32 = 2147483648u32;
pub const ISC_SHOWUIGUIDELINE: u32 = 1073741824u32;
pub const JPOS_1DAN: u32 = 213u32;
pub const JPOS_4DAN_HA: u32 = 212u32;
pub const JPOS_5DAN_AWA: u32 = 200u32;
pub const JPOS_5DAN_AWAUON: u32 = 209u32;
pub const JPOS_5DAN_BA: u32 = 206u32;
pub const JPOS_5DAN_GA: u32 = 202u32;
pub const JPOS_5DAN_KA: u32 = 201u32;
pub const JPOS_5DAN_KASOKUON: u32 = 210u32;
pub const JPOS_5DAN_MA: u32 = 207u32;
pub const JPOS_5DAN_NA: u32 = 205u32;
pub const JPOS_5DAN_RA: u32 = 208u32;
pub const JPOS_5DAN_RAHEN: u32 = 211u32;
pub const JPOS_5DAN_SA: u32 = 203u32;
pub const JPOS_5DAN_TA: u32 = 204u32;
pub const JPOS_BUPPIN: u32 = 122u32;
pub const JPOS_CHIMEI: u32 = 109u32;
pub const JPOS_CHIMEI_EKI: u32 = 117u32;
pub const JPOS_CHIMEI_GUN: u32 = 112u32;
pub const JPOS_CHIMEI_KEN: u32 = 111u32;
pub const JPOS_CHIMEI_KU: u32 = 113u32;
pub const JPOS_CHIMEI_KUNI: u32 = 110u32;
pub const JPOS_CHIMEI_MACHI: u32 = 115u32;
pub const JPOS_CHIMEI_MURA: u32 = 116u32;
pub const JPOS_CHIMEI_SHI: u32 = 114u32;
pub const JPOS_CLOSEBRACE: u32 = 911u32;
pub const JPOS_DAIMEISHI: u32 = 123u32;
pub const JPOS_DAIMEISHI_NINSHOU: u32 = 124u32;
pub const JPOS_DAIMEISHI_SHIJI: u32 = 125u32;
pub const JPOS_DOKURITSUGO: u32 = 903u32;
pub const JPOS_EIJI: u32 = 906u32;
pub const JPOS_FUKUSHI: u32 = 500u32;
pub const JPOS_FUKUSHI_DA: u32 = 504u32;
pub const JPOS_FUKUSHI_NANO: u32 = 503u32;
pub const JPOS_FUKUSHI_NI: u32 = 502u32;
pub const JPOS_FUKUSHI_SAHEN: u32 = 501u32;
pub const JPOS_FUKUSHI_TO: u32 = 505u32;
pub const JPOS_FUKUSHI_TOSURU: u32 = 506u32;
pub const JPOS_FUTEIGO: u32 = 904u32;
pub const JPOS_HUKUSIMEISHI: u32 = 104u32;
pub const JPOS_JINMEI: u32 = 106u32;
pub const JPOS_JINMEI_MEI: u32 = 108u32;
pub const JPOS_JINMEI_SEI: u32 = 107u32;
pub const JPOS_KANDOUSHI: u32 = 670u32;
pub const JPOS_KANJI: u32 = 909u32;
pub const JPOS_KANYOUKU: u32 = 902u32;
pub const JPOS_KAZU: u32 = 126u32;
pub const JPOS_KAZU_SURYOU: u32 = 127u32;
pub const JPOS_KAZU_SUSHI: u32 = 128u32;
pub const JPOS_KEIDOU: u32 = 400u32;
pub const JPOS_KEIDOU_GARU: u32 = 403u32;
pub const JPOS_KEIDOU_NO: u32 = 401u32;
pub const JPOS_KEIDOU_TARU: u32 = 402u32;
pub const JPOS_KEIYOU: u32 = 300u32;
pub const JPOS_KEIYOU_GARU: u32 = 301u32;
pub const JPOS_KEIYOU_GE: u32 = 302u32;
pub const JPOS_KEIYOU_ME: u32 = 303u32;
pub const JPOS_KEIYOU_U: u32 = 305u32;
pub const JPOS_KEIYOU_YUU: u32 = 304u32;
pub const JPOS_KENCHIKU: u32 = 121u32;
pub const JPOS_KIGOU: u32 = 905u32;
pub const JPOS_KURU_KI: u32 = 219u32;
pub const JPOS_KURU_KITA: u32 = 220u32;
pub const JPOS_KURU_KITARA: u32 = 221u32;
pub const JPOS_KURU_KITARI: u32 = 222u32;
pub const JPOS_KURU_KITAROU: u32 = 223u32;
pub const JPOS_KURU_KITE: u32 = 224u32;
pub const JPOS_KURU_KO: u32 = 226u32;
pub const JPOS_KURU_KOI: u32 = 227u32;
pub const JPOS_KURU_KOYOU: u32 = 228u32;
pub const JPOS_KURU_KUREBA: u32 = 225u32;
pub const JPOS_KUTEN: u32 = 907u32;
pub const JPOS_MEISA_KEIDOU: u32 = 105u32;
pub const JPOS_MEISHI_FUTSU: u32 = 100u32;
pub const JPOS_MEISHI_KEIYOUDOUSHI: u32 = 103u32;
pub const JPOS_MEISHI_SAHEN: u32 = 101u32;
pub const JPOS_MEISHI_ZAHEN: u32 = 102u32;
pub const JPOS_OPENBRACE: u32 = 910u32;
pub const JPOS_RENTAISHI: u32 = 600u32;
pub const JPOS_RENTAISHI_SHIJI: u32 = 601u32;
pub const JPOS_RENYOU_SETSUBI: u32 = 826u32;
pub const JPOS_SETSUBI: u32 = 800u32;
pub const JPOS_SETSUBI_CHIMEI: u32 = 811u32;
pub const JPOS_SETSUBI_CHOU: u32 = 818u32;
pub const JPOS_SETSUBI_CHU: u32 = 804u32;
pub const JPOS_SETSUBI_DONO: u32 = 835u32;
pub const JPOS_SETSUBI_EKI: u32 = 821u32;
pub const JPOS_SETSUBI_FU: u32 = 805u32;
pub const JPOS_SETSUBI_FUKUSU: u32 = 836u32;
pub const JPOS_SETSUBI_GUN: u32 = 814u32;
pub const JPOS_SETSUBI_JIKAN: u32 = 829u32;
pub const JPOS_SETSUBI_JIKANPLUS: u32 = 830u32;
pub const JPOS_SETSUBI_JINMEI: u32 = 810u32;
pub const JPOS_SETSUBI_JOSUSHI: u32 = 827u32;
pub const JPOS_SETSUBI_JOSUSHIPLUS: u32 = 828u32;
pub const JPOS_SETSUBI_KA: u32 = 803u32;
pub const JPOS_SETSUBI_KATA: u32 = 808u32;
pub const JPOS_SETSUBI_KEN: u32 = 813u32;
pub const JPOS_SETSUBI_KENCHIKU: u32 = 825u32;
pub const JPOS_SETSUBI_KU: u32 = 815u32;
pub const JPOS_SETSUBI_KUN: u32 = 833u32;
pub const JPOS_SETSUBI_KUNI: u32 = 812u32;
pub const JPOS_SETSUBI_MACHI: u32 = 817u32;
pub const JPOS_SETSUBI_MEISHIRENDAKU: u32 = 809u32;
pub const JPOS_SETSUBI_MURA: u32 = 819u32;
pub const JPOS_SETSUBI_RA: u32 = 838u32;
pub const JPOS_SETSUBI_RYU: u32 = 806u32;
pub const JPOS_SETSUBI_SAMA: u32 = 834u32;
pub const JPOS_SETSUBI_SAN: u32 = 832u32;
pub const JPOS_SETSUBI_SEI: u32 = 802u32;
pub const JPOS_SETSUBI_SHAMEI: u32 = 823u32;
pub const JPOS_SETSUBI_SHI: u32 = 816u32;
pub const JPOS_SETSUBI_SON: u32 = 820u32;
pub const JPOS_SETSUBI_SONOTA: u32 = 822u32;
pub const JPOS_SETSUBI_SOSHIKI: u32 = 824u32;
pub const JPOS_SETSUBI_TACHI: u32 = 837u32;
pub const JPOS_SETSUBI_TEINEI: u32 = 831u32;
pub const JPOS_SETSUBI_TEKI: u32 = 801u32;
pub const JPOS_SETSUBI_YOU: u32 = 807u32;
pub const JPOS_SETSUZOKUSHI: u32 = 650u32;
pub const JPOS_SETTOU: u32 = 700u32;
pub const JPOS_SETTOU_CHIMEI: u32 = 710u32;
pub const JPOS_SETTOU_CHOUTAN: u32 = 707u32;
pub const JPOS_SETTOU_DAISHOU: u32 = 705u32;
pub const JPOS_SETTOU_FUKU: u32 = 703u32;
pub const JPOS_SETTOU_JINMEI: u32 = 709u32;
pub const JPOS_SETTOU_JOSUSHI: u32 = 712u32;
pub const JPOS_SETTOU_KAKU: u32 = 701u32;
pub const JPOS_SETTOU_KOUTEI: u32 = 706u32;
pub const JPOS_SETTOU_MI: u32 = 704u32;
pub const JPOS_SETTOU_SAI: u32 = 702u32;
pub const JPOS_SETTOU_SHINKYU: u32 = 708u32;
pub const JPOS_SETTOU_SONOTA: u32 = 711u32;
pub const JPOS_SETTOU_TEINEI_GO: u32 = 714u32;
pub const JPOS_SETTOU_TEINEI_O: u32 = 713u32;
pub const JPOS_SETTOU_TEINEI_ON: u32 = 715u32;
pub const JPOS_SHAMEI: u32 = 119u32;
pub const JPOS_SONOTA: u32 = 118u32;
pub const JPOS_SOSHIKI: u32 = 120u32;
pub const JPOS_SURU_SA: u32 = 229u32;
pub const JPOS_SURU_SE: u32 = 238u32;
pub const JPOS_SURU_SEYO: u32 = 239u32;
pub const JPOS_SURU_SI: u32 = 230u32;
pub const JPOS_SURU_SIATRI: u32 = 233u32;
pub const JPOS_SURU_SITA: u32 = 231u32;
pub const JPOS_SURU_SITARA: u32 = 232u32;
pub const JPOS_SURU_SITAROU: u32 = 234u32;
pub const JPOS_SURU_SITE: u32 = 235u32;
pub const JPOS_SURU_SIYOU: u32 = 236u32;
pub const JPOS_SURU_SUREBA: u32 = 237u32;
pub const JPOS_TANKANJI: u32 = 900u32;
pub const JPOS_TANKANJI_KAO: u32 = 901u32;
pub const JPOS_TANSHUKU: u32 = 913u32;
pub const JPOS_TOKUSHU_KAHEN: u32 = 214u32;
pub const JPOS_TOKUSHU_NAHEN: u32 = 218u32;
pub const JPOS_TOKUSHU_SAHEN: u32 = 216u32;
pub const JPOS_TOKUSHU_SAHENSURU: u32 = 215u32;
pub const JPOS_TOKUSHU_ZAHEN: u32 = 217u32;
pub const JPOS_TOUTEN: u32 = 908u32;
pub const JPOS_UNDEFINED: u32 = 0u32;
pub const JPOS_YOKUSEI: u32 = 912u32;
pub const MAX_APPLETTITLE: u32 = 64u32;
pub const MAX_FONTFACE: u32 = 32u32;
pub const MODEBIASMODE_DEFAULT: u32 = 0u32;
pub const MODEBIASMODE_DIGIT: u32 = 4u32;
pub const MODEBIASMODE_FILENAME: u32 = 1u32;
pub const MODEBIASMODE_READING: u32 = 2u32;
pub const MODEBIAS_GETVALUE: u32 = 2u32;
pub const MODEBIAS_GETVERSION: u32 = 0u32;
pub const MODEBIAS_SETVALUE: u32 = 1u32;
pub const MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
pub const MOD_LEFT: u32 = 32768u32;
pub const MOD_ON_KEYUP: u32 = 2048u32;
pub const MOD_RIGHT: u32 = 16384u32;
pub const NI_CHANGECANDIDATELIST: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(19u32);
pub const NI_CLOSECANDIDATE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(17u32);
pub const NI_COMPOSITIONSTR: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(21u32);
pub const NI_CONTEXTUPDATED: u32 = 3u32;
pub const NI_FINALIZECONVERSIONRESULT: u32 = 20u32;
pub const NI_IMEMENUSELECTED: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(24u32);
pub const NI_OPENCANDIDATE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(16u32);
pub const NI_SELECTCANDIDATESTR: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(18u32);
pub const NI_SETCANDIDATE_PAGESIZE: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(23u32);
pub const NI_SETCANDIDATE_PAGESTART: NOTIFY_IME_ACTION = NOTIFY_IME_ACTION(22u32);
pub const POS_UNDEFINED: u32 = 0u32;
pub const RECONVOPT_NONE: u32 = 0u32;
pub const RECONVOPT_USECANCELNOTIFY: u32 = 1u32;
pub const RWM_CHGKEYMAP: windows_core::PCWSTR = windows_core::w!("MSIMEChangeKeyMap");
pub const RWM_DOCUMENTFEED: windows_core::PCWSTR = windows_core::w!("MSIMEDocumentFeed");
pub const RWM_KEYMAP: windows_core::PCWSTR = windows_core::w!("MSIMEKeyMap");
pub const RWM_MODEBIAS: windows_core::PCWSTR = windows_core::w!("MSIMEModeBias");
pub const RWM_MOUSE: windows_core::PCWSTR = windows_core::w!("MSIMEMouseOperation");
pub const RWM_NTFYKEYMAP: windows_core::PCWSTR = windows_core::w!("MSIMENotifyKeyMap");
pub const RWM_QUERYPOSITION: windows_core::PCWSTR = windows_core::w!("MSIMEQueryPosition");
pub const RWM_RECONVERT: windows_core::PCWSTR = windows_core::w!("MSIMEReconvert");
pub const RWM_RECONVERTOPTIONS: windows_core::PCWSTR = windows_core::w!("MSIMEReconvertOptions");
pub const RWM_RECONVERTREQUEST: windows_core::PCWSTR = windows_core::w!("MSIMEReconvertRequest");
pub const RWM_SERVICE: windows_core::PCWSTR = windows_core::w!("MSIMEService");
pub const RWM_SHOWIMEPAD: windows_core::PCWSTR = windows_core::w!("MSIMEShowImePad");
pub const RWM_UIREADY: windows_core::PCWSTR = windows_core::w!("MSIMEUIReady");
pub const SCS_CAP_COMPSTR: u32 = 1u32;
pub const SCS_CAP_MAKEREAD: u32 = 2u32;
pub const SCS_CAP_SETRECONVERTSTRING: u32 = 4u32;
pub const SCS_CHANGEATTR: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(18u32);
pub const SCS_CHANGECLAUSE: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(36u32);
pub const SCS_QUERYRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(131072u32);
pub const SCS_SETRECONVERTSTRING: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(65536u32);
pub const SCS_SETSTR: SET_COMPOSITION_STRING_TYPE = SET_COMPOSITION_STRING_TYPE(9u32);
pub const SELECT_CAP_CONVERSION: u32 = 1u32;
pub const SELECT_CAP_SENTENCE: u32 = 2u32;
pub const SHOWIMEPAD_CATEGORY: u32 = 1u32;
pub const SHOWIMEPAD_DEFAULT: u32 = 0u32;
pub const SHOWIMEPAD_GUID: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1u32;
pub const STYLE_DESCRIPTION_SIZE: u32 = 32u32;
pub const UI_CAP_2700: u32 = 1u32;
pub const UI_CAP_ROT90: u32 = 2u32;
pub const UI_CAP_ROTANY: u32 = 4u32;
pub const UI_CAP_SOFTKBD: u32 = 65536u32;
pub const VERSION_DOCUMENTFEED: u32 = 1u32;
pub const VERSION_ID_CHINESE_SIMPLIFIED: u32 = 134217728u32;
pub const VERSION_ID_CHINESE_TRADITIONAL: u32 = 67108864u32;
pub const VERSION_ID_JAPANESE: u32 = 16777216u32;
pub const VERSION_ID_KOREAN: u32 = 33554432u32;
pub const VERSION_MODEBIAS: u32 = 1u32;
pub const VERSION_MOUSE_OPERATION: u32 = 1u32;
pub const VERSION_QUERYPOSITION: u32 = 1u32;
pub const VERSION_RECONVERSION: u32 = 1u32;
pub const cbCommentMax: u32 = 256u32;
pub const szImeChina: windows_core::PCWSTR = windows_core::w!("MSIME.China");
pub const szImeJapan: windows_core::PCWSTR = windows_core::w!("MSIME.Japan");
pub const szImeKorea: windows_core::PCWSTR = windows_core::w!("MSIME.Korea");
pub const szImeTaiwan: windows_core::PCWSTR = windows_core::w!("MSIME.Taiwan");
pub const wchPrivate1: u32 = 57344u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_CONVERSION_LIST_FLAG(pub u32);
impl windows_core::TypeKind for GET_CONVERSION_LIST_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_CONVERSION_LIST_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_CONVERSION_LIST_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GET_GUIDE_LINE_TYPE(pub u32);
impl windows_core::TypeKind for GET_GUIDE_LINE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GET_GUIDE_LINE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GET_GUIDE_LINE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMEFMT(pub i32);
impl windows_core::TypeKind for IMEFMT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMEFMT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMEFMT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMEREG(pub i32);
impl windows_core::TypeKind for IMEREG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMEREG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMEREG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMEREL(pub i32);
impl windows_core::TypeKind for IMEREL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMEREL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMEREL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMEUCT(pub i32);
impl windows_core::TypeKind for IMEUCT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMEUCT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMEUCT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IME_COMPOSITION_STRING(pub u32);
impl windows_core::TypeKind for IME_COMPOSITION_STRING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IME_COMPOSITION_STRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IME_COMPOSITION_STRING").field(&self.0).finish()
    }
}
impl IME_COMPOSITION_STRING {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IME_COMPOSITION_STRING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IME_COMPOSITION_STRING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IME_COMPOSITION_STRING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IME_COMPOSITION_STRING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IME_COMPOSITION_STRING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IME_CONVERSION_MODE(pub u32);
impl windows_core::TypeKind for IME_CONVERSION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IME_CONVERSION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IME_CONVERSION_MODE").field(&self.0).finish()
    }
}
impl IME_CONVERSION_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IME_CONVERSION_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IME_CONVERSION_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IME_CONVERSION_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IME_CONVERSION_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IME_CONVERSION_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IME_ESCAPE(pub u32);
impl windows_core::TypeKind for IME_ESCAPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IME_ESCAPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IME_ESCAPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IME_HOTKEY_IDENTIFIER(pub u32);
impl windows_core::TypeKind for IME_HOTKEY_IDENTIFIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IME_HOTKEY_IDENTIFIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IME_HOTKEY_IDENTIFIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IME_PAD_REQUEST_FLAGS(pub u32);
impl windows_core::TypeKind for IME_PAD_REQUEST_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IME_PAD_REQUEST_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IME_PAD_REQUEST_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IME_SENTENCE_MODE(pub u32);
impl windows_core::TypeKind for IME_SENTENCE_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IME_SENTENCE_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IME_SENTENCE_MODE").field(&self.0).finish()
    }
}
impl IME_SENTENCE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IME_SENTENCE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IME_SENTENCE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IME_SENTENCE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IME_SENTENCE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IME_SENTENCE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NOTIFY_IME_ACTION(pub u32);
impl windows_core::TypeKind for NOTIFY_IME_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NOTIFY_IME_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NOTIFY_IME_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NOTIFY_IME_INDEX(pub u32);
impl windows_core::TypeKind for NOTIFY_IME_INDEX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NOTIFY_IME_INDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NOTIFY_IME_INDEX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SET_COMPOSITION_STRING_TYPE(pub u32);
impl windows_core::TypeKind for SET_COMPOSITION_STRING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SET_COMPOSITION_STRING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SET_COMPOSITION_STRING_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPLETIDLIST {
    pub count: i32,
    pub pIIDList: *mut windows_core::GUID,
}
impl windows_core::TypeKind for APPLETIDLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPLETIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct APPLYCANDEXPARAM {
    pub dwSize: u32,
    pub lpwstrDisplay: windows_core::PWSTR,
    pub lpwstrReading: windows_core::PWSTR,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for APPLYCANDEXPARAM {
    type TypeKind = windows_core::CopyType;
}
impl Default for APPLYCANDEXPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
impl windows_core::TypeKind for CANDIDATEFORM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CANDIDATEFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CANDIDATEINFO {
    pub dwSize: u32,
    pub dwCount: u32,
    pub dwOffset: [u32; 32],
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl windows_core::TypeKind for CANDIDATEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CANDIDATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CANDIDATELIST {
    pub dwSize: u32,
    pub dwStyle: u32,
    pub dwCount: u32,
    pub dwSelection: u32,
    pub dwPageStart: u32,
    pub dwPageSize: u32,
    pub dwOffset: [u32; 1],
}
impl windows_core::TypeKind for CANDIDATELIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CActiveIMM: windows_core::GUID = windows_core::GUID::from_u128(0x4955dd33_b159_11d0_8fcf_00aa006bcc59);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
impl windows_core::TypeKind for COMPOSITIONFORM {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPOSITIONFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMPOSITIONSTRING {
    pub dwSize: u32,
    pub dwCompReadAttrLen: u32,
    pub dwCompReadAttrOffset: u32,
    pub dwCompReadClauseLen: u32,
    pub dwCompReadClauseOffset: u32,
    pub dwCompReadStrLen: u32,
    pub dwCompReadStrOffset: u32,
    pub dwCompAttrLen: u32,
    pub dwCompAttrOffset: u32,
    pub dwCompClauseLen: u32,
    pub dwCompClauseOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwCursorPos: u32,
    pub dwDeltaStart: u32,
    pub dwResultReadClauseLen: u32,
    pub dwResultReadClauseOffset: u32,
    pub dwResultReadStrLen: u32,
    pub dwResultReadStrOffset: u32,
    pub dwResultClauseLen: u32,
    pub dwResultClauseOffset: u32,
    pub dwResultStrLen: u32,
    pub dwResultStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl windows_core::TypeKind for COMPOSITIONSTRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMPOSITIONSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GUIDELINE {
    pub dwSize: u32,
    pub dwLevel: u32,
    pub dwIndex: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl windows_core::TypeKind for GUIDELINE {
    type TypeKind = windows_core::CopyType;
}
impl Default for GUIDELINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMC(pub *mut core::ffi::c_void);
impl HIMC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HIMC {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            _ = ImmDestroyContext(*self);
        }
    }
}
impl Default for HIMC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HIMC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HIMCC(pub *mut core::ffi::c_void);
impl HIMCC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HIMCC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HIMCC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEAPPLETCFG {
    pub dwConfig: u32,
    pub wchTitle: [u16; 64],
    pub wchTitleFontFace: [u16; 32],
    pub dwCharSet: u32,
    pub iCategory: i32,
    pub hIcon: super::super::WindowsAndMessaging::HICON,
    pub langID: u16,
    pub dummy: u16,
    pub lReserved1: super::super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for IMEAPPLETCFG {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for IMEAPPLETCFG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEAPPLETUI {
    pub hwnd: super::super::super::Foundation::HWND,
    pub dwStyle: u32,
    pub width: i32,
    pub height: i32,
    pub minWidth: i32,
    pub minHeight: i32,
    pub maxWidth: i32,
    pub maxHeight: i32,
    pub lReserved1: super::super::super::Foundation::LPARAM,
    pub lReserved2: super::super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for IMEAPPLETUI {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEAPPLETUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMECHARINFO {
    pub wch: u16,
    pub dwCharInfo: u32,
}
impl windows_core::TypeKind for IMECHARINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMECHARINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::super::super::Foundation::RECT,
}
impl windows_core::TypeKind for IMECHARPOSITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMECHARPOSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMECOMPOSITIONSTRINGINFO {
    pub iCompStrLen: i32,
    pub iCaretPos: i32,
    pub iEditStart: i32,
    pub iEditLen: i32,
    pub iTargetStart: i32,
    pub iTargetLen: i32,
}
impl windows_core::TypeKind for IMECOMPOSITIONSTRINGINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMECOMPOSITIONSTRINGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEDLG {
    pub cbIMEDLG: i32,
    pub hwnd: super::super::super::Foundation::HWND,
    pub lpwstrWord: windows_core::PWSTR,
    pub nTabId: i32,
}
impl windows_core::TypeKind for IMEDLG {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEDLG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEDP {
    pub wrdModifier: IMEWRD,
    pub wrdModifiee: IMEWRD,
    pub relID: IMEREL,
}
impl windows_core::TypeKind for IMEDP {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEDP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEFAREASTINFO {
    pub dwSize: u32,
    pub dwType: u32,
    pub dwData: [u32; 1],
}
impl windows_core::TypeKind for IMEFAREASTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEFAREASTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEINFO {
    pub dwPrivateDataSize: u32,
    pub fdwProperty: u32,
    pub fdwConversionCaps: u32,
    pub fdwSentenceCaps: u32,
    pub fdwUICaps: u32,
    pub fdwSCSCaps: u32,
    pub fdwSelectCaps: u32,
}
impl windows_core::TypeKind for IMEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEITEM {
    pub cbSize: i32,
    pub iType: i32,
    pub lpItemData: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for IMEITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEITEMCANDIDATE {
    pub uCount: u32,
    pub imeItem: [IMEITEM; 1],
}
impl windows_core::TypeKind for IMEITEMCANDIDATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEITEMCANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMS {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub cKeyList: u32,
    pub pKeyList: *mut IMEKMSKEY,
}
impl windows_core::TypeKind for IMEKMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMSFUNCDESC {
    pub cbSize: i32,
    pub idLang: u16,
    pub dwControl: u32,
    pub pwszDescription: [u16; 128],
}
impl windows_core::TypeKind for IMEKMSFUNCDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSFUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMSINIT {
    pub cbSize: i32,
    pub hWnd: super::super::super::Foundation::HWND,
}
impl windows_core::TypeKind for IMEKMSINIT {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSINIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMSINVK {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub dwControl: u32,
}
impl windows_core::TypeKind for IMEKMSINVK {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSINVK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMSKEY {
    pub dwStatus: u32,
    pub dwCompStatus: u32,
    pub dwVKEY: u32,
    pub Anonymous1: IMEKMSKEY_0,
    pub Anonymous2: IMEKMSKEY_1,
}
impl windows_core::TypeKind for IMEKMSKEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union IMEKMSKEY_0 {
    pub dwControl: u32,
    pub dwNotUsed: u32,
}
impl windows_core::TypeKind for IMEKMSKEY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSKEY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union IMEKMSKEY_1 {
    pub pwszDscr: [u16; 31],
    pub pwszNoUse: [u16; 31],
}
impl windows_core::TypeKind for IMEKMSKEY_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSKEY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMSKMP {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub idLang: u16,
    pub wVKStart: u16,
    pub wVKEnd: u16,
    pub cKeyList: i32,
    pub pKeyList: *mut IMEKMSKEY,
}
impl windows_core::TypeKind for IMEKMSKMP {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSKMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMSNTFY {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub fSelect: super::super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for IMEKMSNTFY {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEKMSNTFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEMENUITEMINFOA {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [i8; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for IMEMENUITEMINFOA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMEMENUITEMINFOW {
    pub cbSize: u32,
    pub fType: u32,
    pub fState: u32,
    pub wID: u32,
    pub hbmpChecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub hbmpUnchecked: super::super::super::Graphics::Gdi::HBITMAP,
    pub dwItemData: u32,
    pub szString: [u16; 80],
    pub hbmpItem: super::super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for IMEMENUITEMINFOW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMESHF {
    pub cbShf: u16,
    pub verDic: u16,
    pub szTitle: [i8; 48],
    pub szDescription: [i8; 256],
    pub szCopyright: [i8; 128],
}
impl windows_core::TypeKind for IMESHF {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMESHF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMESTRINGCANDIDATE {
    pub uCount: u32,
    pub lpwstr: [windows_core::PWSTR; 1],
}
impl windows_core::TypeKind for IMESTRINGCANDIDATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMESTRINGCANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMESTRINGCANDIDATEINFO {
    pub dwFarEastId: u32,
    pub lpFarEastInfo: *mut IMEFAREASTINFO,
    pub fInfoMask: u32,
    pub iSelIndex: i32,
    pub uCount: u32,
    pub lpwstr: [windows_core::PWSTR; 1],
}
impl windows_core::TypeKind for IMESTRINGCANDIDATEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMESTRINGCANDIDATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IMESTRINGINFO {
    pub dwFarEastId: u32,
    pub lpwstr: windows_core::PWSTR,
}
impl windows_core::TypeKind for IMESTRINGINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMESTRINGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEWRD {
    pub pwchReading: windows_core::PWSTR,
    pub pwchDisplay: windows_core::PWSTR,
    pub Anonymous: IMEWRD_0,
    pub rgulAttrs: [u32; 2],
    pub cbComment: i32,
    pub uct: IMEUCT,
    pub pvComment: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for IMEWRD {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEWRD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union IMEWRD_0 {
    pub ulPos: u32,
    pub Anonymous: IMEWRD_0_0,
}
impl windows_core::TypeKind for IMEWRD_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEWRD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEWRD_0_0 {
    pub nPos1: u16,
    pub nPos2: u16,
}
impl windows_core::TypeKind for IMEWRD_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IMEWRD_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct INPUTCONTEXT {
    pub hWnd: super::super::super::Foundation::HWND,
    pub fOpen: super::super::super::Foundation::BOOL,
    pub ptStatusWndPos: super::super::super::Foundation::POINT,
    pub ptSoftKbdPos: super::super::super::Foundation::POINT,
    pub fdwConversion: u32,
    pub fdwSentence: u32,
    pub lfFont: INPUTCONTEXT_0,
    pub cfCompForm: COMPOSITIONFORM,
    pub cfCandForm: [CANDIDATEFORM; 4],
    pub hCompStr: HIMCC,
    pub hCandInfo: HIMCC,
    pub hGuideLine: HIMCC,
    pub hPrivate: HIMCC,
    pub dwNumMsgBuf: u32,
    pub hMsgBuf: HIMCC,
    pub fdwInit: u32,
    pub dwReserve: [u32; 3],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for INPUTCONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for INPUTCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union INPUTCONTEXT_0 {
    pub A: super::super::super::Graphics::Gdi::LOGFONTA,
    pub W: super::super::super::Graphics::Gdi::LOGFONTW,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for INPUTCONTEXT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for INPUTCONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MORRSLT {
    pub dwSize: u32,
    pub pwchOutput: windows_core::PWSTR,
    pub cchOutput: u16,
    pub Anonymous1: MORRSLT_0,
    pub Anonymous2: MORRSLT_1,
    pub pchInputPos: *mut u16,
    pub pchOutputIdxWDD: *mut u16,
    pub Anonymous3: MORRSLT_2,
    pub paMonoRubyPos: *mut u16,
    pub pWDD: *mut WDD,
    pub cWDD: i32,
    pub pPrivate: *mut core::ffi::c_void,
    pub BLKBuff: [u16; 1],
}
impl windows_core::TypeKind for MORRSLT {
    type TypeKind = windows_core::CopyType;
}
impl Default for MORRSLT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MORRSLT_0 {
    pub pwchRead: windows_core::PWSTR,
    pub pwchComp: windows_core::PWSTR,
}
impl windows_core::TypeKind for MORRSLT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MORRSLT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MORRSLT_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl windows_core::TypeKind for MORRSLT_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MORRSLT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MORRSLT_2 {
    pub pchReadIdxWDD: *mut u16,
    pub pchCompIdxWDD: *mut u16,
}
impl windows_core::TypeKind for MORRSLT_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MORRSLT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct POSTBL {
    pub nPos: u16,
    pub szName: *mut u8,
}
impl windows_core::TypeKind for POSTBL {
    type TypeKind = windows_core::CopyType;
}
impl Default for POSTBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RECONVERTSTRING {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwCompStrLen: u32,
    pub dwCompStrOffset: u32,
    pub dwTargetStrLen: u32,
    pub dwTargetStrOffset: u32,
}
impl windows_core::TypeKind for RECONVERTSTRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for RECONVERTSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REGISTERWORDA {
    pub lpReading: windows_core::PSTR,
    pub lpWord: windows_core::PSTR,
}
impl windows_core::TypeKind for REGISTERWORDA {
    type TypeKind = windows_core::CopyType;
}
impl Default for REGISTERWORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REGISTERWORDW {
    pub lpReading: windows_core::PWSTR,
    pub lpWord: windows_core::PWSTR,
}
impl windows_core::TypeKind for REGISTERWORDW {
    type TypeKind = windows_core::CopyType;
}
impl Default for REGISTERWORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOFTKBDDATA {
    pub uCount: u32,
    pub wCode: [u16; 256],
}
impl windows_core::TypeKind for SOFTKBDDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOFTKBDDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STYLEBUFA {
    pub dwStyle: u32,
    pub szDescription: [i8; 32],
}
impl windows_core::TypeKind for STYLEBUFA {
    type TypeKind = windows_core::CopyType;
}
impl Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl windows_core::TypeKind for STYLEBUFW {
    type TypeKind = windows_core::CopyType;
}
impl Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSMSG {
    pub message: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
impl windows_core::TypeKind for TRANSMSG {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSMSGLIST {
    pub uMsgCount: u32,
    pub TransMsg: [TRANSMSG; 1],
}
impl windows_core::TypeKind for TRANSMSGLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSMSGLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WDD {
    pub wDispPos: u16,
    pub Anonymous1: WDD_0,
    pub cchDisp: u16,
    pub Anonymous2: WDD_1,
    pub WDD_nReserve1: u32,
    pub nPos: u16,
    pub _bitfield: u16,
    pub pReserved: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for WDD {
    type TypeKind = windows_core::CopyType;
}
impl Default for WDD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WDD_0 {
    pub wReadPos: u16,
    pub wCompPos: u16,
}
impl windows_core::TypeKind for WDD_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WDD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WDD_1 {
    pub cchRead: u16,
    pub cchComp: u16,
}
impl windows_core::TypeKind for WDD_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WDD_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMCENUMPROC = Option<unsafe extern "system" fn(param0: HIMC, param1: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::BOOL>;
pub type PFNLOG = Option<unsafe extern "system" fn(param0: *mut IMEDP, param1: windows_core::HRESULT) -> super::super::super::Foundation::BOOL>;
pub type REGISTERWORDENUMPROCA = Option<unsafe extern "system" fn(lpszreading: windows_core::PCSTR, param1: u32, lpszstring: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> i32>;
pub type REGISTERWORDENUMPROCW = Option<unsafe extern "system" fn(lpszreading: windows_core::PCWSTR, param1: u32, lpszstring: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> i32>;
pub type fpCreateIFECommonInstanceType = Option<unsafe extern "system" fn(ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type fpCreateIFEDictionaryInstanceType = Option<unsafe extern "system" fn(ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type fpCreateIFELanguageInstanceType = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
