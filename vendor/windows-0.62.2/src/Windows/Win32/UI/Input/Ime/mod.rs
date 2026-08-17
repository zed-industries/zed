#[inline]
pub unsafe fn ImmAssociateContext(param0: super::super::super::Foundation::HWND, param1: HIMC) -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmAssociateContext(param0 : super::super::super::Foundation:: HWND, param1 : HIMC) -> HIMC);
    unsafe { ImmAssociateContext(param0, param1) }
}
#[inline]
pub unsafe fn ImmAssociateContextEx(param0: super::super::super::Foundation::HWND, param1: HIMC, param2: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmAssociateContextEx(param0 : super::super::super::Foundation:: HWND, param1 : HIMC, param2 : u32) -> windows_core::BOOL);
    unsafe { ImmAssociateContextEx(param0, param1, param2) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmConfigureIMEA(param0: super::KeyboardAndMouse::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmConfigureIMEA(param0 : super::KeyboardAndMouse:: HKL, param1 : super::super::super::Foundation:: HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImmConfigureIMEA(param0, param1, param2, param3 as _) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmConfigureIMEW(param0: super::KeyboardAndMouse::HKL, param1: super::super::super::Foundation::HWND, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmConfigureIMEW(param0 : super::KeyboardAndMouse:: HKL, param1 : super::super::super::Foundation:: HWND, param2 : u32, param3 : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ImmConfigureIMEW(param0, param1, param2, param3 as _) }
}
#[inline]
pub unsafe fn ImmCreateContext() -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmCreateContext() -> HIMC);
    unsafe { ImmCreateContext() }
}
#[inline]
pub unsafe fn ImmCreateIMCC(param0: u32) -> HIMCC {
    windows_core::link!("imm32.dll" "system" fn ImmCreateIMCC(param0 : u32) -> HIMCC);
    unsafe { ImmCreateIMCC(param0) }
}
#[inline]
pub unsafe fn ImmCreateSoftKeyboard(param0: u32, param1: super::super::super::Foundation::HWND, param2: i32, param3: i32) -> super::super::super::Foundation::HWND {
    windows_core::link!("imm32.dll" "system" fn ImmCreateSoftKeyboard(param0 : u32, param1 : super::super::super::Foundation:: HWND, param2 : i32, param3 : i32) -> super::super::super::Foundation:: HWND);
    unsafe { ImmCreateSoftKeyboard(param0, param1, param2, param3) }
}
#[inline]
pub unsafe fn ImmDestroyContext(param0: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDestroyContext(param0 : HIMC) -> windows_core::BOOL);
    unsafe { ImmDestroyContext(param0) }
}
#[inline]
pub unsafe fn ImmDestroyIMCC(param0: HIMCC) -> HIMCC {
    windows_core::link!("imm32.dll" "system" fn ImmDestroyIMCC(param0 : HIMCC) -> HIMCC);
    unsafe { ImmDestroyIMCC(param0) }
}
#[inline]
pub unsafe fn ImmDestroySoftKeyboard(param0: super::super::super::Foundation::HWND) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDestroySoftKeyboard(param0 : super::super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { ImmDestroySoftKeyboard(param0) }
}
#[inline]
pub unsafe fn ImmDisableIME(param0: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDisableIME(param0 : u32) -> windows_core::BOOL);
    unsafe { ImmDisableIME(param0) }
}
#[inline]
pub unsafe fn ImmDisableLegacyIME() -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDisableLegacyIME() -> windows_core::BOOL);
    unsafe { ImmDisableLegacyIME() }
}
#[inline]
pub unsafe fn ImmDisableTextFrameService(idthread: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmDisableTextFrameService(idthread : u32) -> windows_core::BOOL);
    unsafe { ImmDisableTextFrameService(idthread) }
}
#[inline]
pub unsafe fn ImmEnumInputContext(idthread: u32, lpfn: IMCENUMPROC, lparam: super::super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmEnumInputContext(idthread : u32, lpfn : IMCENUMPROC, lparam : super::super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { ImmEnumInputContext(idthread, lpfn, lparam) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEnumRegisterWordA<P2, P4>(param0: super::KeyboardAndMouse::HKL, param1: REGISTERWORDENUMPROCA, lpszreading: P2, param3: u32, lpszregister: P4, param5: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmEnumRegisterWordA(param0 : super::KeyboardAndMouse:: HKL, param1 : REGISTERWORDENUMPROCA, lpszreading : windows_core::PCSTR, param3 : u32, lpszregister : windows_core::PCSTR, param5 : *mut core::ffi::c_void) -> u32);
    unsafe { ImmEnumRegisterWordA(param0, param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5 as _) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEnumRegisterWordW<P2, P4>(param0: super::KeyboardAndMouse::HKL, param1: REGISTERWORDENUMPROCW, lpszreading: P2, param3: u32, lpszregister: P4, param5: *mut core::ffi::c_void) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmEnumRegisterWordW(param0 : super::KeyboardAndMouse:: HKL, param1 : REGISTERWORDENUMPROCW, lpszreading : windows_core::PCWSTR, param3 : u32, lpszregister : windows_core::PCWSTR, param5 : *mut core::ffi::c_void) -> u32);
    unsafe { ImmEnumRegisterWordW(param0, param1, lpszreading.param().abi(), param3, lpszregister.param().abi(), param5 as _) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEscapeA(param0: super::KeyboardAndMouse::HKL, param1: HIMC, param2: IME_ESCAPE, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmEscapeA(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, param2 : IME_ESCAPE, param3 : *mut core::ffi::c_void) -> super::super::super::Foundation:: LRESULT);
    unsafe { ImmEscapeA(param0, param1, param2, param3 as _) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmEscapeW(param0: super::KeyboardAndMouse::HKL, param1: HIMC, param2: IME_ESCAPE, param3: *mut core::ffi::c_void) -> super::super::super::Foundation::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmEscapeW(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, param2 : IME_ESCAPE, param3 : *mut core::ffi::c_void) -> super::super::super::Foundation:: LRESULT);
    unsafe { ImmEscapeW(param0, param1, param2, param3 as _) }
}
#[inline]
pub unsafe fn ImmGenerateMessage(param0: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGenerateMessage(param0 : HIMC) -> windows_core::BOOL);
    unsafe { ImmGenerateMessage(param0) }
}
#[inline]
pub unsafe fn ImmGetCandidateListA(param0: HIMC, deindex: u32, lpcandlist: Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListA(param0 : HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    unsafe { ImmGetCandidateListA(param0, deindex, lpcandlist.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[inline]
pub unsafe fn ImmGetCandidateListCountA(param0: HIMC, lpdwlistcount: *mut u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListCountA(param0 : HIMC, lpdwlistcount : *mut u32) -> u32);
    unsafe { ImmGetCandidateListCountA(param0, lpdwlistcount as _) }
}
#[inline]
pub unsafe fn ImmGetCandidateListCountW(param0: HIMC, lpdwlistcount: *mut u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListCountW(param0 : HIMC, lpdwlistcount : *mut u32) -> u32);
    unsafe { ImmGetCandidateListCountW(param0, lpdwlistcount as _) }
}
#[inline]
pub unsafe fn ImmGetCandidateListW(param0: HIMC, deindex: u32, lpcandlist: Option<*mut CANDIDATELIST>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateListW(param0 : HIMC, deindex : u32, lpcandlist : *mut CANDIDATELIST, dwbuflen : u32) -> u32);
    unsafe { ImmGetCandidateListW(param0, deindex, lpcandlist.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[inline]
pub unsafe fn ImmGetCandidateWindow(param0: HIMC, param1: u32, lpcandidate: *mut CANDIDATEFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCandidateWindow(param0 : HIMC, param1 : u32, lpcandidate : *mut CANDIDATEFORM) -> windows_core::BOOL);
    unsafe { ImmGetCandidateWindow(param0, param1, lpcandidate as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontA(param0: HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionFontA(param0 : HIMC, lplf : *mut super::super::super::Graphics::Gdi:: LOGFONTA) -> windows_core::BOOL);
    unsafe { ImmGetCompositionFontA(param0, lplf as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetCompositionFontW(param0: HIMC, lplf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionFontW(param0 : HIMC, lplf : *mut super::super::super::Graphics::Gdi:: LOGFONTW) -> windows_core::BOOL);
    unsafe { ImmGetCompositionFontW(param0, lplf as _) }
}
#[inline]
pub unsafe fn ImmGetCompositionStringA(param0: HIMC, param1: IME_COMPOSITION_STRING, lpbuf: Option<*mut core::ffi::c_void>, dwbuflen: u32) -> i32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionStringA(param0 : HIMC, param1 : IME_COMPOSITION_STRING, lpbuf : *mut core::ffi::c_void, dwbuflen : u32) -> i32);
    unsafe { ImmGetCompositionStringA(param0, param1, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[inline]
pub unsafe fn ImmGetCompositionStringW(param0: HIMC, param1: IME_COMPOSITION_STRING, lpbuf: Option<*mut core::ffi::c_void>, dwbuflen: u32) -> i32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionStringW(param0 : HIMC, param1 : IME_COMPOSITION_STRING, lpbuf : *mut core::ffi::c_void, dwbuflen : u32) -> i32);
    unsafe { ImmGetCompositionStringW(param0, param1, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[inline]
pub unsafe fn ImmGetCompositionWindow(param0: HIMC, lpcompform: *mut COMPOSITIONFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetCompositionWindow(param0 : HIMC, lpcompform : *mut COMPOSITIONFORM) -> windows_core::BOOL);
    unsafe { ImmGetCompositionWindow(param0, lpcompform as _) }
}
#[inline]
pub unsafe fn ImmGetContext(param0: super::super::super::Foundation::HWND) -> HIMC {
    windows_core::link!("imm32.dll" "system" fn ImmGetContext(param0 : super::super::super::Foundation:: HWND) -> HIMC);
    unsafe { ImmGetContext(param0) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetConversionListA<P2>(param0: super::KeyboardAndMouse::HKL, param1: HIMC, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionListA(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, lpsrc : windows_core::PCSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : GET_CONVERSION_LIST_FLAG) -> u32);
    unsafe { ImmGetConversionListA(param0, param1, lpsrc.param().abi(), lpdst as _, dwbuflen, uflag) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetConversionListW<P2>(param0: super::KeyboardAndMouse::HKL, param1: HIMC, lpsrc: P2, lpdst: *mut CANDIDATELIST, dwbuflen: u32, uflag: GET_CONVERSION_LIST_FLAG) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionListW(param0 : super::KeyboardAndMouse:: HKL, param1 : HIMC, lpsrc : windows_core::PCWSTR, lpdst : *mut CANDIDATELIST, dwbuflen : u32, uflag : GET_CONVERSION_LIST_FLAG) -> u32);
    unsafe { ImmGetConversionListW(param0, param1, lpsrc.param().abi(), lpdst as _, dwbuflen, uflag) }
}
#[inline]
pub unsafe fn ImmGetConversionStatus(param0: HIMC, lpfdwconversion: Option<*mut IME_CONVERSION_MODE>, lpfdwsentence: Option<*mut IME_SENTENCE_MODE>) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetConversionStatus(param0 : HIMC, lpfdwconversion : *mut IME_CONVERSION_MODE, lpfdwsentence : *mut IME_SENTENCE_MODE) -> windows_core::BOOL);
    unsafe { ImmGetConversionStatus(param0, lpfdwconversion.unwrap_or(core::mem::zeroed()) as _, lpfdwsentence.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ImmGetDefaultIMEWnd(param0: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND {
    windows_core::link!("imm32.dll" "system" fn ImmGetDefaultIMEWnd(param0 : super::super::super::Foundation:: HWND) -> super::super::super::Foundation:: HWND);
    unsafe { ImmGetDefaultIMEWnd(param0) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetDescriptionA(param0: super::KeyboardAndMouse::HKL, lpszdescription: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetDescriptionA(param0 : super::KeyboardAndMouse:: HKL, lpszdescription : windows_core::PSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetDescriptionA(param0, core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetDescriptionW(param0: super::KeyboardAndMouse::HKL, lpszdescription: Option<&mut [u16]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetDescriptionW(param0 : super::KeyboardAndMouse:: HKL, lpszdescription : windows_core::PWSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetDescriptionW(param0, core::mem::transmute(lpszdescription.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszdescription.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ImmGetGuideLineA(param0: HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetGuideLineA(param0 : HIMC, dwindex : GET_GUIDE_LINE_TYPE, lpbuf : windows_core::PSTR, dwbuflen : u32) -> u32);
    unsafe { ImmGetGuideLineA(param0, dwindex, core::mem::transmute(lpbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ImmGetGuideLineW(param0: HIMC, dwindex: GET_GUIDE_LINE_TYPE, lpbuf: Option<windows_core::PWSTR>, dwbuflen: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetGuideLineW(param0 : HIMC, dwindex : GET_GUIDE_LINE_TYPE, lpbuf : windows_core::PWSTR, dwbuflen : u32) -> u32);
    unsafe { ImmGetGuideLineW(param0, dwindex, lpbuf.unwrap_or(core::mem::zeroed()) as _, dwbuflen) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetHotKey(param0: u32, lpumodifiers: *mut u32, lpuvkey: *mut u32, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetHotKey(param0 : u32, lpumodifiers : *mut u32, lpuvkey : *mut u32, phkl : *mut super::KeyboardAndMouse:: HKL) -> windows_core::BOOL);
    unsafe { ImmGetHotKey(param0, lpumodifiers as _, lpuvkey as _, phkl as _) }
}
#[inline]
pub unsafe fn ImmGetIMCCLockCount(param0: HIMCC) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMCCLockCount(param0 : HIMCC) -> u32);
    unsafe { ImmGetIMCCLockCount(param0) }
}
#[inline]
pub unsafe fn ImmGetIMCCSize(param0: HIMCC) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMCCSize(param0 : HIMCC) -> u32);
    unsafe { ImmGetIMCCSize(param0) }
}
#[inline]
pub unsafe fn ImmGetIMCLockCount(param0: HIMC) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMCLockCount(param0 : HIMC) -> u32);
    unsafe { ImmGetIMCLockCount(param0) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetIMEFileNameA(param0: super::KeyboardAndMouse::HKL, lpszfilename: Option<&mut [u8]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMEFileNameA(param0 : super::KeyboardAndMouse:: HKL, lpszfilename : windows_core::PSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetIMEFileNameA(param0, core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetIMEFileNameW(param0: super::KeyboardAndMouse::HKL, lpszfilename: Option<&mut [u16]>) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetIMEFileNameW(param0 : super::KeyboardAndMouse:: HKL, lpszfilename : windows_core::PWSTR, ubuflen : u32) -> u32);
    unsafe { ImmGetIMEFileNameW(param0, core::mem::transmute(lpszfilename.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpszfilename.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetImeMenuItemsA(param0: HIMC, param1: u32, param2: u32, lpimeparentmenu: Option<*mut IMEMENUITEMINFOA>, lpimemenu: Option<*mut IMEMENUITEMINFOA>, dwsize: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetImeMenuItemsA(param0 : HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOA, lpimemenu : *mut IMEMENUITEMINFOA, dwsize : u32) -> u32);
    unsafe { ImmGetImeMenuItemsA(param0, param1, param2, lpimeparentmenu.unwrap_or(core::mem::zeroed()) as _, lpimemenu.unwrap_or(core::mem::zeroed()) as _, dwsize) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmGetImeMenuItemsW(param0: HIMC, param1: u32, param2: u32, lpimeparentmenu: Option<*mut IMEMENUITEMINFOW>, lpimemenu: Option<*mut IMEMENUITEMINFOW>, dwsize: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetImeMenuItemsW(param0 : HIMC, param1 : u32, param2 : u32, lpimeparentmenu : *mut IMEMENUITEMINFOW, lpimemenu : *mut IMEMENUITEMINFOW, dwsize : u32) -> u32);
    unsafe { ImmGetImeMenuItemsW(param0, param1, param2, lpimeparentmenu.unwrap_or(core::mem::zeroed()) as _, lpimemenu.unwrap_or(core::mem::zeroed()) as _, dwsize) }
}
#[inline]
pub unsafe fn ImmGetOpenStatus(param0: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetOpenStatus(param0 : HIMC) -> windows_core::BOOL);
    unsafe { ImmGetOpenStatus(param0) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetProperty(param0: super::KeyboardAndMouse::HKL, param1: u32) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetProperty(param0 : super::KeyboardAndMouse:: HKL, param1 : u32) -> u32);
    unsafe { ImmGetProperty(param0, param1) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleA(param0: super::KeyboardAndMouse::HKL, lpstylebuf: &mut [STYLEBUFA]) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleA(param0 : super::KeyboardAndMouse:: HKL, nitem : u32, lpstylebuf : *mut STYLEBUFA) -> u32);
    unsafe { ImmGetRegisterWordStyleA(param0, lpstylebuf.len().try_into().unwrap(), core::mem::transmute(lpstylebuf.as_ptr())) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmGetRegisterWordStyleW(param0: super::KeyboardAndMouse::HKL, lpstylebuf: &mut [STYLEBUFW]) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetRegisterWordStyleW(param0 : super::KeyboardAndMouse:: HKL, nitem : u32, lpstylebuf : *mut STYLEBUFW) -> u32);
    unsafe { ImmGetRegisterWordStyleW(param0, lpstylebuf.len().try_into().unwrap(), core::mem::transmute(lpstylebuf.as_ptr())) }
}
#[inline]
pub unsafe fn ImmGetStatusWindowPos(param0: HIMC, lpptpos: *mut super::super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmGetStatusWindowPos(param0 : HIMC, lpptpos : *mut super::super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { ImmGetStatusWindowPos(param0, lpptpos as _) }
}
#[inline]
pub unsafe fn ImmGetVirtualKey(param0: super::super::super::Foundation::HWND) -> u32 {
    windows_core::link!("imm32.dll" "system" fn ImmGetVirtualKey(param0 : super::super::super::Foundation:: HWND) -> u32);
    unsafe { ImmGetVirtualKey(param0) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmInstallIMEA<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::KeyboardAndMouse::HKL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmInstallIMEA(lpszimefilename : windows_core::PCSTR, lpszlayouttext : windows_core::PCSTR) -> super::KeyboardAndMouse:: HKL);
    unsafe { ImmInstallIMEA(lpszimefilename.param().abi(), lpszlayouttext.param().abi()) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmInstallIMEW<P0, P1>(lpszimefilename: P0, lpszlayouttext: P1) -> super::KeyboardAndMouse::HKL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmInstallIMEW(lpszimefilename : windows_core::PCWSTR, lpszlayouttext : windows_core::PCWSTR) -> super::KeyboardAndMouse:: HKL);
    unsafe { ImmInstallIMEW(lpszimefilename.param().abi(), lpszlayouttext.param().abi()) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmIsIME(param0: super::KeyboardAndMouse::HKL) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsIME(param0 : super::KeyboardAndMouse:: HKL) -> windows_core::BOOL);
    unsafe { ImmIsIME(param0) }
}
#[inline]
pub unsafe fn ImmIsUIMessageA(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsUIMessageA(param0 : super::super::super::Foundation:: HWND, param1 : u32, param2 : super::super::super::Foundation:: WPARAM, param3 : super::super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { ImmIsUIMessageA(param0, param1, param2, param3) }
}
#[inline]
pub unsafe fn ImmIsUIMessageW(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmIsUIMessageW(param0 : super::super::super::Foundation:: HWND, param1 : u32, param2 : super::super::super::Foundation:: WPARAM, param3 : super::super::super::Foundation:: LPARAM) -> windows_core::BOOL);
    unsafe { ImmIsUIMessageW(param0, param1, param2, param3) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmLockIMC(param0: HIMC) -> *mut INPUTCONTEXT {
    windows_core::link!("imm32.dll" "system" fn ImmLockIMC(param0 : HIMC) -> *mut INPUTCONTEXT);
    unsafe { ImmLockIMC(param0) }
}
#[inline]
pub unsafe fn ImmLockIMCC(param0: HIMCC) -> *mut core::ffi::c_void {
    windows_core::link!("imm32.dll" "system" fn ImmLockIMCC(param0 : HIMCC) -> *mut core::ffi::c_void);
    unsafe { ImmLockIMCC(param0) }
}
#[inline]
pub unsafe fn ImmNotifyIME(param0: HIMC, dwaction: NOTIFY_IME_ACTION, dwindex: NOTIFY_IME_INDEX, dwvalue: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmNotifyIME(param0 : HIMC, dwaction : NOTIFY_IME_ACTION, dwindex : NOTIFY_IME_INDEX, dwvalue : u32) -> windows_core::BOOL);
    unsafe { ImmNotifyIME(param0, dwaction, dwindex, dwvalue) }
}
#[inline]
pub unsafe fn ImmReSizeIMCC(param0: HIMCC, param1: u32) -> HIMCC {
    windows_core::link!("imm32.dll" "system" fn ImmReSizeIMCC(param0 : HIMCC, param1 : u32) -> HIMCC);
    unsafe { ImmReSizeIMCC(param0, param1) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmRegisterWordA<P1, P3>(param0: super::KeyboardAndMouse::HKL, lpszreading: P1, param2: u32, lpszregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmRegisterWordA(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszregister : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImmRegisterWordA(param0, lpszreading.param().abi(), param2, lpszregister.param().abi()) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmRegisterWordW<P1, P3>(param0: super::KeyboardAndMouse::HKL, lpszreading: P1, param2: u32, lpszregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmRegisterWordW(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszregister : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImmRegisterWordW(param0, lpszreading.param().abi(), param2, lpszregister.param().abi()) }
}
#[inline]
pub unsafe fn ImmReleaseContext(param0: super::super::super::Foundation::HWND, param1: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmReleaseContext(param0 : super::super::super::Foundation:: HWND, param1 : HIMC) -> windows_core::BOOL);
    unsafe { ImmReleaseContext(param0, param1) }
}
#[inline]
pub unsafe fn ImmRequestMessageA(param0: HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmRequestMessageA(param0 : HIMC, param1 : super::super::super::Foundation:: WPARAM, param2 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: LRESULT);
    unsafe { ImmRequestMessageA(param0, param1, param2) }
}
#[inline]
pub unsafe fn ImmRequestMessageW(param0: HIMC, param1: super::super::super::Foundation::WPARAM, param2: super::super::super::Foundation::LPARAM) -> super::super::super::Foundation::LRESULT {
    windows_core::link!("imm32.dll" "system" fn ImmRequestMessageW(param0 : HIMC, param1 : super::super::super::Foundation:: WPARAM, param2 : super::super::super::Foundation:: LPARAM) -> super::super::super::Foundation:: LRESULT);
    unsafe { ImmRequestMessageW(param0, param1, param2) }
}
#[inline]
pub unsafe fn ImmSetCandidateWindow(param0: HIMC, lpcandidate: *const CANDIDATEFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCandidateWindow(param0 : HIMC, lpcandidate : *const CANDIDATEFORM) -> windows_core::BOOL);
    unsafe { ImmSetCandidateWindow(param0, lpcandidate) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontA(param0: HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionFontA(param0 : HIMC, lplf : *const super::super::super::Graphics::Gdi:: LOGFONTA) -> windows_core::BOOL);
    unsafe { ImmSetCompositionFontA(param0, lplf) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImmSetCompositionFontW(param0: HIMC, lplf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionFontW(param0 : HIMC, lplf : *const super::super::super::Graphics::Gdi:: LOGFONTW) -> windows_core::BOOL);
    unsafe { ImmSetCompositionFontW(param0, lplf) }
}
#[inline]
pub unsafe fn ImmSetCompositionStringA(param0: HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: Option<*const core::ffi::c_void>, dwcomplen: u32, lpread: Option<*const core::ffi::c_void>, dwreadlen: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionStringA(param0 : HIMC, dwindex : SET_COMPOSITION_STRING_TYPE, lpcomp : *const core::ffi::c_void, dwcomplen : u32, lpread : *const core::ffi::c_void, dwreadlen : u32) -> windows_core::BOOL);
    unsafe { ImmSetCompositionStringA(param0, dwindex, lpcomp.unwrap_or(core::mem::zeroed()) as _, dwcomplen, lpread.unwrap_or(core::mem::zeroed()) as _, dwreadlen) }
}
#[inline]
pub unsafe fn ImmSetCompositionStringW(param0: HIMC, dwindex: SET_COMPOSITION_STRING_TYPE, lpcomp: Option<*const core::ffi::c_void>, dwcomplen: u32, lpread: Option<*const core::ffi::c_void>, dwreadlen: u32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionStringW(param0 : HIMC, dwindex : SET_COMPOSITION_STRING_TYPE, lpcomp : *const core::ffi::c_void, dwcomplen : u32, lpread : *const core::ffi::c_void, dwreadlen : u32) -> windows_core::BOOL);
    unsafe { ImmSetCompositionStringW(param0, dwindex, lpcomp.unwrap_or(core::mem::zeroed()) as _, dwcomplen, lpread.unwrap_or(core::mem::zeroed()) as _, dwreadlen) }
}
#[inline]
pub unsafe fn ImmSetCompositionWindow(param0: HIMC, lpcompform: *const COMPOSITIONFORM) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetCompositionWindow(param0 : HIMC, lpcompform : *const COMPOSITIONFORM) -> windows_core::BOOL);
    unsafe { ImmSetCompositionWindow(param0, lpcompform) }
}
#[inline]
pub unsafe fn ImmSetConversionStatus(param0: HIMC, param1: IME_CONVERSION_MODE, param2: IME_SENTENCE_MODE) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetConversionStatus(param0 : HIMC, param1 : IME_CONVERSION_MODE, param2 : IME_SENTENCE_MODE) -> windows_core::BOOL);
    unsafe { ImmSetConversionStatus(param0, param1, param2) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmSetHotKey(param0: u32, param1: u32, param2: u32, param3: super::KeyboardAndMouse::HKL) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetHotKey(param0 : u32, param1 : u32, param2 : u32, param3 : super::KeyboardAndMouse:: HKL) -> windows_core::BOOL);
    unsafe { ImmSetHotKey(param0, param1, param2, param3) }
}
#[inline]
pub unsafe fn ImmSetOpenStatus(param0: HIMC, param1: bool) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetOpenStatus(param0 : HIMC, param1 : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ImmSetOpenStatus(param0, param1.into()) }
}
#[inline]
pub unsafe fn ImmSetStatusWindowPos(param0: HIMC, lpptpos: *const super::super::super::Foundation::POINT) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSetStatusWindowPos(param0 : HIMC, lpptpos : *const super::super::super::Foundation:: POINT) -> windows_core::BOOL);
    unsafe { ImmSetStatusWindowPos(param0, lpptpos) }
}
#[inline]
pub unsafe fn ImmShowSoftKeyboard(param0: super::super::super::Foundation::HWND, param1: i32) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmShowSoftKeyboard(param0 : super::super::super::Foundation:: HWND, param1 : i32) -> windows_core::BOOL);
    unsafe { ImmShowSoftKeyboard(param0, param1) }
}
#[inline]
pub unsafe fn ImmSimulateHotKey(param0: super::super::super::Foundation::HWND, param1: IME_HOTKEY_IDENTIFIER) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmSimulateHotKey(param0 : super::super::super::Foundation:: HWND, param1 : IME_HOTKEY_IDENTIFIER) -> windows_core::BOOL);
    unsafe { ImmSimulateHotKey(param0, param1) }
}
#[inline]
pub unsafe fn ImmUnlockIMC(param0: HIMC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmUnlockIMC(param0 : HIMC) -> windows_core::BOOL);
    unsafe { ImmUnlockIMC(param0) }
}
#[inline]
pub unsafe fn ImmUnlockIMCC(param0: HIMCC) -> windows_core::BOOL {
    windows_core::link!("imm32.dll" "system" fn ImmUnlockIMCC(param0 : HIMCC) -> windows_core::BOOL);
    unsafe { ImmUnlockIMCC(param0) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmUnregisterWordA<P1, P3>(param0: super::KeyboardAndMouse::HKL, lpszreading: P1, param2: u32, lpszunregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmUnregisterWordA(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCSTR, param2 : u32, lpszunregister : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ImmUnregisterWordA(param0, lpszreading.param().abi(), param2, lpszunregister.param().abi()) }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
#[inline]
pub unsafe fn ImmUnregisterWordW<P1, P3>(param0: super::KeyboardAndMouse::HKL, lpszreading: P1, param2: u32, lpszunregister: P3) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("imm32.dll" "system" fn ImmUnregisterWordW(param0 : super::KeyboardAndMouse:: HKL, lpszreading : windows_core::PCWSTR, param2 : u32, lpszunregister : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ImmUnregisterWordW(param0, lpszreading.param().abi(), param2, lpszunregister.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APPLETIDLIST {
    pub count: i32,
    pub pIIDList: *mut windows_core::GUID,
}
impl Default for APPLETIDLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APPLYCANDEXPARAM {
    pub dwSize: u32,
    pub lpwstrDisplay: windows_core::PWSTR,
    pub lpwstrReading: windows_core::PWSTR,
    pub dwReserved: u32,
}
pub const ATTR_CONVERTED: u32 = 2u32;
pub const ATTR_FIXEDCONVERTED: u32 = 5u32;
pub const ATTR_INPUT: u32 = 0u32;
pub const ATTR_INPUT_ERROR: u32 = 4u32;
pub const ATTR_TARGET_CONVERTED: u32 = 1u32;
pub const ATTR_TARGET_NOTCONVERTED: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CANDIDATEFORM {
    pub dwIndex: u32,
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CANDIDATEINFO {
    pub dwSize: u32,
    pub dwCount: u32,
    pub dwOffset: [u32; 32],
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
}
impl Default for CANDIDATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CANDIDATELIST {
    pub dwSize: u32,
    pub dwStyle: u32,
    pub dwCount: u32,
    pub dwSelection: u32,
    pub dwPageStart: u32,
    pub dwPageSize: u32,
    pub dwOffset: [u32; 1],
}
impl Default for CANDIDATELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CATID_MSIME_IImePadApplet: windows_core::GUID = windows_core::GUID::from_u128(0x7566cad1_4ec9_4478_9fe9_8ed766619edf);
pub const CATID_MSIME_IImePadApplet1000: windows_core::GUID = windows_core::GUID::from_u128(0xe081e1d6_2389_43cb_b66f_609f823d9f9c);
pub const CATID_MSIME_IImePadApplet1200: windows_core::GUID = windows_core::GUID::from_u128(0xa47fb5fc_7d15_4223_a789_b781bf9ae667);
pub const CATID_MSIME_IImePadApplet900: windows_core::GUID = windows_core::GUID::from_u128(0xfaae51bf_5e5b_4a1d_8de1_17c1d9e1728d);
pub const CATID_MSIME_IImePadApplet_VER7: windows_core::GUID = windows_core::GUID::from_u128(0x4a0f8e31_c3ee_11d1_afef_00805f0c8b6d);
pub const CATID_MSIME_IImePadApplet_VER80: windows_core::GUID = windows_core::GUID::from_u128(0x56f7a792_fef1_11d3_8463_00c04f7a06e5);
pub const CATID_MSIME_IImePadApplet_VER81: windows_core::GUID = windows_core::GUID::from_u128(0x656520b0_bb88_11d4_84c0_00c04f7a06e5);
pub const CActiveIMM: windows_core::GUID = windows_core::GUID::from_u128(0x4955dd33_b159_11d0_8fcf_00aa006bcc59);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPOSITIONFORM {
    pub dwStyle: u32,
    pub ptCurrentPos: super::super::super::Foundation::POINT,
    pub rcArea: super::super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_CONVERSION_LIST_FLAG(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_GUIDE_LINE_TYPE(pub u32);
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GUIDELINE {
    pub dwSize: u32,
    pub dwLevel: u32,
    pub dwIndex: u32,
    pub dwStrLen: u32,
    pub dwStrOffset: u32,
    pub dwPrivateSize: u32,
    pub dwPrivateOffset: u32,
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
            windows_core::link!("imm32.dll" "system" fn ImmDestroyContext(param0 : *mut core::ffi::c_void) -> i32);
            unsafe {
                ImmDestroyContext(self.0);
            }
        }
    }
}
impl Default for HIMC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub const IACE_CHILDREN: u32 = 1u32;
pub const IACE_DEFAULT: u32 = 16u32;
pub const IACE_IGNORENOCONTEXT: u32 = 32u32;
windows_core::imp::define_interface!(IActiveIME, IActiveIME_Vtbl, 0x6fe20962_d077_11d0_8fe7_00aa006bcc59);
windows_core::imp::interface_hierarchy!(IActiveIME, windows_core::IUnknown);
impl IActiveIME {
    pub unsafe fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: windows_core::PWSTR, pdwprivate: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Inquire)(windows_core::Interface::as_raw(self), dwsysteminfoflags, pimeinfo as _, core::mem::transmute(szwndclass), pdwprivate as _).ok() }
    }
    pub unsafe fn ConversionList<P1>(&self, himc: HIMC, szsource: P1, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConversionList)(windows_core::Interface::as_raw(self), himc, szsource.param().abi(), uflag, ubuflen, pdest as _, pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn Configure(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Configure)(windows_core::Interface::as_raw(self), hkl, hwnd, dwmode, pregisterword).ok() }
    }
    pub unsafe fn Destroy(&self, ureserved: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Destroy)(windows_core::Interface::as_raw(self), ureserved).ok() }
    }
    pub unsafe fn Escape(&self, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), himc, uescape, pdata as _, plresult as _).ok() }
    }
    pub unsafe fn SetActiveContext(&self, himc: HIMC, fflag: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetActiveContext)(windows_core::Interface::as_raw(self), himc, fflag.into()).ok() }
    }
    pub unsafe fn ProcessKey(&self, himc: HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessKey)(windows_core::Interface::as_raw(self), himc, uvirkey, lparam, pbkeystate).ok() }
    }
    pub unsafe fn Notify(&self, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), himc, dwaction, dwindex, dwvalue).ok() }
    }
    pub unsafe fn Select(&self, himc: HIMC, fselect: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), himc, fselect.into()).ok() }
    }
    pub unsafe fn SetCompositionString(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionString)(windows_core::Interface::as_raw(self), himc, dwindex, pcomp, dwcomplen, pread, dwreadlen).ok() }
    }
    pub unsafe fn ToAsciiEx(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ToAsciiEx)(windows_core::Interface::as_raw(self), uvirkey, uscancode, pbkeystate, fustate, himc, pdwtransbuf as _, pusize as _).ok() }
    }
    pub unsafe fn RegisterWord<P0, P2>(&self, szreading: P0, dwstyle: u32, szstring: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterWord)(windows_core::Interface::as_raw(self), szreading.param().abi(), dwstyle, szstring.param().abi()).ok() }
    }
    pub unsafe fn UnregisterWord<P0, P2>(&self, szreading: P0, dwstyle: u32, szstring: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterWord)(windows_core::Interface::as_raw(self), szreading.param().abi(), dwstyle, szstring.param().abi()).ok() }
    }
    pub unsafe fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegisterWordStyle)(windows_core::Interface::as_raw(self), nitem, pstylebuf as _, pubufsize as _).ok() }
    }
    pub unsafe fn EnumRegisterWord<P0, P2>(&self, szreading: P0, dwstyle: u32, szregister: P2, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterWord)(windows_core::Interface::as_raw(self), szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCodePageA(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodePageA)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLangId(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SetActiveContext: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, windows_core::BOOL) -> windows_core::HRESULT,
    pub ProcessKey: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, *const u8) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, u32, u32) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCompositionString: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, u32, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ToAsciiEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, HIMC, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRegisterWordStyle: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STYLEBUFW, *mut u32) -> windows_core::HRESULT,
    pub EnumRegisterWord: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCodePageA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLangId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait IActiveIME_Impl: windows_core::IUnknownImpl {
    fn Inquire(&self, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: windows_core::PWSTR, pdwprivate: *mut u32) -> windows_core::Result<()>;
    fn ConversionList(&self, himc: HIMC, szsource: &windows_core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn Configure(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> windows_core::Result<()>;
    fn Destroy(&self, ureserved: u32) -> windows_core::Result<()>;
    fn Escape(&self, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn SetActiveContext(&self, himc: HIMC, fflag: windows_core::BOOL) -> windows_core::Result<()>;
    fn ProcessKey(&self, himc: HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> windows_core::Result<()>;
    fn Notify(&self, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()>;
    fn Select(&self, himc: HIMC, fselect: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCompositionString(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>;
    fn ToAsciiEx(&self, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> windows_core::Result<()>;
    fn RegisterWord(&self, szreading: &windows_core::PCWSTR, dwstyle: u32, szstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterWord(&self, szreading: &windows_core::PCWSTR, dwstyle: u32, szstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRegisterWordStyle(&self, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> windows_core::Result<()>;
    fn EnumRegisterWord(&self, szreading: &windows_core::PCWSTR, dwstyle: u32, szregister: &windows_core::PCWSTR, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>;
    fn GetCodePageA(&self) -> windows_core::Result<u32>;
    fn GetLangId(&self) -> windows_core::Result<u16>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl IActiveIME_Vtbl {
    pub const fn new<Identity: IActiveIME_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Inquire<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: windows_core::PWSTR, pdwprivate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::Inquire(this, core::mem::transmute_copy(&dwsysteminfoflags), core::mem::transmute_copy(&pimeinfo), core::mem::transmute_copy(&szwndclass), core::mem::transmute_copy(&pdwprivate)).into()
            }
        }
        unsafe extern "system" fn ConversionList<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, szsource: windows_core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::ConversionList(this, core::mem::transmute_copy(&himc), core::mem::transmute(&szsource), core::mem::transmute_copy(&uflag), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&pdest), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn Configure<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::Configure(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pregisterword)).into()
            }
        }
        unsafe extern "system" fn Destroy<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ureserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::Destroy(this, core::mem::transmute_copy(&ureserved)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::Escape(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&uescape), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn SetActiveContext<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, fflag: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::SetActiveContext(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&fflag)).into()
            }
        }
        unsafe extern "system" fn ProcessKey<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::ProcessKey(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&uvirkey), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&pbkeystate)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::Notify(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn Select<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, fselect: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::Select(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&fselect)).into()
            }
        }
        unsafe extern "system" fn SetCompositionString<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::SetCompositionString(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwcomplen), core::mem::transmute_copy(&pread), core::mem::transmute_copy(&dwreadlen)).into()
            }
        }
        unsafe extern "system" fn ToAsciiEx<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::ToAsciiEx(this, core::mem::transmute_copy(&uvirkey), core::mem::transmute_copy(&uscancode), core::mem::transmute_copy(&pbkeystate), core::mem::transmute_copy(&fustate), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pdwtransbuf), core::mem::transmute_copy(&pusize)).into()
            }
        }
        unsafe extern "system" fn RegisterWord<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szreading: windows_core::PCWSTR, dwstyle: u32, szstring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::RegisterWord(this, core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szstring)).into()
            }
        }
        unsafe extern "system" fn UnregisterWord<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szreading: windows_core::PCWSTR, dwstyle: u32, szstring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::UnregisterWord(this, core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szstring)).into()
            }
        }
        unsafe extern "system" fn GetRegisterWordStyle<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME_Impl::GetRegisterWordStyle(this, core::mem::transmute_copy(&nitem), core::mem::transmute_copy(&pstylebuf), core::mem::transmute_copy(&pubufsize)).into()
            }
        }
        unsafe extern "system" fn EnumRegisterWord<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szreading: windows_core::PCWSTR, dwstyle: u32, szregister: windows_core::PCWSTR, pdata: *const core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIME_Impl::EnumRegisterWord(this, core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister), core::mem::transmute_copy(&pdata)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCodePageA<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ucodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIME_Impl::GetCodePageA(this) {
                    Ok(ok__) => {
                        ucodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLangId<Identity: IActiveIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plid: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIME_Impl::GetLangId(this) {
                    Ok(ok__) => {
                        plid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Inquire: Inquire::<Identity, OFFSET>,
            ConversionList: ConversionList::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            SetActiveContext: SetActiveContext::<Identity, OFFSET>,
            ProcessKey: ProcessKey::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            Select: Select::<Identity, OFFSET>,
            SetCompositionString: SetCompositionString::<Identity, OFFSET>,
            ToAsciiEx: ToAsciiEx::<Identity, OFFSET>,
            RegisterWord: RegisterWord::<Identity, OFFSET>,
            UnregisterWord: UnregisterWord::<Identity, OFFSET>,
            GetRegisterWordStyle: GetRegisterWordStyle::<Identity, OFFSET>,
            EnumRegisterWord: EnumRegisterWord::<Identity, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, OFFSET>,
            GetLangId: GetLangId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveIME as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for IActiveIME {}
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
        unsafe { (windows_core::Interface::vtable(self).Sleep)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unsleep(&self, fdead: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unsleep)(windows_core::Interface::as_raw(self), fdead.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIME2_Vtbl {
    pub base__: IActiveIME_Vtbl,
    pub Sleep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unsleep: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
pub trait IActiveIME2_Impl: IActiveIME_Impl {
    fn Sleep(&self) -> windows_core::Result<()>;
    fn Unsleep(&self, fdead: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl IActiveIME2_Vtbl {
    pub const fn new<Identity: IActiveIME2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Sleep<Identity: IActiveIME2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME2_Impl::Sleep(this).into()
            }
        }
        unsafe extern "system" fn Unsleep<Identity: IActiveIME2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdead: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIME2_Impl::Unsleep(this, core::mem::transmute_copy(&fdead)).into()
            }
        }
        Self { base__: IActiveIME_Vtbl::new::<Identity, OFFSET>(), Sleep: Sleep::<Identity, OFFSET>, Unsleep: Unsleep::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveIME2 as windows_core::Interface>::IID || iid == &<IActiveIME as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
impl windows_core::RuntimeName for IActiveIME2 {}
windows_core::imp::define_interface!(IActiveIMMApp, IActiveIMMApp_Vtbl, 0x08c0e040_62d1_11d1_9326_0060b067b86e);
windows_core::imp::interface_hierarchy!(IActiveIMMApp, windows_core::IUnknown);
impl IActiveIMMApp {
    pub unsafe fn AssociateContext(&self, hwnd: super::super::super::Foundation::HWND, hime: HIMC) -> windows_core::Result<HIMC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssociateContext)(windows_core::Interface::as_raw(self), hwnd, hime, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEA(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureIMEA)(windows_core::Interface::as_raw(self), hkl, hwnd, dwmode, pdata).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEW(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureIMEW)(windows_core::Interface::as_raw(self), hkl, hwnd, dwmode, pdata).ok() }
    }
    pub unsafe fn CreateContext(&self) -> windows_core::Result<HIMC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestroyContext(&self, hime: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyContext)(windows_core::Interface::as_raw(self), hime).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordA<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordA>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterWordA)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordW<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterWordW)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeA(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EscapeA)(windows_core::Interface::as_raw(self), hkl, himc, uescape, pdata as _, plresult as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeW(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EscapeW)(windows_core::Interface::as_raw(self), hkl, himc, uescape, pdata as _, plresult as _).ok() }
    }
    pub unsafe fn GetCandidateListA(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListA)(windows_core::Interface::as_raw(self), himc, dwindex, ubuflen, pcandlist as _, pucopied as _).ok() }
    }
    pub unsafe fn GetCandidateListW(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListW)(windows_core::Interface::as_raw(self), himc, dwindex, ubuflen, pcandlist as _, pucopied as _).ok() }
    }
    pub unsafe fn GetCandidateListCountA(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListCountA)(windows_core::Interface::as_raw(self), himc, pdwlistsize as _, pdwbuflen as _).ok() }
    }
    pub unsafe fn GetCandidateListCountW(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListCountW)(windows_core::Interface::as_raw(self), himc, pdwlistsize as _, pdwbuflen as _).ok() }
    }
    pub unsafe fn GetCandidateWindow(&self, himc: HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateWindow)(windows_core::Interface::as_raw(self), himc, dwindex, pcandidate as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontA(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionFontA)(windows_core::Interface::as_raw(self), himc, plf as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontW(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionFontW)(windows_core::Interface::as_raw(self), himc, plf as _).ok() }
    }
    pub unsafe fn GetCompositionStringA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionStringA)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, plcopied as _, pbuf as _).ok() }
    }
    pub unsafe fn GetCompositionStringW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionStringW)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, plcopied as _, pbuf as _).ok() }
    }
    pub unsafe fn GetCompositionWindow(&self, himc: HIMC, pcompform: *mut COMPOSITIONFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionWindow)(windows_core::Interface::as_raw(self), himc, pcompform as _).ok() }
    }
    pub unsafe fn GetContext(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<HIMC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), hwnd, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListA<P2>(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConversionListA)(windows_core::Interface::as_raw(self), hkl, himc, psrc.param().abi(), ubuflen, uflag, pdst as _, pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListW<P2>(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConversionListW)(windows_core::Interface::as_raw(self), hkl, himc, psrc.param().abi(), ubuflen, uflag, pdst as _, pucopied as _).ok() }
    }
    pub unsafe fn GetConversionStatus(&self, himc: HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConversionStatus)(windows_core::Interface::as_raw(self), himc, pfdwconversion as _, pfdwsentence as _).ok() }
    }
    pub unsafe fn GetDefaultIMEWnd(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultIMEWnd)(windows_core::Interface::as_raw(self), hwnd, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptionA)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szdescription), pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptionW)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szdescription), pucopied as _).ok() }
    }
    pub unsafe fn GetGuideLineA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGuideLineA)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult as _).ok() }
    }
    pub unsafe fn GetGuideLineW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGuideLineW)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIMEFileNameA)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szfilename), pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIMEFileNameW)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szfilename), pucopied as _).ok() }
    }
    pub unsafe fn GetOpenStatus(&self, himc: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOpenStatus)(windows_core::Interface::as_raw(self), himc).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetProperty(&self, hkl: super::KeyboardAndMouse::HKL, fdwindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), hkl, fdwindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleA(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegisterWordStyleA)(windows_core::Interface::as_raw(self), hkl, nitem, pstylebuf as _, pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleW(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegisterWordStyleW)(windows_core::Interface::as_raw(self), hkl, nitem, pstylebuf as _, pucopied as _).ok() }
    }
    pub unsafe fn GetStatusWindowPos(&self, himc: HIMC) -> windows_core::Result<super::super::super::Foundation::POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatusWindowPos)(windows_core::Interface::as_raw(self), himc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVirtualKey(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVirtualKey)(windows_core::Interface::as_raw(self), hwnd, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEA<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallIMEA)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEW<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallIMEW)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn IsIME(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsIME)(windows_core::Interface::as_raw(self), hkl).ok() }
    }
    pub unsafe fn IsUIMessageA(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsUIMessageA)(windows_core::Interface::as_raw(self), hwndime, msg, wparam, lparam).ok() }
    }
    pub unsafe fn IsUIMessageW(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsUIMessageW)(windows_core::Interface::as_raw(self), hwndime, msg, wparam, lparam).ok() }
    }
    pub unsafe fn NotifyIME(&self, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyIME)(windows_core::Interface::as_raw(self), himc, dwaction, dwindex, dwvalue).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordA<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterWordA)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordW<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterWordW)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi()).ok() }
    }
    pub unsafe fn ReleaseContext(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseContext)(windows_core::Interface::as_raw(self), hwnd, himc).ok() }
    }
    pub unsafe fn SetCandidateWindow(&self, himc: HIMC, pcandidate: *const CANDIDATEFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCandidateWindow)(windows_core::Interface::as_raw(self), himc, pcandidate).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontA(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionFontA)(windows_core::Interface::as_raw(self), himc, plf).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontW(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionFontW)(windows_core::Interface::as_raw(self), himc, plf).ok() }
    }
    pub unsafe fn SetCompositionStringA(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionStringA)(windows_core::Interface::as_raw(self), himc, dwindex, pcomp, dwcomplen, pread, dwreadlen).ok() }
    }
    pub unsafe fn SetCompositionStringW(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionStringW)(windows_core::Interface::as_raw(self), himc, dwindex, pcomp, dwcomplen, pread, dwreadlen).ok() }
    }
    pub unsafe fn SetCompositionWindow(&self, himc: HIMC, pcompform: *const COMPOSITIONFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionWindow)(windows_core::Interface::as_raw(self), himc, pcompform).ok() }
    }
    pub unsafe fn SetConversionStatus(&self, himc: HIMC, fdwconversion: u32, fdwsentence: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConversionStatus)(windows_core::Interface::as_raw(self), himc, fdwconversion, fdwsentence).ok() }
    }
    pub unsafe fn SetOpenStatus(&self, himc: HIMC, fopen: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpenStatus)(windows_core::Interface::as_raw(self), himc, fopen.into()).ok() }
    }
    pub unsafe fn SetStatusWindowPos(&self, himc: HIMC, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStatusWindowPos)(windows_core::Interface::as_raw(self), himc, pptpos).ok() }
    }
    pub unsafe fn SimulateHotKey(&self, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SimulateHotKey)(windows_core::Interface::as_raw(self), hwnd, dwhotkeyid).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordA<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szunregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterWordA)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szunregister.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordW<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szunregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterWordW)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szunregister.param().abi()).ok() }
    }
    pub unsafe fn Activate(&self, frestorelayout: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), frestorelayout.into()).ok() }
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnDefWindowProc(&self, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnDefWindowProc)(windows_core::Interface::as_raw(self), hwnd, msg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FilterClientWindows)(windows_core::Interface::as_raw(self), aaclasslist, usize).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetCodePageA(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodePageA)(windows_core::Interface::as_raw(self), hkl, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetLangId(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), hkl, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AssociateContextEx(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssociateContextEx)(windows_core::Interface::as_raw(self), hwnd, himc, dwflags).ok() }
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableIME)(windows_core::Interface::as_raw(self), idthread).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsA(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImeMenuItemsA)(windows_core::Interface::as_raw(self), himc, dwflags, dwtype, pimeparentmenu, pimemenu as _, dwsize, pdwresult as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsW(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImeMenuItemsW)(windows_core::Interface::as_raw(self), himc, dwflags, dwtype, pimeparentmenu, pimemenu as _, dwsize, pdwresult as _).ok() }
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> windows_core::Result<IEnumInputContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumInputContext)(windows_core::Interface::as_raw(self), idthread, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SetOpenStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, windows_core::BOOL) -> windows_core::HRESULT,
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
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_KeyboardAndMouse"))]
pub trait IActiveIMMApp_Impl: windows_core::IUnknownImpl {
    fn AssociateContext(&self, hwnd: super::super::super::Foundation::HWND, hime: HIMC) -> windows_core::Result<HIMC>;
    fn ConfigureIMEA(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::Result<()>;
    fn ConfigureIMEW(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::Result<()>;
    fn CreateContext(&self) -> windows_core::Result<HIMC>;
    fn DestroyContext(&self, hime: HIMC) -> windows_core::Result<()>;
    fn EnumRegisterWordA(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCSTR, dwstyle: u32, szregister: &windows_core::PCSTR, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordA>;
    fn EnumRegisterWordW(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCWSTR, dwstyle: u32, szregister: &windows_core::PCWSTR, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>;
    fn EscapeA(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn EscapeW(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn GetCandidateListA(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateListW(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateListCountA(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateListCountW(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateWindow(&self, himc: HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::Result<()>;
    fn GetCompositionFontA(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>;
    fn GetCompositionFontW(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>;
    fn GetCompositionStringA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCompositionStringW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCompositionWindow(&self, himc: HIMC, pcompform: *mut COMPOSITIONFORM) -> windows_core::Result<()>;
    fn GetContext(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<HIMC>;
    fn GetConversionListA(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: &windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetConversionListW(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: &windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetConversionStatus(&self, himc: HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::Result<()>;
    fn GetDefaultIMEWnd(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND>;
    fn GetDescriptionA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetDescriptionW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetGuideLineA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn GetGuideLineW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn GetIMEFileNameA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetIMEFileNameW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetOpenStatus(&self, himc: HIMC) -> windows_core::Result<()>;
    fn GetProperty(&self, hkl: super::KeyboardAndMouse::HKL, fdwindex: u32) -> windows_core::Result<u32>;
    fn GetRegisterWordStyleA(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetRegisterWordStyleW(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetStatusWindowPos(&self, himc: HIMC) -> windows_core::Result<super::super::super::Foundation::POINT>;
    fn GetVirtualKey(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<u32>;
    fn InstallIMEA(&self, szimefilename: &windows_core::PCSTR, szlayouttext: &windows_core::PCSTR) -> windows_core::Result<super::KeyboardAndMouse::HKL>;
    fn InstallIMEW(&self, szimefilename: &windows_core::PCWSTR, szlayouttext: &windows_core::PCWSTR) -> windows_core::Result<super::KeyboardAndMouse::HKL>;
    fn IsIME(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<()>;
    fn IsUIMessageA(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn IsUIMessageW(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn NotifyIME(&self, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()>;
    fn RegisterWordA(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCSTR, dwstyle: u32, szregister: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn RegisterWordW(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCWSTR, dwstyle: u32, szregister: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReleaseContext(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC) -> windows_core::Result<()>;
    fn SetCandidateWindow(&self, himc: HIMC, pcandidate: *const CANDIDATEFORM) -> windows_core::Result<()>;
    fn SetCompositionFontA(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>;
    fn SetCompositionFontW(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>;
    fn SetCompositionStringA(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>;
    fn SetCompositionStringW(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>;
    fn SetCompositionWindow(&self, himc: HIMC, pcompform: *const COMPOSITIONFORM) -> windows_core::Result<()>;
    fn SetConversionStatus(&self, himc: HIMC, fdwconversion: u32, fdwsentence: u32) -> windows_core::Result<()>;
    fn SetOpenStatus(&self, himc: HIMC, fopen: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetStatusWindowPos(&self, himc: HIMC, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn SimulateHotKey(&self, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> windows_core::Result<()>;
    fn UnregisterWordA(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCSTR, dwstyle: u32, szunregister: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn UnregisterWordW(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCWSTR, dwstyle: u32, szunregister: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Activate(&self, frestorelayout: windows_core::BOOL) -> windows_core::Result<()>;
    fn Deactivate(&self) -> windows_core::Result<()>;
    fn OnDefWindowProc(&self, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn FilterClientWindows(&self, aaclasslist: *const u16, usize: u32) -> windows_core::Result<()>;
    fn GetCodePageA(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u32>;
    fn GetLangId(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u16>;
    fn AssociateContextEx(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC, dwflags: u32) -> windows_core::Result<()>;
    fn DisableIME(&self, idthread: u32) -> windows_core::Result<()>;
    fn GetImeMenuItemsA(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn GetImeMenuItemsW(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn EnumInputContext(&self, idthread: u32) -> windows_core::Result<IEnumInputContext>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl IActiveIMMApp_Vtbl {
    pub const fn new<Identity: IActiveIMMApp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AssociateContext<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: HIMC, phprev: *mut HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::AssociateContext(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&hime)) {
                    Ok(ok__) => {
                        phprev.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConfigureIMEA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::ConfigureIMEA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn ConfigureIMEW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::ConfigureIMEW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn CreateContext<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phimc: *mut HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::CreateContext(this) {
                    Ok(ok__) => {
                        phimc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroyContext<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hime: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::DestroyContext(this, core::mem::transmute_copy(&hime)).into()
            }
        }
        unsafe extern "system" fn EnumRegisterWordA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCSTR, dwstyle: u32, szregister: windows_core::PCSTR, pdata: *const core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::EnumRegisterWordA(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister), core::mem::transmute_copy(&pdata)) {
                    Ok(ok__) => {
                        penum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRegisterWordW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCWSTR, dwstyle: u32, szregister: windows_core::PCWSTR, pdata: *const core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::EnumRegisterWordW(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister), core::mem::transmute_copy(&pdata)) {
                    Ok(ok__) => {
                        penum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EscapeA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::EscapeA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&uescape), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn EscapeW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::EscapeW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&uescape), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCandidateListA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&pcandlist), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCandidateListW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&pcandlist), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListCountA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCandidateListCountA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pdwlistsize), core::mem::transmute_copy(&pdwbuflen)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListCountW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCandidateListCountW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pdwlistsize), core::mem::transmute_copy(&pdwbuflen)).into()
            }
        }
        unsafe extern "system" fn GetCandidateWindow<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCandidateWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcandidate)).into()
            }
        }
        unsafe extern "system" fn GetCompositionFontA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCompositionFontA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionFontW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCompositionFontW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionStringA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCompositionStringA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&plcopied), core::mem::transmute_copy(&pbuf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionStringW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCompositionStringW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&plcopied), core::mem::transmute_copy(&pbuf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionWindow<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pcompform: *mut COMPOSITIONFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetCompositionWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pcompform)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetContext(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        phimc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConversionListA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetConversionListA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute(&psrc), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&uflag), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetConversionListW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetConversionListW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute(&psrc), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&uflag), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetConversionStatus<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetConversionStatus(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pfdwconversion), core::mem::transmute_copy(&pfdwsentence)).into()
            }
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetDefaultIMEWnd(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        phdefwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescriptionA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetDescriptionA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetDescriptionW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetDescriptionW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetGuideLineA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetGuideLineA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&pbuf), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn GetGuideLineW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetGuideLineW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&pbuf), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn GetIMEFileNameA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetIMEFileNameA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szfilename), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetIMEFileNameW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetIMEFileNameW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szfilename), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetOpenStatus<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetOpenStatus(this, core::mem::transmute_copy(&himc)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, fdwindex: u32, pdwproperty: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetProperty(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&fdwindex)) {
                    Ok(ok__) => {
                        pdwproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetRegisterWordStyleA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&nitem), core::mem::transmute_copy(&pstylebuf), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetRegisterWordStyleW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&nitem), core::mem::transmute_copy(&pstylebuf), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetStatusWindowPos<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetStatusWindowPos(this, core::mem::transmute_copy(&himc)) {
                    Ok(ok__) => {
                        pptpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVirtualKey<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetVirtualKey(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        puvirtualkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallIMEA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szimefilename: windows_core::PCSTR, szlayouttext: windows_core::PCSTR, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::InstallIMEA(this, core::mem::transmute(&szimefilename), core::mem::transmute(&szlayouttext)) {
                    Ok(ok__) => {
                        phkl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallIMEW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szimefilename: windows_core::PCWSTR, szlayouttext: windows_core::PCWSTR, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::InstallIMEW(this, core::mem::transmute(&szimefilename), core::mem::transmute(&szlayouttext)) {
                    Ok(ok__) => {
                        phkl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsIME<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::IsIME(this, core::mem::transmute_copy(&hkl)).into()
            }
        }
        unsafe extern "system" fn IsUIMessageA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::IsUIMessageA(this, core::mem::transmute_copy(&hwndime), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn IsUIMessageW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::IsUIMessageW(this, core::mem::transmute_copy(&hwndime), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn NotifyIME<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::NotifyIME(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn RegisterWordA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCSTR, dwstyle: u32, szregister: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::RegisterWordA(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister)).into()
            }
        }
        unsafe extern "system" fn RegisterWordW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCWSTR, dwstyle: u32, szregister: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::RegisterWordW(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister)).into()
            }
        }
        unsafe extern "system" fn ReleaseContext<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::ReleaseContext(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&himc)).into()
            }
        }
        unsafe extern "system" fn SetCandidateWindow<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pcandidate: *const CANDIDATEFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetCandidateWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pcandidate)).into()
            }
        }
        unsafe extern "system" fn SetCompositionFontA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetCompositionFontA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn SetCompositionFontW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetCompositionFontW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn SetCompositionStringA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetCompositionStringA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwcomplen), core::mem::transmute_copy(&pread), core::mem::transmute_copy(&dwreadlen)).into()
            }
        }
        unsafe extern "system" fn SetCompositionStringW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetCompositionStringW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwcomplen), core::mem::transmute_copy(&pread), core::mem::transmute_copy(&dwreadlen)).into()
            }
        }
        unsafe extern "system" fn SetCompositionWindow<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pcompform: *const COMPOSITIONFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetCompositionWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pcompform)).into()
            }
        }
        unsafe extern "system" fn SetConversionStatus<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, fdwconversion: u32, fdwsentence: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetConversionStatus(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&fdwconversion), core::mem::transmute_copy(&fdwsentence)).into()
            }
        }
        unsafe extern "system" fn SetOpenStatus<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, fopen: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetOpenStatus(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&fopen)).into()
            }
        }
        unsafe extern "system" fn SetStatusWindowPos<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SetStatusWindowPos(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pptpos)).into()
            }
        }
        unsafe extern "system" fn SimulateHotKey<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::SimulateHotKey(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwhotkeyid)).into()
            }
        }
        unsafe extern "system" fn UnregisterWordA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCSTR, dwstyle: u32, szunregister: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::UnregisterWordA(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szunregister)).into()
            }
        }
        unsafe extern "system" fn UnregisterWordW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCWSTR, dwstyle: u32, szunregister: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::UnregisterWordW(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szunregister)).into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frestorelayout: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::Activate(this, core::mem::transmute_copy(&frestorelayout)).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::Deactivate(this).into()
            }
        }
        unsafe extern "system" fn OnDefWindowProc<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::OnDefWindowProc(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilterClientWindows<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::FilterClientWindows(this, core::mem::transmute_copy(&aaclasslist), core::mem::transmute_copy(&usize)).into()
            }
        }
        unsafe extern "system" fn GetCodePageA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ucodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetCodePageA(this, core::mem::transmute_copy(&hkl)) {
                    Ok(ok__) => {
                        ucodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLangId<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, plid: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::GetLangId(this, core::mem::transmute_copy(&hkl)) {
                    Ok(ok__) => {
                        plid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssociateContextEx<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: HIMC, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::AssociateContextEx(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DisableIME<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idthread: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::DisableIME(this, core::mem::transmute_copy(&idthread)).into()
            }
        }
        unsafe extern "system" fn GetImeMenuItemsA<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetImeMenuItemsA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pimeparentmenu), core::mem::transmute_copy(&pimemenu), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn GetImeMenuItemsW<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMApp_Impl::GetImeMenuItemsW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pimeparentmenu), core::mem::transmute_copy(&pimemenu), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn EnumInputContext<Identity: IActiveIMMApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idthread: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMApp_Impl::EnumInputContext(this, core::mem::transmute_copy(&idthread)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssociateContext: AssociateContext::<Identity, OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Identity, OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Identity, OFFSET>,
            CreateContext: CreateContext::<Identity, OFFSET>,
            DestroyContext: DestroyContext::<Identity, OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Identity, OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Identity, OFFSET>,
            EscapeA: EscapeA::<Identity, OFFSET>,
            EscapeW: EscapeW::<Identity, OFFSET>,
            GetCandidateListA: GetCandidateListA::<Identity, OFFSET>,
            GetCandidateListW: GetCandidateListW::<Identity, OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Identity, OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Identity, OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Identity, OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Identity, OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Identity, OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Identity, OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Identity, OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetConversionListA: GetConversionListA::<Identity, OFFSET>,
            GetConversionListW: GetConversionListW::<Identity, OFFSET>,
            GetConversionStatus: GetConversionStatus::<Identity, OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Identity, OFFSET>,
            GetDescriptionA: GetDescriptionA::<Identity, OFFSET>,
            GetDescriptionW: GetDescriptionW::<Identity, OFFSET>,
            GetGuideLineA: GetGuideLineA::<Identity, OFFSET>,
            GetGuideLineW: GetGuideLineW::<Identity, OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Identity, OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Identity, OFFSET>,
            GetOpenStatus: GetOpenStatus::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Identity, OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Identity, OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Identity, OFFSET>,
            GetVirtualKey: GetVirtualKey::<Identity, OFFSET>,
            InstallIMEA: InstallIMEA::<Identity, OFFSET>,
            InstallIMEW: InstallIMEW::<Identity, OFFSET>,
            IsIME: IsIME::<Identity, OFFSET>,
            IsUIMessageA: IsUIMessageA::<Identity, OFFSET>,
            IsUIMessageW: IsUIMessageW::<Identity, OFFSET>,
            NotifyIME: NotifyIME::<Identity, OFFSET>,
            RegisterWordA: RegisterWordA::<Identity, OFFSET>,
            RegisterWordW: RegisterWordW::<Identity, OFFSET>,
            ReleaseContext: ReleaseContext::<Identity, OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Identity, OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Identity, OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Identity, OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Identity, OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Identity, OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Identity, OFFSET>,
            SetConversionStatus: SetConversionStatus::<Identity, OFFSET>,
            SetOpenStatus: SetOpenStatus::<Identity, OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Identity, OFFSET>,
            SimulateHotKey: SimulateHotKey::<Identity, OFFSET>,
            UnregisterWordA: UnregisterWordA::<Identity, OFFSET>,
            UnregisterWordW: UnregisterWordW::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            OnDefWindowProc: OnDefWindowProc::<Identity, OFFSET>,
            FilterClientWindows: FilterClientWindows::<Identity, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, OFFSET>,
            GetLangId: GetLangId::<Identity, OFFSET>,
            AssociateContextEx: AssociateContextEx::<Identity, OFFSET>,
            DisableIME: DisableIME::<Identity, OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Identity, OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Identity, OFFSET>,
            EnumInputContext: EnumInputContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveIMMApp as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl windows_core::RuntimeName for IActiveIMMApp {}
windows_core::imp::define_interface!(IActiveIMMIME, IActiveIMMIME_Vtbl, 0x08c03411_f96b_11d0_a475_00aa006bcc59);
windows_core::imp::interface_hierarchy!(IActiveIMMIME, windows_core::IUnknown);
impl IActiveIMMIME {
    pub unsafe fn AssociateContext(&self, hwnd: super::super::super::Foundation::HWND, hime: HIMC) -> windows_core::Result<HIMC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AssociateContext)(windows_core::Interface::as_raw(self), hwnd, hime, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEA(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureIMEA)(windows_core::Interface::as_raw(self), hkl, hwnd, dwmode, pdata).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn ConfigureIMEW(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureIMEW)(windows_core::Interface::as_raw(self), hkl, hwnd, dwmode, pdata).ok() }
    }
    pub unsafe fn CreateContext(&self) -> windows_core::Result<HIMC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestroyContext(&self, hime: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyContext)(windows_core::Interface::as_raw(self), hime).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordA<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordA>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterWordA)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EnumRegisterWordW<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRegisterWordW)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi(), pdata, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeA(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EscapeA)(windows_core::Interface::as_raw(self), hkl, himc, uescape, pdata as _, plresult as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn EscapeW(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EscapeW)(windows_core::Interface::as_raw(self), hkl, himc, uescape, pdata as _, plresult as _).ok() }
    }
    pub unsafe fn GetCandidateListA(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListA)(windows_core::Interface::as_raw(self), himc, dwindex, ubuflen, pcandlist as _, pucopied as _).ok() }
    }
    pub unsafe fn GetCandidateListW(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListW)(windows_core::Interface::as_raw(self), himc, dwindex, ubuflen, pcandlist as _, pucopied as _).ok() }
    }
    pub unsafe fn GetCandidateListCountA(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListCountA)(windows_core::Interface::as_raw(self), himc, pdwlistsize as _, pdwbuflen as _).ok() }
    }
    pub unsafe fn GetCandidateListCountW(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateListCountW)(windows_core::Interface::as_raw(self), himc, pdwlistsize as _, pdwbuflen as _).ok() }
    }
    pub unsafe fn GetCandidateWindow(&self, himc: HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCandidateWindow)(windows_core::Interface::as_raw(self), himc, dwindex, pcandidate as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontA(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionFontA)(windows_core::Interface::as_raw(self), himc, plf as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetCompositionFontW(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionFontW)(windows_core::Interface::as_raw(self), himc, plf as _).ok() }
    }
    pub unsafe fn GetCompositionStringA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionStringA)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, plcopied as _, pbuf as _).ok() }
    }
    pub unsafe fn GetCompositionStringW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionStringW)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, plcopied as _, pbuf as _).ok() }
    }
    pub unsafe fn GetCompositionWindow(&self, himc: HIMC, pcompform: *mut COMPOSITIONFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCompositionWindow)(windows_core::Interface::as_raw(self), himc, pcompform as _).ok() }
    }
    pub unsafe fn GetContext(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<HIMC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), hwnd, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListA<P2>(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConversionListA)(windows_core::Interface::as_raw(self), hkl, himc, psrc.param().abi(), ubuflen, uflag, pdst as _, pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetConversionListW<P2>(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: P2, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetConversionListW)(windows_core::Interface::as_raw(self), hkl, himc, psrc.param().abi(), ubuflen, uflag, pdst as _, pucopied as _).ok() }
    }
    pub unsafe fn GetConversionStatus(&self, himc: HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConversionStatus)(windows_core::Interface::as_raw(self), himc, pfdwconversion as _, pfdwsentence as _).ok() }
    }
    pub unsafe fn GetDefaultIMEWnd(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultIMEWnd)(windows_core::Interface::as_raw(self), hwnd, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptionA)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szdescription), pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetDescriptionW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDescriptionW)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szdescription), pucopied as _).ok() }
    }
    pub unsafe fn GetGuideLineA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGuideLineA)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult as _).ok() }
    }
    pub unsafe fn GetGuideLineW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetGuideLineW)(windows_core::Interface::as_raw(self), himc, dwindex, dwbuflen, core::mem::transmute(pbuf), pdwresult as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIMEFileNameA)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szfilename), pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetIMEFileNameW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIMEFileNameW)(windows_core::Interface::as_raw(self), hkl, ubuflen, core::mem::transmute(szfilename), pucopied as _).ok() }
    }
    pub unsafe fn GetOpenStatus(&self, himc: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetOpenStatus)(windows_core::Interface::as_raw(self), himc).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetProperty(&self, hkl: super::KeyboardAndMouse::HKL, fdwindex: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), hkl, fdwindex, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleA(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegisterWordStyleA)(windows_core::Interface::as_raw(self), hkl, nitem, pstylebuf as _, pucopied as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetRegisterWordStyleW(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegisterWordStyleW)(windows_core::Interface::as_raw(self), hkl, nitem, pstylebuf as _, pucopied as _).ok() }
    }
    pub unsafe fn GetStatusWindowPos(&self, himc: HIMC) -> windows_core::Result<super::super::super::Foundation::POINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatusWindowPos)(windows_core::Interface::as_raw(self), himc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVirtualKey(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVirtualKey)(windows_core::Interface::as_raw(self), hwnd, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEA<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallIMEA)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn InstallIMEW<P0, P1>(&self, szimefilename: P0, szlayouttext: P1) -> windows_core::Result<super::KeyboardAndMouse::HKL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallIMEW)(windows_core::Interface::as_raw(self), szimefilename.param().abi(), szlayouttext.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn IsIME(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsIME)(windows_core::Interface::as_raw(self), hkl).ok() }
    }
    pub unsafe fn IsUIMessageA(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsUIMessageA)(windows_core::Interface::as_raw(self), hwndime, msg, wparam, lparam).ok() }
    }
    pub unsafe fn IsUIMessageW(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsUIMessageW)(windows_core::Interface::as_raw(self), hwndime, msg, wparam, lparam).ok() }
    }
    pub unsafe fn NotifyIME(&self, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyIME)(windows_core::Interface::as_raw(self), himc, dwaction, dwindex, dwvalue).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordA<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterWordA)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn RegisterWordW<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterWordW)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szregister.param().abi()).ok() }
    }
    pub unsafe fn ReleaseContext(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleaseContext)(windows_core::Interface::as_raw(self), hwnd, himc).ok() }
    }
    pub unsafe fn SetCandidateWindow(&self, himc: HIMC, pcandidate: *const CANDIDATEFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCandidateWindow)(windows_core::Interface::as_raw(self), himc, pcandidate).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontA(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionFontA)(windows_core::Interface::as_raw(self), himc, plf).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetCompositionFontW(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionFontW)(windows_core::Interface::as_raw(self), himc, plf).ok() }
    }
    pub unsafe fn SetCompositionStringA(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionStringA)(windows_core::Interface::as_raw(self), himc, dwindex, pcomp, dwcomplen, pread, dwreadlen).ok() }
    }
    pub unsafe fn SetCompositionStringW(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionStringW)(windows_core::Interface::as_raw(self), himc, dwindex, pcomp, dwcomplen, pread, dwreadlen).ok() }
    }
    pub unsafe fn SetCompositionWindow(&self, himc: HIMC, pcompform: *const COMPOSITIONFORM) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCompositionWindow)(windows_core::Interface::as_raw(self), himc, pcompform).ok() }
    }
    pub unsafe fn SetConversionStatus(&self, himc: HIMC, fdwconversion: u32, fdwsentence: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConversionStatus)(windows_core::Interface::as_raw(self), himc, fdwconversion, fdwsentence).ok() }
    }
    pub unsafe fn SetOpenStatus(&self, himc: HIMC, fopen: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpenStatus)(windows_core::Interface::as_raw(self), himc, fopen.into()).ok() }
    }
    pub unsafe fn SetStatusWindowPos(&self, himc: HIMC, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStatusWindowPos)(windows_core::Interface::as_raw(self), himc, pptpos).ok() }
    }
    pub unsafe fn SimulateHotKey(&self, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SimulateHotKey)(windows_core::Interface::as_raw(self), hwnd, dwhotkeyid).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordA<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szunregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
        P3: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterWordA)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szunregister.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn UnregisterWordW<P1, P3>(&self, hkl: super::KeyboardAndMouse::HKL, szreading: P1, dwstyle: u32, szunregister: P3) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterWordW)(windows_core::Interface::as_raw(self), hkl, szreading.param().abi(), dwstyle, szunregister.param().abi()).ok() }
    }
    pub unsafe fn GenerateMessage(&self, himc: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GenerateMessage)(windows_core::Interface::as_raw(self), himc).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn LockIMC(&self, himc: HIMC) -> windows_core::Result<*mut INPUTCONTEXT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LockIMC)(windows_core::Interface::as_raw(self), himc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnlockIMC(&self, himc: HIMC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockIMC)(windows_core::Interface::as_raw(self), himc).ok() }
    }
    pub unsafe fn GetIMCLockCount(&self, himc: HIMC) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIMCLockCount)(windows_core::Interface::as_raw(self), himc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateIMCC(&self, dwsize: u32) -> windows_core::Result<HIMCC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateIMCC)(windows_core::Interface::as_raw(self), dwsize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestroyIMCC(&self, himcc: HIMCC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyIMCC)(windows_core::Interface::as_raw(self), himcc).ok() }
    }
    pub unsafe fn LockIMCC(&self, himcc: HIMCC, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockIMCC)(windows_core::Interface::as_raw(self), himcc, ppv as _).ok() }
    }
    pub unsafe fn UnlockIMCC(&self, himcc: HIMCC) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockIMCC)(windows_core::Interface::as_raw(self), himcc).ok() }
    }
    pub unsafe fn ReSizeIMCC(&self, himcc: HIMCC, dwsize: u32) -> windows_core::Result<HIMCC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReSizeIMCC)(windows_core::Interface::as_raw(self), himcc, dwsize, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIMCCSize(&self, himcc: HIMCC) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIMCCSize)(windows_core::Interface::as_raw(self), himcc, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIMCCLockCount(&self, himcc: HIMCC) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIMCCLockCount)(windows_core::Interface::as_raw(self), himcc, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHotKey)(windows_core::Interface::as_raw(self), dwhotkeyid, pumodifiers as _, puvkey as _, phkl as _).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn SetHotKey(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHotKey)(windows_core::Interface::as_raw(self), dwhotkeyid, umodifiers, uvkey, hkl).ok() }
    }
    pub unsafe fn CreateSoftKeyboard(&self, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::Result<super::super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSoftKeyboard)(windows_core::Interface::as_raw(self), utype, howner, x, y, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DestroySoftKeyboard(&self, hsoftkbdwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroySoftKeyboard)(windows_core::Interface::as_raw(self), hsoftkbdwnd).ok() }
    }
    pub unsafe fn ShowSoftKeyboard(&self, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ShowSoftKeyboard)(windows_core::Interface::as_raw(self), hsoftkbdwnd, ncmdshow).ok() }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetCodePageA(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCodePageA)(windows_core::Interface::as_raw(self), hkl, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_UI_Input_KeyboardAndMouse")]
    pub unsafe fn GetLangId(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLangId)(windows_core::Interface::as_raw(self), hkl, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).KeybdEvent)(windows_core::Interface::as_raw(self), lgidime, bvk, bscan, dwflags, dwextrainfo).ok() }
    }
    pub unsafe fn LockModal(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockModal)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnlockModal(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockModal)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AssociateContextEx(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AssociateContextEx)(windows_core::Interface::as_raw(self), hwnd, himc, dwflags).ok() }
    }
    pub unsafe fn DisableIME(&self, idthread: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableIME)(windows_core::Interface::as_raw(self), idthread).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsA(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImeMenuItemsA)(windows_core::Interface::as_raw(self), himc, dwflags, dwtype, pimeparentmenu, pimemenu as _, dwsize, pdwresult as _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetImeMenuItemsW(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImeMenuItemsW)(windows_core::Interface::as_raw(self), himc, dwflags, dwtype, pimeparentmenu, pimemenu as _, dwsize, pdwresult as _).ok() }
    }
    pub unsafe fn EnumInputContext(&self, idthread: u32) -> windows_core::Result<IEnumInputContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumInputContext)(windows_core::Interface::as_raw(self), idthread, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RequestMessageA(&self, himc: HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestMessageA)(windows_core::Interface::as_raw(self), himc, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestMessageW(&self, himc: HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestMessageW)(windows_core::Interface::as_raw(self), himc, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SendIMCA(&self, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendIMCA)(windows_core::Interface::as_raw(self), hwnd, umsg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SendIMCW(&self, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendIMCW)(windows_core::Interface::as_raw(self), hwnd, umsg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSleeping(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsSleeping)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
    pub SetOpenStatus: unsafe extern "system" fn(*mut core::ffi::c_void, HIMC, windows_core::BOOL) -> windows_core::HRESULT,
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
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_KeyboardAndMouse"))]
pub trait IActiveIMMIME_Impl: windows_core::IUnknownImpl {
    fn AssociateContext(&self, hwnd: super::super::super::Foundation::HWND, hime: HIMC) -> windows_core::Result<HIMC>;
    fn ConfigureIMEA(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::Result<()>;
    fn ConfigureIMEW(&self, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::Result<()>;
    fn CreateContext(&self) -> windows_core::Result<HIMC>;
    fn DestroyContext(&self, hime: HIMC) -> windows_core::Result<()>;
    fn EnumRegisterWordA(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCSTR, dwstyle: u32, szregister: &windows_core::PCSTR, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordA>;
    fn EnumRegisterWordW(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCWSTR, dwstyle: u32, szregister: &windows_core::PCWSTR, pdata: *const core::ffi::c_void) -> windows_core::Result<IEnumRegisterWordW>;
    fn EscapeA(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn EscapeW(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn GetCandidateListA(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateListW(&self, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateListCountA(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateListCountW(&self, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::Result<()>;
    fn GetCandidateWindow(&self, himc: HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::Result<()>;
    fn GetCompositionFontA(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>;
    fn GetCompositionFontW(&self, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>;
    fn GetCompositionStringA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCompositionStringW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCompositionWindow(&self, himc: HIMC, pcompform: *mut COMPOSITIONFORM) -> windows_core::Result<()>;
    fn GetContext(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<HIMC>;
    fn GetConversionListA(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: &windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetConversionListW(&self, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: &windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetConversionStatus(&self, himc: HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::Result<()>;
    fn GetDefaultIMEWnd(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND>;
    fn GetDescriptionA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetDescriptionW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetGuideLineA(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn GetGuideLineW(&self, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn GetIMEFileNameA(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetIMEFileNameW(&self, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetOpenStatus(&self, himc: HIMC) -> windows_core::Result<()>;
    fn GetProperty(&self, hkl: super::KeyboardAndMouse::HKL, fdwindex: u32) -> windows_core::Result<u32>;
    fn GetRegisterWordStyleA(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetRegisterWordStyleW(&self, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::Result<()>;
    fn GetStatusWindowPos(&self, himc: HIMC) -> windows_core::Result<super::super::super::Foundation::POINT>;
    fn GetVirtualKey(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<u32>;
    fn InstallIMEA(&self, szimefilename: &windows_core::PCSTR, szlayouttext: &windows_core::PCSTR) -> windows_core::Result<super::KeyboardAndMouse::HKL>;
    fn InstallIMEW(&self, szimefilename: &windows_core::PCWSTR, szlayouttext: &windows_core::PCWSTR) -> windows_core::Result<super::KeyboardAndMouse::HKL>;
    fn IsIME(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<()>;
    fn IsUIMessageA(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn IsUIMessageW(&self, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn NotifyIME(&self, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::Result<()>;
    fn RegisterWordA(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCSTR, dwstyle: u32, szregister: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn RegisterWordW(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCWSTR, dwstyle: u32, szregister: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReleaseContext(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC) -> windows_core::Result<()>;
    fn SetCandidateWindow(&self, himc: HIMC, pcandidate: *const CANDIDATEFORM) -> windows_core::Result<()>;
    fn SetCompositionFontA(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::Result<()>;
    fn SetCompositionFontW(&self, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::Result<()>;
    fn SetCompositionStringA(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>;
    fn SetCompositionStringW(&self, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::Result<()>;
    fn SetCompositionWindow(&self, himc: HIMC, pcompform: *const COMPOSITIONFORM) -> windows_core::Result<()>;
    fn SetConversionStatus(&self, himc: HIMC, fdwconversion: u32, fdwsentence: u32) -> windows_core::Result<()>;
    fn SetOpenStatus(&self, himc: HIMC, fopen: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetStatusWindowPos(&self, himc: HIMC, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::Result<()>;
    fn SimulateHotKey(&self, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> windows_core::Result<()>;
    fn UnregisterWordA(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCSTR, dwstyle: u32, szunregister: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn UnregisterWordW(&self, hkl: super::KeyboardAndMouse::HKL, szreading: &windows_core::PCWSTR, dwstyle: u32, szunregister: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GenerateMessage(&self, himc: HIMC) -> windows_core::Result<()>;
    fn LockIMC(&self, himc: HIMC) -> windows_core::Result<*mut INPUTCONTEXT>;
    fn UnlockIMC(&self, himc: HIMC) -> windows_core::Result<()>;
    fn GetIMCLockCount(&self, himc: HIMC) -> windows_core::Result<u32>;
    fn CreateIMCC(&self, dwsize: u32) -> windows_core::Result<HIMCC>;
    fn DestroyIMCC(&self, himcc: HIMCC) -> windows_core::Result<()>;
    fn LockIMCC(&self, himcc: HIMCC, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UnlockIMCC(&self, himcc: HIMCC) -> windows_core::Result<()>;
    fn ReSizeIMCC(&self, himcc: HIMCC, dwsize: u32) -> windows_core::Result<HIMCC>;
    fn GetIMCCSize(&self, himcc: HIMCC) -> windows_core::Result<u32>;
    fn GetIMCCLockCount(&self, himcc: HIMCC) -> windows_core::Result<u32>;
    fn GetHotKey(&self, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::Result<()>;
    fn SetHotKey(&self, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<()>;
    fn CreateSoftKeyboard(&self, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::Result<super::super::super::Foundation::HWND>;
    fn DestroySoftKeyboard(&self, hsoftkbdwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn ShowSoftKeyboard(&self, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> windows_core::Result<()>;
    fn GetCodePageA(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u32>;
    fn GetLangId(&self, hkl: super::KeyboardAndMouse::HKL) -> windows_core::Result<u16>;
    fn KeybdEvent(&self, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> windows_core::Result<()>;
    fn LockModal(&self) -> windows_core::Result<()>;
    fn UnlockModal(&self) -> windows_core::Result<()>;
    fn AssociateContextEx(&self, hwnd: super::super::super::Foundation::HWND, himc: HIMC, dwflags: u32) -> windows_core::Result<()>;
    fn DisableIME(&self, idthread: u32) -> windows_core::Result<()>;
    fn GetImeMenuItemsA(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn GetImeMenuItemsW(&self, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::Result<()>;
    fn EnumInputContext(&self, idthread: u32) -> windows_core::Result<IEnumInputContext>;
    fn RequestMessageA(&self, himc: HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn RequestMessageW(&self, himc: HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn SendIMCA(&self, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn SendIMCW(&self, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn IsSleeping(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl IActiveIMMIME_Vtbl {
    pub const fn new<Identity: IActiveIMMIME_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AssociateContext<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: HIMC, phprev: *mut HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::AssociateContext(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&hime)) {
                    Ok(ok__) => {
                        phprev.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConfigureIMEA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::ConfigureIMEA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn ConfigureIMEW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::ConfigureIMEW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwmode), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn CreateContext<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phimc: *mut HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::CreateContext(this) {
                    Ok(ok__) => {
                        phimc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroyContext<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hime: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::DestroyContext(this, core::mem::transmute_copy(&hime)).into()
            }
        }
        unsafe extern "system" fn EnumRegisterWordA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCSTR, dwstyle: u32, szregister: windows_core::PCSTR, pdata: *const core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::EnumRegisterWordA(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister), core::mem::transmute_copy(&pdata)) {
                    Ok(ok__) => {
                        penum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRegisterWordW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCWSTR, dwstyle: u32, szregister: windows_core::PCWSTR, pdata: *const core::ffi::c_void, penum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::EnumRegisterWordW(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister), core::mem::transmute_copy(&pdata)) {
                    Ok(ok__) => {
                        penum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EscapeA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::EscapeA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&uescape), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn EscapeW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, uescape: u32, pdata: *mut core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::EscapeW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&uescape), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&plresult)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCandidateListA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&pcandlist), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCandidateListW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&pcandlist), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListCountA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCandidateListCountA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pdwlistsize), core::mem::transmute_copy(&pdwbuflen)).into()
            }
        }
        unsafe extern "system" fn GetCandidateListCountW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCandidateListCountW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pdwlistsize), core::mem::transmute_copy(&pdwbuflen)).into()
            }
        }
        unsafe extern "system" fn GetCandidateWindow<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCandidateWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcandidate)).into()
            }
        }
        unsafe extern "system" fn GetCompositionFontA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCompositionFontA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionFontW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCompositionFontW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionStringA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCompositionStringA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&plcopied), core::mem::transmute_copy(&pbuf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionStringW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCompositionStringW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&plcopied), core::mem::transmute_copy(&pbuf)).into()
            }
        }
        unsafe extern "system" fn GetCompositionWindow<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pcompform: *mut COMPOSITIONFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetCompositionWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pcompform)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetContext(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        phimc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConversionListA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetConversionListA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute(&psrc), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&uflag), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetConversionListW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, himc: HIMC, psrc: windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetConversionListW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&himc), core::mem::transmute(&psrc), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&uflag), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetConversionStatus<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetConversionStatus(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pfdwconversion), core::mem::transmute_copy(&pfdwsentence)).into()
            }
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetDefaultIMEWnd(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        phdefwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescriptionA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetDescriptionA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetDescriptionW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szdescription: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetDescriptionW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szdescription), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetGuideLineA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PSTR, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetGuideLineA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&pbuf), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn GetGuideLineW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, dwbuflen: u32, pbuf: windows_core::PWSTR, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetGuideLineW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwbuflen), core::mem::transmute_copy(&pbuf), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn GetIMEFileNameA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetIMEFileNameA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szfilename), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetIMEFileNameW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ubuflen: u32, szfilename: windows_core::PWSTR, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetIMEFileNameW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&ubuflen), core::mem::transmute_copy(&szfilename), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetOpenStatus<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetOpenStatus(this, core::mem::transmute_copy(&himc)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, fdwindex: u32, pdwproperty: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetProperty(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&fdwindex)) {
                    Ok(ok__) => {
                        pdwproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetRegisterWordStyleA(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&nitem), core::mem::transmute_copy(&pstylebuf), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetRegisterWordStyleW(this, core::mem::transmute_copy(&hkl), core::mem::transmute_copy(&nitem), core::mem::transmute_copy(&pstylebuf), core::mem::transmute_copy(&pucopied)).into()
            }
        }
        unsafe extern "system" fn GetStatusWindowPos<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetStatusWindowPos(this, core::mem::transmute_copy(&himc)) {
                    Ok(ok__) => {
                        pptpos.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVirtualKey<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetVirtualKey(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        puvirtualkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallIMEA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szimefilename: windows_core::PCSTR, szlayouttext: windows_core::PCSTR, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::InstallIMEA(this, core::mem::transmute(&szimefilename), core::mem::transmute(&szlayouttext)) {
                    Ok(ok__) => {
                        phkl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstallIMEW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szimefilename: windows_core::PCWSTR, szlayouttext: windows_core::PCWSTR, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::InstallIMEW(this, core::mem::transmute(&szimefilename), core::mem::transmute(&szlayouttext)) {
                    Ok(ok__) => {
                        phkl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsIME<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::IsIME(this, core::mem::transmute_copy(&hkl)).into()
            }
        }
        unsafe extern "system" fn IsUIMessageA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::IsUIMessageA(this, core::mem::transmute_copy(&hwndime), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn IsUIMessageW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::IsUIMessageW(this, core::mem::transmute_copy(&hwndime), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        unsafe extern "system" fn NotifyIME<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::NotifyIME(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwaction), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwvalue)).into()
            }
        }
        unsafe extern "system" fn RegisterWordA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCSTR, dwstyle: u32, szregister: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::RegisterWordA(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister)).into()
            }
        }
        unsafe extern "system" fn RegisterWordW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCWSTR, dwstyle: u32, szregister: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::RegisterWordW(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szregister)).into()
            }
        }
        unsafe extern "system" fn ReleaseContext<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::ReleaseContext(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&himc)).into()
            }
        }
        unsafe extern "system" fn SetCandidateWindow<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pcandidate: *const CANDIDATEFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetCandidateWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pcandidate)).into()
            }
        }
        unsafe extern "system" fn SetCompositionFontA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetCompositionFontA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn SetCompositionFontW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetCompositionFontW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&plf)).into()
            }
        }
        unsafe extern "system" fn SetCompositionStringA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetCompositionStringA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwcomplen), core::mem::transmute_copy(&pread), core::mem::transmute_copy(&dwreadlen)).into()
            }
        }
        unsafe extern "system" fn SetCompositionStringW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwindex: u32, pcomp: *const core::ffi::c_void, dwcomplen: u32, pread: *const core::ffi::c_void, dwreadlen: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetCompositionStringW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pcomp), core::mem::transmute_copy(&dwcomplen), core::mem::transmute_copy(&pread), core::mem::transmute_copy(&dwreadlen)).into()
            }
        }
        unsafe extern "system" fn SetCompositionWindow<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pcompform: *const COMPOSITIONFORM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetCompositionWindow(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pcompform)).into()
            }
        }
        unsafe extern "system" fn SetConversionStatus<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, fdwconversion: u32, fdwsentence: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetConversionStatus(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&fdwconversion), core::mem::transmute_copy(&fdwsentence)).into()
            }
        }
        unsafe extern "system" fn SetOpenStatus<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, fopen: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetOpenStatus(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&fopen)).into()
            }
        }
        unsafe extern "system" fn SetStatusWindowPos<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pptpos: *const super::super::super::Foundation::POINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetStatusWindowPos(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&pptpos)).into()
            }
        }
        unsafe extern "system" fn SimulateHotKey<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SimulateHotKey(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&dwhotkeyid)).into()
            }
        }
        unsafe extern "system" fn UnregisterWordA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCSTR, dwstyle: u32, szunregister: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::UnregisterWordA(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szunregister)).into()
            }
        }
        unsafe extern "system" fn UnregisterWordW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, szreading: windows_core::PCWSTR, dwstyle: u32, szunregister: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::UnregisterWordW(this, core::mem::transmute_copy(&hkl), core::mem::transmute(&szreading), core::mem::transmute_copy(&dwstyle), core::mem::transmute(&szunregister)).into()
            }
        }
        unsafe extern "system" fn GenerateMessage<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GenerateMessage(this, core::mem::transmute_copy(&himc)).into()
            }
        }
        unsafe extern "system" fn LockIMC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, ppimc: *mut *mut INPUTCONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::LockIMC(this, core::mem::transmute_copy(&himc)) {
                    Ok(ok__) => {
                        ppimc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnlockIMC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::UnlockIMC(this, core::mem::transmute_copy(&himc)).into()
            }
        }
        unsafe extern "system" fn GetIMCLockCount<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, pdwlockcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetIMCLockCount(this, core::mem::transmute_copy(&himc)) {
                    Ok(ok__) => {
                        pdwlockcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateIMCC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsize: u32, phimcc: *mut HIMCC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::CreateIMCC(this, core::mem::transmute_copy(&dwsize)) {
                    Ok(ok__) => {
                        phimcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroyIMCC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himcc: HIMCC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::DestroyIMCC(this, core::mem::transmute_copy(&himcc)).into()
            }
        }
        unsafe extern "system" fn LockIMCC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himcc: HIMCC, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::LockIMCC(this, core::mem::transmute_copy(&himcc), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn UnlockIMCC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himcc: HIMCC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::UnlockIMCC(this, core::mem::transmute_copy(&himcc)).into()
            }
        }
        unsafe extern "system" fn ReSizeIMCC<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himcc: HIMCC, dwsize: u32, phimcc: *mut HIMCC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::ReSizeIMCC(this, core::mem::transmute_copy(&himcc), core::mem::transmute_copy(&dwsize)) {
                    Ok(ok__) => {
                        phimcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIMCCSize<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himcc: HIMCC, pdwsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetIMCCSize(this, core::mem::transmute_copy(&himcc)) {
                    Ok(ok__) => {
                        pdwsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIMCCLockCount<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himcc: HIMCC, pdwlockcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetIMCCLockCount(this, core::mem::transmute_copy(&himcc)) {
                    Ok(ok__) => {
                        pdwlockcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHotKey<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetHotKey(this, core::mem::transmute_copy(&dwhotkeyid), core::mem::transmute_copy(&pumodifiers), core::mem::transmute_copy(&puvkey), core::mem::transmute_copy(&phkl)).into()
            }
        }
        unsafe extern "system" fn SetHotKey<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::KeyboardAndMouse::HKL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::SetHotKey(this, core::mem::transmute_copy(&dwhotkeyid), core::mem::transmute_copy(&umodifiers), core::mem::transmute_copy(&uvkey), core::mem::transmute_copy(&hkl)).into()
            }
        }
        unsafe extern "system" fn CreateSoftKeyboard<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::CreateSoftKeyboard(this, core::mem::transmute_copy(&utype), core::mem::transmute_copy(&howner), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                    Ok(ok__) => {
                        phsoftkbdwnd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroySoftKeyboard<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::DestroySoftKeyboard(this, core::mem::transmute_copy(&hsoftkbdwnd)).into()
            }
        }
        unsafe extern "system" fn ShowSoftKeyboard<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::ShowSoftKeyboard(this, core::mem::transmute_copy(&hsoftkbdwnd), core::mem::transmute_copy(&ncmdshow)).into()
            }
        }
        unsafe extern "system" fn GetCodePageA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, ucodepage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetCodePageA(this, core::mem::transmute_copy(&hkl)) {
                    Ok(ok__) => {
                        ucodepage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLangId<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkl: super::KeyboardAndMouse::HKL, plid: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::GetLangId(this, core::mem::transmute_copy(&hkl)) {
                    Ok(ok__) => {
                        plid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeybdEvent<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::KeybdEvent(this, core::mem::transmute_copy(&lgidime), core::mem::transmute_copy(&bvk), core::mem::transmute_copy(&bscan), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwextrainfo)).into()
            }
        }
        unsafe extern "system" fn LockModal<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::LockModal(this).into()
            }
        }
        unsafe extern "system" fn UnlockModal<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::UnlockModal(this).into()
            }
        }
        unsafe extern "system" fn AssociateContextEx<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: HIMC, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::AssociateContextEx(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DisableIME<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idthread: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::DisableIME(this, core::mem::transmute_copy(&idthread)).into()
            }
        }
        unsafe extern "system" fn GetImeMenuItemsA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetImeMenuItemsA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pimeparentmenu), core::mem::transmute_copy(&pimemenu), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn GetImeMenuItemsW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::GetImeMenuItemsW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwtype), core::mem::transmute_copy(&pimeparentmenu), core::mem::transmute_copy(&pimemenu), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwresult)).into()
            }
        }
        unsafe extern "system" fn EnumInputContext<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idthread: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::EnumInputContext(this, core::mem::transmute_copy(&idthread)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestMessageA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::RequestMessageA(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestMessageW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, himc: HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::RequestMessageW(this, core::mem::transmute_copy(&himc), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendIMCA<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::SendIMCA(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendIMCW<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMIME_Impl::SendIMCW(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        plresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSleeping<Identity: IActiveIMMIME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMIME_Impl::IsSleeping(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AssociateContext: AssociateContext::<Identity, OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Identity, OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Identity, OFFSET>,
            CreateContext: CreateContext::<Identity, OFFSET>,
            DestroyContext: DestroyContext::<Identity, OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Identity, OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Identity, OFFSET>,
            EscapeA: EscapeA::<Identity, OFFSET>,
            EscapeW: EscapeW::<Identity, OFFSET>,
            GetCandidateListA: GetCandidateListA::<Identity, OFFSET>,
            GetCandidateListW: GetCandidateListW::<Identity, OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Identity, OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Identity, OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Identity, OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Identity, OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Identity, OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Identity, OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Identity, OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetConversionListA: GetConversionListA::<Identity, OFFSET>,
            GetConversionListW: GetConversionListW::<Identity, OFFSET>,
            GetConversionStatus: GetConversionStatus::<Identity, OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Identity, OFFSET>,
            GetDescriptionA: GetDescriptionA::<Identity, OFFSET>,
            GetDescriptionW: GetDescriptionW::<Identity, OFFSET>,
            GetGuideLineA: GetGuideLineA::<Identity, OFFSET>,
            GetGuideLineW: GetGuideLineW::<Identity, OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Identity, OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Identity, OFFSET>,
            GetOpenStatus: GetOpenStatus::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Identity, OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Identity, OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Identity, OFFSET>,
            GetVirtualKey: GetVirtualKey::<Identity, OFFSET>,
            InstallIMEA: InstallIMEA::<Identity, OFFSET>,
            InstallIMEW: InstallIMEW::<Identity, OFFSET>,
            IsIME: IsIME::<Identity, OFFSET>,
            IsUIMessageA: IsUIMessageA::<Identity, OFFSET>,
            IsUIMessageW: IsUIMessageW::<Identity, OFFSET>,
            NotifyIME: NotifyIME::<Identity, OFFSET>,
            RegisterWordA: RegisterWordA::<Identity, OFFSET>,
            RegisterWordW: RegisterWordW::<Identity, OFFSET>,
            ReleaseContext: ReleaseContext::<Identity, OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Identity, OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Identity, OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Identity, OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Identity, OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Identity, OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Identity, OFFSET>,
            SetConversionStatus: SetConversionStatus::<Identity, OFFSET>,
            SetOpenStatus: SetOpenStatus::<Identity, OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Identity, OFFSET>,
            SimulateHotKey: SimulateHotKey::<Identity, OFFSET>,
            UnregisterWordA: UnregisterWordA::<Identity, OFFSET>,
            UnregisterWordW: UnregisterWordW::<Identity, OFFSET>,
            GenerateMessage: GenerateMessage::<Identity, OFFSET>,
            LockIMC: LockIMC::<Identity, OFFSET>,
            UnlockIMC: UnlockIMC::<Identity, OFFSET>,
            GetIMCLockCount: GetIMCLockCount::<Identity, OFFSET>,
            CreateIMCC: CreateIMCC::<Identity, OFFSET>,
            DestroyIMCC: DestroyIMCC::<Identity, OFFSET>,
            LockIMCC: LockIMCC::<Identity, OFFSET>,
            UnlockIMCC: UnlockIMCC::<Identity, OFFSET>,
            ReSizeIMCC: ReSizeIMCC::<Identity, OFFSET>,
            GetIMCCSize: GetIMCCSize::<Identity, OFFSET>,
            GetIMCCLockCount: GetIMCCLockCount::<Identity, OFFSET>,
            GetHotKey: GetHotKey::<Identity, OFFSET>,
            SetHotKey: SetHotKey::<Identity, OFFSET>,
            CreateSoftKeyboard: CreateSoftKeyboard::<Identity, OFFSET>,
            DestroySoftKeyboard: DestroySoftKeyboard::<Identity, OFFSET>,
            ShowSoftKeyboard: ShowSoftKeyboard::<Identity, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, OFFSET>,
            GetLangId: GetLangId::<Identity, OFFSET>,
            KeybdEvent: KeybdEvent::<Identity, OFFSET>,
            LockModal: LockModal::<Identity, OFFSET>,
            UnlockModal: UnlockModal::<Identity, OFFSET>,
            AssociateContextEx: AssociateContextEx::<Identity, OFFSET>,
            DisableIME: DisableIME::<Identity, OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Identity, OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Identity, OFFSET>,
            EnumInputContext: EnumInputContext::<Identity, OFFSET>,
            RequestMessageA: RequestMessageA::<Identity, OFFSET>,
            RequestMessageW: RequestMessageW::<Identity, OFFSET>,
            SendIMCA: SendIMCA::<Identity, OFFSET>,
            SendIMCW: SendIMCW::<Identity, OFFSET>,
            IsSleeping: IsSleeping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveIMMIME as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Input_KeyboardAndMouse"))]
impl windows_core::RuntimeName for IActiveIMMIME {}
windows_core::imp::define_interface!(IActiveIMMMessagePumpOwner, IActiveIMMMessagePumpOwner_Vtbl, 0xb5cf2cfa_8aeb_11d1_9364_0060b067b86e);
windows_core::imp::interface_hierarchy!(IActiveIMMMessagePumpOwner, windows_core::IUnknown);
impl IActiveIMMMessagePumpOwner {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn End(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTranslateMessage)(windows_core::Interface::as_raw(self), pmsg).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Resume(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IActiveIMMMessagePumpOwner_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
    fn OnTranslateMessage(&self, pmsg: *const super::super::WindowsAndMessaging::MSG) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<u32>;
    fn Resume(&self, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IActiveIMMMessagePumpOwner_Vtbl {
    pub const fn new<Identity: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMMessagePumpOwner_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn End<Identity: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMMessagePumpOwner_Impl::End(this).into()
            }
        }
        unsafe extern "system" fn OnTranslateMessage<Identity: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMMessagePumpOwner_Impl::OnTranslateMessage(this, core::mem::transmute_copy(&pmsg)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveIMMMessagePumpOwner_Impl::Pause(this) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Resume<Identity: IActiveIMMMessagePumpOwner_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMMessagePumpOwner_Impl::Resume(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            End: End::<Identity, OFFSET>,
            OnTranslateMessage: OnTranslateMessage::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveIMMMessagePumpOwner as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IActiveIMMMessagePumpOwner {}
windows_core::imp::define_interface!(IActiveIMMRegistrar, IActiveIMMRegistrar_Vtbl, 0xb3458082_bd00_11d1_939b_0060b067b86e);
windows_core::imp::interface_hierarchy!(IActiveIMMRegistrar, windows_core::IUnknown);
impl IActiveIMMRegistrar {
    pub unsafe fn RegisterIME<P2, P3>(&self, rclsid: *const windows_core::GUID, lgid: u16, psziconfile: P2, pszdesc: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterIME)(windows_core::Interface::as_raw(self), rclsid, lgid, psziconfile.param().abi(), pszdesc.param().abi()).ok() }
    }
    pub unsafe fn UnregisterIME(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterIME)(windows_core::Interface::as_raw(self), rclsid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveIMMRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterIME: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u16, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterIME: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IActiveIMMRegistrar_Impl: windows_core::IUnknownImpl {
    fn RegisterIME(&self, rclsid: *const windows_core::GUID, lgid: u16, psziconfile: &windows_core::PCWSTR, pszdesc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterIME(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IActiveIMMRegistrar_Vtbl {
    pub const fn new<Identity: IActiveIMMRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterIME<Identity: IActiveIMMRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, lgid: u16, psziconfile: windows_core::PCWSTR, pszdesc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMRegistrar_Impl::RegisterIME(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&lgid), core::mem::transmute(&psziconfile), core::mem::transmute(&pszdesc)).into()
            }
        }
        unsafe extern "system" fn UnregisterIME<Identity: IActiveIMMRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveIMMRegistrar_Impl::UnregisterIME(this, core::mem::transmute_copy(&rclsid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterIME: RegisterIME::<Identity, OFFSET>,
            UnregisterIME: UnregisterIME::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveIMMRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveIMMRegistrar {}
windows_core::imp::define_interface!(IEnumInputContext, IEnumInputContext_Vtbl, 0x09b5eab0_f997_11d1_93d4_0060b067b86e);
windows_core::imp::interface_hierarchy!(IEnumInputContext, windows_core::IUnknown);
impl IEnumInputContext {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumInputContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rginputcontext: *mut HIMC, pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rginputcontext as _, pcfetched as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumInputContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut HIMC, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumInputContext_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumInputContext>;
    fn Next(&self, ulcount: u32, rginputcontext: *mut HIMC, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumInputContext_Vtbl {
    pub const fn new<Identity: IEnumInputContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumInputContext_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rginputcontext: *mut HIMC, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumInputContext_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rginputcontext), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumInputContext_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumInputContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumInputContext_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumInputContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumInputContext {}
windows_core::imp::define_interface!(IEnumRegisterWordA, IEnumRegisterWordA_Vtbl, 0x08c03412_f96b_11d0_a475_00aa006bcc59);
windows_core::imp::interface_hierarchy!(IEnumRegisterWordA, windows_core::IUnknown);
impl IEnumRegisterWordA {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRegisterWordA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rgregisterword as _, pcfetched as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut REGISTERWORDA, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumRegisterWordA_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumRegisterWordA>;
    fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumRegisterWordA_Vtbl {
    pub const fn new<Identity: IEnumRegisterWordA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumRegisterWordA_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRegisterWordA_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgregisterword), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRegisterWordA_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumRegisterWordA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRegisterWordA_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRegisterWordA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumRegisterWordA {}
windows_core::imp::define_interface!(IEnumRegisterWordW, IEnumRegisterWordW_Vtbl, 0x4955dd31_b159_11d0_8fcf_00aa006bcc59);
windows_core::imp::interface_hierarchy!(IEnumRegisterWordW, windows_core::IUnknown);
impl IEnumRegisterWordW {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumRegisterWordW> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ulcount, rgregisterword as _, pcfetched as _).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), ulcount).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRegisterWordW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut REGISTERWORDW, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumRegisterWordW_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumRegisterWordW>;
    fn Next(&self, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Skip(&self, ulcount: u32) -> windows_core::Result<()>;
}
impl IEnumRegisterWordW_Vtbl {
    pub const fn new<Identity: IEnumRegisterWordW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumRegisterWordW_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRegisterWordW_Impl::Next(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&rgregisterword), core::mem::transmute_copy(&pcfetched)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRegisterWordW_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumRegisterWordW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumRegisterWordW_Impl::Skip(this, core::mem::transmute_copy(&ulcount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumRegisterWordW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumRegisterWordW {}
pub const IFEC_S_ALREADY_DEFAULT: windows_core::HRESULT = windows_core::HRESULT(0x47400_u32 as _);
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
#[repr(C)]
#[doc(hidden)]
pub struct IFEClassFactory_Vtbl {
    pub base__: super::super::super::System::Com::IClassFactory_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFEClassFactory_Impl: super::super::super::System::Com::IClassFactory_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl IFEClassFactory_Vtbl {
    pub const fn new<Identity: IFEClassFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::super::System::Com::IClassFactory_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFEClassFactory as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFEClassFactory {}
windows_core::imp::define_interface!(IFECommon, IFECommon_Vtbl, 0x019f7151_e6db_11d0_83c3_00c04fddb82e);
windows_core::imp::interface_hierarchy!(IFECommon, windows_core::IUnknown);
impl IFECommon {
    pub unsafe fn IsDefaultIME(&self, szname: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IsDefaultIME)(windows_core::Interface::as_raw(self), core::mem::transmute(szname.as_ptr()), szname.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetDefaultIME(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultIME)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvokeWordRegDialog)(windows_core::Interface::as_raw(self), pimedlg as _).ok() }
    }
    pub unsafe fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvokeDictToolDialog)(windows_core::Interface::as_raw(self), pimedlg as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFECommon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDefaultIME: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, i32) -> windows_core::HRESULT,
    pub SetDefaultIME: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeWordRegDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEDLG) -> windows_core::HRESULT,
    pub InvokeDictToolDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMEDLG) -> windows_core::HRESULT,
}
pub trait IFECommon_Impl: windows_core::IUnknownImpl {
    fn IsDefaultIME(&self, szname: &windows_core::PCSTR, cszname: i32) -> windows_core::Result<()>;
    fn SetDefaultIME(&self) -> windows_core::Result<()>;
    fn InvokeWordRegDialog(&self, pimedlg: *mut IMEDLG) -> windows_core::Result<()>;
    fn InvokeDictToolDialog(&self, pimedlg: *mut IMEDLG) -> windows_core::Result<()>;
}
impl IFECommon_Vtbl {
    pub const fn new<Identity: IFECommon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDefaultIME<Identity: IFECommon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szname: windows_core::PCSTR, cszname: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFECommon_Impl::IsDefaultIME(this, core::mem::transmute(&szname), core::mem::transmute_copy(&cszname)).into()
            }
        }
        unsafe extern "system" fn SetDefaultIME<Identity: IFECommon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFECommon_Impl::SetDefaultIME(this).into()
            }
        }
        unsafe extern "system" fn InvokeWordRegDialog<Identity: IFECommon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimedlg: *mut IMEDLG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFECommon_Impl::InvokeWordRegDialog(this, core::mem::transmute_copy(&pimedlg)).into()
            }
        }
        unsafe extern "system" fn InvokeDictToolDialog<Identity: IFECommon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimedlg: *mut IMEDLG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFECommon_Impl::InvokeDictToolDialog(this, core::mem::transmute_copy(&pimedlg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDefaultIME: IsDefaultIME::<Identity, OFFSET>,
            SetDefaultIME: SetDefaultIME::<Identity, OFFSET>,
            InvokeWordRegDialog: InvokeWordRegDialog::<Identity, OFFSET>,
            InvokeDictToolDialog: InvokeDictToolDialog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFECommon as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFECommon {}
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
windows_core::imp::define_interface!(IFEDictionary, IFEDictionary_Vtbl, 0x019f7153_e6db_11d0_83c3_00c04fddb82e);
windows_core::imp::interface_hierarchy!(IFEDictionary, windows_core::IUnknown);
impl IFEDictionary {
    pub unsafe fn Open(&self, pchdictpath: Option<&mut [u8; 260]>, pshf: *mut IMESHF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute(pchdictpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pshf as _).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetHeader(&self, pchdictpath: Option<&mut [u8; 260]>, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHeader)(windows_core::Interface::as_raw(self), core::mem::transmute(pchdictpath.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pshf as _, pjfmt as _, pultype as _).ok() }
    }
    pub unsafe fn DisplayProperty(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayProperty)(windows_core::Interface::as_raw(self), hwnd).ok() }
    }
    pub unsafe fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPosTable)(windows_core::Interface::as_raw(self), prgpostbl as _, pcpostbl as _).ok() }
    }
    pub unsafe fn GetWords<P0, P1, P2>(&self, pwchfirst: P0, pwchlast: P1, pwchdisplay: P2, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetWords)(windows_core::Interface::as_raw(self), pwchfirst.param().abi(), pwchlast.param().abi(), pwchdisplay.param().abi(), ulpos, ulselect, ulwordsrc, pchbuffer as _, cbbuffer, pcwrd as _).ok() }
    }
    pub unsafe fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NextWords)(windows_core::Interface::as_raw(self), pchbuffer as _, cbbuffer, pcwrd as _).ok() }
    }
    pub unsafe fn Create<P0>(&self, pchdictpath: P0, pshf: *mut IMESHF) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), pchdictpath.param().abi(), pshf as _).ok() }
    }
    pub unsafe fn SetHeader(&self, pshf: *mut IMESHF) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHeader)(windows_core::Interface::as_raw(self), pshf as _).ok() }
    }
    pub unsafe fn ExistWord(&self, pwrd: *mut IMEWRD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExistWord)(windows_core::Interface::as_raw(self), pwrd as _) }
    }
    pub unsafe fn ExistDependency(&self, pdp: *mut IMEDP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExistDependency)(windows_core::Interface::as_raw(self), pdp as _).ok() }
    }
    pub unsafe fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterWord)(windows_core::Interface::as_raw(self), reg, pwrd as _).ok() }
    }
    pub unsafe fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterDependency)(windows_core::Interface::as_raw(self), reg, pdp as _).ok() }
    }
    pub unsafe fn GetDependencies<P0, P1, P3, P4>(&self, pwchkakarireading: P0, pwchkakaridisplay: P1, ulkakaripos: u32, pwchukereading: P3, pwchukedisplay: P4, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDependencies)(windows_core::Interface::as_raw(self), pwchkakarireading.param().abi(), pwchkakaridisplay.param().abi(), ulkakaripos, pwchukereading.param().abi(), pwchukedisplay.param().abi(), ulukepos, jrel, ulwordsrc, pchbuffer as _, cbbuffer, pcdp as _).ok() }
    }
    pub unsafe fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NextDependencies)(windows_core::Interface::as_raw(self), pchbuffer as _, cbbuffer, pcdp as _).ok() }
    }
    pub unsafe fn ConvertFromOldMSIME<P0>(&self, pchdic: P0, pfnlog: PFNLOG, reg: IMEREG) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConvertFromOldMSIME)(windows_core::Interface::as_raw(self), pchdic.param().abi(), pfnlog, reg).ok() }
    }
    pub unsafe fn ConvertFromUserToSys(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConvertFromUserToSys)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IFEDictionary_Impl: windows_core::IUnknownImpl {
    fn Open(&self, pchdictpath: windows_core::PSTR, pshf: *mut IMESHF) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetHeader(&self, pchdictpath: windows_core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> windows_core::Result<()>;
    fn DisplayProperty(&self, hwnd: super::super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetPosTable(&self, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> windows_core::Result<()>;
    fn GetWords(&self, pwchfirst: &windows_core::PCWSTR, pwchlast: &windows_core::PCWSTR, pwchdisplay: &windows_core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::Result<()>;
    fn NextWords(&self, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::Result<()>;
    fn Create(&self, pchdictpath: &windows_core::PCSTR, pshf: *mut IMESHF) -> windows_core::Result<()>;
    fn SetHeader(&self, pshf: *mut IMESHF) -> windows_core::Result<()>;
    fn ExistWord(&self, pwrd: *mut IMEWRD) -> windows_core::HRESULT;
    fn ExistDependency(&self, pdp: *mut IMEDP) -> windows_core::Result<()>;
    fn RegisterWord(&self, reg: IMEREG, pwrd: *mut IMEWRD) -> windows_core::Result<()>;
    fn RegisterDependency(&self, reg: IMEREG, pdp: *mut IMEDP) -> windows_core::Result<()>;
    fn GetDependencies(&self, pwchkakarireading: &windows_core::PCWSTR, pwchkakaridisplay: &windows_core::PCWSTR, ulkakaripos: u32, pwchukereading: &windows_core::PCWSTR, pwchukedisplay: &windows_core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::Result<()>;
    fn NextDependencies(&self, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::Result<()>;
    fn ConvertFromOldMSIME(&self, pchdic: &windows_core::PCSTR, pfnlog: PFNLOG, reg: IMEREG) -> windows_core::Result<()>;
    fn ConvertFromUserToSys(&self) -> windows_core::Result<()>;
}
impl IFEDictionary_Vtbl {
    pub const fn new<Identity: IFEDictionary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchdictpath: windows_core::PSTR, pshf: *mut IMESHF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::Open(this, core::mem::transmute_copy(&pchdictpath), core::mem::transmute_copy(&pshf)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetHeader<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchdictpath: windows_core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::GetHeader(this, core::mem::transmute_copy(&pchdictpath), core::mem::transmute_copy(&pshf), core::mem::transmute_copy(&pjfmt), core::mem::transmute_copy(&pultype)).into()
            }
        }
        unsafe extern "system" fn DisplayProperty<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::DisplayProperty(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetPosTable<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::GetPosTable(this, core::mem::transmute_copy(&prgpostbl), core::mem::transmute_copy(&pcpostbl)).into()
            }
        }
        unsafe extern "system" fn GetWords<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchfirst: windows_core::PCWSTR, pwchlast: windows_core::PCWSTR, pwchdisplay: windows_core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::GetWords(this, core::mem::transmute(&pwchfirst), core::mem::transmute(&pwchlast), core::mem::transmute(&pwchdisplay), core::mem::transmute_copy(&ulpos), core::mem::transmute_copy(&ulselect), core::mem::transmute_copy(&ulwordsrc), core::mem::transmute_copy(&pchbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcwrd)).into()
            }
        }
        unsafe extern "system" fn NextWords<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::NextWords(this, core::mem::transmute_copy(&pchbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcwrd)).into()
            }
        }
        unsafe extern "system" fn Create<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchdictpath: windows_core::PCSTR, pshf: *mut IMESHF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::Create(this, core::mem::transmute(&pchdictpath), core::mem::transmute_copy(&pshf)).into()
            }
        }
        unsafe extern "system" fn SetHeader<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pshf: *mut IMESHF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::SetHeader(this, core::mem::transmute_copy(&pshf)).into()
            }
        }
        unsafe extern "system" fn ExistWord<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrd: *mut IMEWRD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::ExistWord(this, core::mem::transmute_copy(&pwrd))
            }
        }
        unsafe extern "system" fn ExistDependency<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdp: *mut IMEDP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::ExistDependency(this, core::mem::transmute_copy(&pdp)).into()
            }
        }
        unsafe extern "system" fn RegisterWord<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::RegisterWord(this, core::mem::transmute_copy(&reg), core::mem::transmute_copy(&pwrd)).into()
            }
        }
        unsafe extern "system" fn RegisterDependency<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::RegisterDependency(this, core::mem::transmute_copy(&reg), core::mem::transmute_copy(&pdp)).into()
            }
        }
        unsafe extern "system" fn GetDependencies<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwchkakarireading: windows_core::PCWSTR, pwchkakaridisplay: windows_core::PCWSTR, ulkakaripos: u32, pwchukereading: windows_core::PCWSTR, pwchukedisplay: windows_core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::GetDependencies(this, core::mem::transmute(&pwchkakarireading), core::mem::transmute(&pwchkakaridisplay), core::mem::transmute_copy(&ulkakaripos), core::mem::transmute(&pwchukereading), core::mem::transmute(&pwchukedisplay), core::mem::transmute_copy(&ulukepos), core::mem::transmute_copy(&jrel), core::mem::transmute_copy(&ulwordsrc), core::mem::transmute_copy(&pchbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcdp)).into()
            }
        }
        unsafe extern "system" fn NextDependencies<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::NextDependencies(this, core::mem::transmute_copy(&pchbuffer), core::mem::transmute_copy(&cbbuffer), core::mem::transmute_copy(&pcdp)).into()
            }
        }
        unsafe extern "system" fn ConvertFromOldMSIME<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchdic: windows_core::PCSTR, pfnlog: PFNLOG, reg: IMEREG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::ConvertFromOldMSIME(this, core::mem::transmute(&pchdic), core::mem::transmute_copy(&pfnlog), core::mem::transmute_copy(&reg)).into()
            }
        }
        unsafe extern "system" fn ConvertFromUserToSys<Identity: IFEDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFEDictionary_Impl::ConvertFromUserToSys(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetHeader: GetHeader::<Identity, OFFSET>,
            DisplayProperty: DisplayProperty::<Identity, OFFSET>,
            GetPosTable: GetPosTable::<Identity, OFFSET>,
            GetWords: GetWords::<Identity, OFFSET>,
            NextWords: NextWords::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            SetHeader: SetHeader::<Identity, OFFSET>,
            ExistWord: ExistWord::<Identity, OFFSET>,
            ExistDependency: ExistDependency::<Identity, OFFSET>,
            RegisterWord: RegisterWord::<Identity, OFFSET>,
            RegisterDependency: RegisterDependency::<Identity, OFFSET>,
            GetDependencies: GetDependencies::<Identity, OFFSET>,
            NextDependencies: NextDependencies::<Identity, OFFSET>,
            ConvertFromOldMSIME: ConvertFromOldMSIME::<Identity, OFFSET>,
            ConvertFromUserToSys: ConvertFromUserToSys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFEDictionary as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFEDictionary {}
windows_core::imp::define_interface!(IFELanguage, IFELanguage_Vtbl, 0x019f7152_e6db_11d0_83c3_00c04fddb82e);
windows_core::imp::interface_hierarchy!(IFELanguage, windows_core::IUnknown);
impl IFELanguage {
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetJMorphResult<P3>(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: P3, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetJMorphResult)(windows_core::Interface::as_raw(self), dwrequest, dwcmode, cwchinput, pwchinput.param().abi(), pfcinfo as _, ppresult as _).ok() }
    }
    pub unsafe fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConversionModeCaps)(windows_core::Interface::as_raw(self), pdwcaps as _).ok() }
    }
    pub unsafe fn GetPhonetic(&self, string: &windows_core::BSTR, start: i32, length: i32, phonetic: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPhonetic)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(string), start, length, core::mem::transmute(phonetic)).ok() }
    }
    pub unsafe fn GetConversion(&self, string: &windows_core::BSTR, start: i32, length: i32, result: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConversion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(string), start, length, core::mem::transmute(result)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFELanguage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetJMorphResult: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32, windows_core::PCWSTR, *mut u32, *mut *mut MORRSLT) -> windows_core::HRESULT,
    pub GetConversionModeCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPhonetic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConversion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IFELanguage_Impl: windows_core::IUnknownImpl {
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetJMorphResult(&self, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: &windows_core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> windows_core::Result<()>;
    fn GetConversionModeCaps(&self, pdwcaps: *mut u32) -> windows_core::Result<()>;
    fn GetPhonetic(&self, string: &windows_core::BSTR, start: i32, length: i32, phonetic: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetConversion(&self, string: &windows_core::BSTR, start: i32, length: i32, result: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl IFELanguage_Vtbl {
    pub const fn new<Identity: IFELanguage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IFELanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFELanguage_Impl::Open(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IFELanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFELanguage_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetJMorphResult<Identity: IFELanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: windows_core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFELanguage_Impl::GetJMorphResult(this, core::mem::transmute_copy(&dwrequest), core::mem::transmute_copy(&dwcmode), core::mem::transmute_copy(&cwchinput), core::mem::transmute(&pwchinput), core::mem::transmute_copy(&pfcinfo), core::mem::transmute_copy(&ppresult)).into()
            }
        }
        unsafe extern "system" fn GetConversionModeCaps<Identity: IFELanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcaps: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFELanguage_Impl::GetConversionModeCaps(this, core::mem::transmute_copy(&pdwcaps)).into()
            }
        }
        unsafe extern "system" fn GetPhonetic<Identity: IFELanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: *mut core::ffi::c_void, start: i32, length: i32, phonetic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFELanguage_Impl::GetPhonetic(this, core::mem::transmute(&string), core::mem::transmute_copy(&start), core::mem::transmute_copy(&length), core::mem::transmute_copy(&phonetic)).into()
            }
        }
        unsafe extern "system" fn GetConversion<Identity: IFELanguage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: *mut core::ffi::c_void, start: i32, length: i32, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFELanguage_Impl::GetConversion(this, core::mem::transmute(&string), core::mem::transmute_copy(&start), core::mem::transmute_copy(&length), core::mem::transmute_copy(&result)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetJMorphResult: GetJMorphResult::<Identity, OFFSET>,
            GetConversionModeCaps: GetConversionModeCaps::<Identity, OFFSET>,
            GetPhonetic: GetPhonetic::<Identity, OFFSET>,
            GetConversion: GetConversion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFELanguage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFELanguage {}
pub const IGIMIF_RIGHTMENU: u32 = 1u32;
pub const IGIMII_CMODE: u32 = 1u32;
pub const IGIMII_CONFIGURE: u32 = 4u32;
pub const IGIMII_HELP: u32 = 16u32;
pub const IGIMII_INPUTTOOLS: u32 = 64u32;
pub const IGIMII_OTHER: u32 = 32u32;
pub const IGIMII_SMODE: u32 = 2u32;
pub const IGIMII_TOOLS: u32 = 8u32;
windows_core::imp::define_interface!(IImePad, IImePad_Vtbl, 0x5d8e643a_c3a9_11d1_afef_00805f0c8b6d);
windows_core::imp::interface_hierarchy!(IImePad, windows_core::IUnknown);
impl IImePad {
    pub unsafe fn Request<P0>(&self, piimepadapplet: P0, reqid: IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IImePadApplet>,
    {
        unsafe { (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), piimepadapplet.param().abi(), reqid.0 as _, wparam, lparam).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePad_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, super::super::super::Foundation::WPARAM, super::super::super::Foundation::LPARAM) -> windows_core::HRESULT,
}
pub trait IImePad_Impl: windows_core::IUnknownImpl {
    fn Request(&self, piimepadapplet: windows_core::Ref<IImePadApplet>, reqid: &IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
impl IImePad_Vtbl {
    pub const fn new<Identity: IImePad_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Request<Identity: IImePad_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piimepadapplet: *mut core::ffi::c_void, reqid: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePad_Impl::Request(this, core::mem::transmute_copy(&piimepadapplet), core::mem::transmute(&reqid), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Request: Request::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImePad as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IImePad {}
windows_core::imp::define_interface!(IImePadApplet, IImePadApplet_Vtbl, 0x5d8e643b_c3a9_11d1_afef_00805f0c8b6d);
windows_core::imp::interface_hierarchy!(IImePadApplet, windows_core::IUnknown);
impl IImePadApplet {
    pub unsafe fn Initialize<P0>(&self, lpiimepad: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), lpiimepad.param().abi()).ok() }
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppletConfig)(windows_core::Interface::as_raw(self), lpappletcfg as _).ok() }
    }
    pub unsafe fn CreateUI(&self, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateUI)(windows_core::Interface::as_raw(self), hwndparent, lpimeappletui as _).ok() }
    }
    pub unsafe fn Notify<P0>(&self, lpimepad: P0, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), lpimepad.param().abi(), notify, wparam, lparam).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IImePadApplet_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, lpiimepad: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
    fn GetAppletConfig(&self, lpappletcfg: *mut IMEAPPLETCFG) -> windows_core::Result<()>;
    fn CreateUI(&self, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> windows_core::Result<()>;
    fn Notify(&self, lpimepad: windows_core::Ref<windows_core::IUnknown>, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IImePadApplet_Vtbl {
    pub const fn new<Identity: IImePadApplet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IImePadApplet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiimepad: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePadApplet_Impl::Initialize(this, core::mem::transmute_copy(&lpiimepad)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IImePadApplet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePadApplet_Impl::Terminate(this).into()
            }
        }
        unsafe extern "system" fn GetAppletConfig<Identity: IImePadApplet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePadApplet_Impl::GetAppletConfig(this, core::mem::transmute_copy(&lpappletcfg)).into()
            }
        }
        unsafe extern "system" fn CreateUI<Identity: IImePadApplet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePadApplet_Impl::CreateUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&lpimeappletui)).into()
            }
        }
        unsafe extern "system" fn Notify<Identity: IImePadApplet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpimepad: *mut core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePadApplet_Impl::Notify(this, core::mem::transmute_copy(&lpimepad), core::mem::transmute_copy(&notify), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            GetAppletConfig: GetAppletConfig::<Identity, OFFSET>,
            CreateUI: CreateUI::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImePadApplet as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IImePadApplet {}
windows_core::imp::define_interface!(IImePlugInDictDictionaryList, IImePlugInDictDictionaryList_Vtbl, 0x98752974_b0a6_489b_8f6f_bff3769c8eeb);
windows_core::imp::interface_hierarchy!(IImePlugInDictDictionaryList, windows_core::IUnknown);
impl IImePlugInDictDictionaryList {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDictionariesInUse)(windows_core::Interface::as_raw(self), prgdictionaryguid as _, prgdatecreated as _, prgfencrypted as _).ok() }
    }
    pub unsafe fn DeleteDictionary(&self, bstrdictionaryguid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteDictionary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdictionaryguid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImePlugInDictDictionaryList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDictionariesInUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::super::System::Com::SAFEARRAY, *mut *mut super::super::super::System::Com::SAFEARRAY, *mut *mut super::super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDictionariesInUse: usize,
    pub DeleteDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IImePlugInDictDictionaryList_Impl: windows_core::IUnknownImpl {
    fn GetDictionariesInUse(&self, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn DeleteDictionary(&self, bstrdictionaryguid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IImePlugInDictDictionaryList_Vtbl {
    pub const fn new<Identity: IImePlugInDictDictionaryList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDictionariesInUse<Identity: IImePlugInDictDictionaryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePlugInDictDictionaryList_Impl::GetDictionariesInUse(this, core::mem::transmute_copy(&prgdictionaryguid), core::mem::transmute_copy(&prgdatecreated), core::mem::transmute_copy(&prgfencrypted)).into()
            }
        }
        unsafe extern "system" fn DeleteDictionary<Identity: IImePlugInDictDictionaryList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdictionaryguid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImePlugInDictDictionaryList_Impl::DeleteDictionary(this, core::mem::transmute(&bstrdictionaryguid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDictionariesInUse: GetDictionariesInUse::<Identity, OFFSET>,
            DeleteDictionary: DeleteDictionary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImePlugInDictDictionaryList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IImePlugInDictDictionaryList {}
windows_core::imp::define_interface!(IImeSpecifyApplets, IImeSpecifyApplets_Vtbl, 0x5d8e643c_c3a9_11d1_afef_00805f0c8b6d);
windows_core::imp::interface_hierarchy!(IImeSpecifyApplets, windows_core::IUnknown);
impl IImeSpecifyApplets {
    pub unsafe fn GetAppletIIDList(&self, refiid: *const windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAppletIIDList)(windows_core::Interface::as_raw(self), refiid, lpiidlist as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImeSpecifyApplets_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAppletIIDList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut APPLETIDLIST) -> windows_core::HRESULT,
}
pub trait IImeSpecifyApplets_Impl: windows_core::IUnknownImpl {
    fn GetAppletIIDList(&self, refiid: *const windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> windows_core::Result<()>;
}
impl IImeSpecifyApplets_Vtbl {
    pub const fn new<Identity: IImeSpecifyApplets_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAppletIIDList<Identity: IImeSpecifyApplets_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, refiid: *const windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImeSpecifyApplets_Impl::GetAppletIIDList(this, core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&lpiidlist)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAppletIIDList: GetAppletIIDList::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImeSpecifyApplets as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IImeSpecifyApplets {}
pub type IMCENUMPROC = Option<unsafe extern "system" fn(param0: HIMC, param1: super::super::super::Foundation::LPARAM) -> windows_core::BOOL>;
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
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for IMEAPPLETCFG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMECHARINFO {
    pub wch: u16,
    pub dwCharInfo: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMECHARPOSITION {
    pub dwSize: u32,
    pub dwCharPos: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub cLineHeight: u32,
    pub rcDocument: super::super::super::Foundation::RECT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMECOMPOSITIONSTRINGINFO {
    pub iCompStrLen: i32,
    pub iCaretPos: i32,
    pub iEditStart: i32,
    pub iEditLen: i32,
    pub iTargetStart: i32,
    pub iTargetLen: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMEDLG {
    pub cbIMEDLG: i32,
    pub hwnd: super::super::super::Foundation::HWND,
    pub lpwstrWord: windows_core::PWSTR,
    pub nTabId: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEDP {
    pub wrdModifier: IMEWRD,
    pub wrdModifiee: IMEWRD,
    pub relID: IMEREL,
}
impl Default for IMEDP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEFAREASTINFO {
    pub dwSize: u32,
    pub dwType: u32,
    pub dwData: [u32; 1],
}
impl Default for IMEFAREASTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEFAREASTINFO_TYPE_COMMENT: u32 = 2u32;
pub const IMEFAREASTINFO_TYPE_COSTTIME: u32 = 3u32;
pub const IMEFAREASTINFO_TYPE_DEFAULT: u32 = 0u32;
pub const IMEFAREASTINFO_TYPE_READING: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMEFMT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMEINFO {
    pub dwPrivateDataSize: u32,
    pub fdwProperty: u32,
    pub fdwConversionCaps: u32,
    pub fdwSentenceCaps: u32,
    pub fdwUICaps: u32,
    pub fdwSCSCaps: u32,
    pub fdwSelectCaps: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEITEM {
    pub cbSize: i32,
    pub iType: i32,
    pub lpItemData: *mut core::ffi::c_void,
}
impl Default for IMEITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMEITEMCANDIDATE {
    pub uCount: u32,
    pub imeItem: [IMEITEM; 1],
}
impl Default for IMEITEMCANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMEKEYCTRLMASK_ALT: u32 = 1u32;
pub const IMEKEYCTRLMASK_CTRL: u32 = 2u32;
pub const IMEKEYCTRLMASK_SHIFT: u32 = 4u32;
pub const IMEKEYCTRL_DOWN: u32 = 0u32;
pub const IMEKEYCTRL_UP: u32 = 1u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMEKMS {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub cKeyList: u32,
    pub pKeyList: *mut IMEKMSKEY,
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
impl Default for IMEKMSFUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMEKMSINIT {
    pub cbSize: i32,
    pub hWnd: super::super::super::Foundation::HWND,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMEKMSINVK {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub dwControl: u32,
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
impl Default for IMEKMSKMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMEKMSNTFY {
    pub cbSize: i32,
    pub hIMC: HIMC,
    pub fSelect: windows_core::BOOL,
}
pub const IMEKMS_2NDLEVEL: u32 = 4u32;
pub const IMEKMS_CANDIDATE: u32 = 6u32;
pub const IMEKMS_COMPOSITION: u32 = 1u32;
pub const IMEKMS_IMEOFF: u32 = 3u32;
pub const IMEKMS_INPTGL: u32 = 5u32;
pub const IMEKMS_NOCOMPOSITION: u32 = 0u32;
pub const IMEKMS_SELECTION: u32 = 2u32;
pub const IMEKMS_TYPECAND: u32 = 7u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for IMEMENUITEMINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for IMEMENUITEMINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMEREG(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMEREL(pub i32);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IMESHF {
    pub cbShf: u16,
    pub verDic: u16,
    pub szTitle: [i8; 48],
    pub szDescription: [i8; 256],
    pub szCopyright: [i8; 128],
}
impl Default for IMESHF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMESTRINGCANDIDATE {
    pub uCount: u32,
    pub lpwstr: [windows_core::PWSTR; 1],
}
impl Default for IMESTRINGCANDIDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMESTRINGCANDIDATEINFO {
    pub dwFarEastId: u32,
    pub lpFarEastInfo: *mut IMEFAREASTINFO,
    pub fInfoMask: u32,
    pub iSelIndex: i32,
    pub uCount: u32,
    pub lpwstr: [windows_core::PWSTR; 1],
}
impl Default for IMESTRINGCANDIDATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMESTRINGINFO {
    pub dwFarEastId: u32,
    pub lpwstr: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMEUCT(pub i32);
pub const IMEVER_0310: u32 = 196618u32;
pub const IMEVER_0400: u32 = 262144u32;
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
impl Default for IMEWRD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMEWRD_0_0 {
    pub nPos1: u16,
    pub nPos2: u16,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IME_COMPOSITION_STRING(pub u32);
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
pub const IME_CONFIG_GENERAL: u32 = 1u32;
pub const IME_CONFIG_REGISTERWORD: u32 = 2u32;
pub const IME_CONFIG_SELECTDICTIONARY: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IME_CONVERSION_MODE(pub u32);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IME_ESCAPE(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IME_HOTKEY_IDENTIFIER(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IME_PAD_REQUEST_FLAGS(pub u32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IME_SENTENCE_MODE(pub u32);
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
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct INPUTCONTEXT {
    pub hWnd: super::super::super::Foundation::HWND,
    pub fOpen: windows_core::BOOL,
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
impl Default for INPUTCONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl Default for MORRSLT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NOTIFY_IME_ACTION(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NOTIFY_IME_INDEX(pub u32);
pub type PFNLOG = Option<unsafe extern "system" fn(param0: *mut IMEDP, param1: windows_core::HRESULT) -> windows_core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct POSTBL {
    pub nPos: u16,
    pub szName: *mut u8,
}
impl Default for POSTBL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POS_UNDEFINED: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const RECONVOPT_NONE: u32 = 0u32;
pub const RECONVOPT_USECANCELNOTIFY: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REGISTERWORDA {
    pub lpReading: windows_core::PSTR,
    pub lpWord: windows_core::PSTR,
}
pub type REGISTERWORDENUMPROCA = Option<unsafe extern "system" fn(lpszreading: windows_core::PCSTR, param1: u32, lpszstring: windows_core::PCSTR, param3: *mut core::ffi::c_void) -> i32>;
pub type REGISTERWORDENUMPROCW = Option<unsafe extern "system" fn(lpszreading: windows_core::PCWSTR, param1: u32, lpszstring: windows_core::PCWSTR, param3: *mut core::ffi::c_void) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REGISTERWORDW {
    pub lpReading: windows_core::PWSTR,
    pub lpWord: windows_core::PWSTR,
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SET_COMPOSITION_STRING_TYPE(pub u32);
pub const SHOWIMEPAD_CATEGORY: u32 = 1u32;
pub const SHOWIMEPAD_DEFAULT: u32 = 0u32;
pub const SHOWIMEPAD_GUID: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOFTKBDDATA {
    pub uCount: u32,
    pub wCode: [u16; 256],
}
impl Default for SOFTKBDDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SOFTKEYBOARD_TYPE_C1: u32 = 2u32;
pub const SOFTKEYBOARD_TYPE_T1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STYLEBUFA {
    pub dwStyle: u32,
    pub szDescription: [i8; 32],
}
impl Default for STYLEBUFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STYLEBUFW {
    pub dwStyle: u32,
    pub szDescription: [u16; 32],
}
impl Default for STYLEBUFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STYLE_DESCRIPTION_SIZE: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSMSG {
    pub message: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSMSGLIST {
    pub uMsgCount: u32,
    pub TransMsg: [TRANSMSG; 1],
}
impl Default for TRANSMSGLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl Default for WDD_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const cbCommentMax: u32 = 256u32;
pub type fpCreateIFECommonInstanceType = Option<unsafe extern "system" fn(ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type fpCreateIFEDictionaryInstanceType = Option<unsafe extern "system" fn(ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type fpCreateIFELanguageInstanceType = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub const szImeChina: windows_core::PCWSTR = windows_core::w!("MSIME.China");
pub const szImeJapan: windows_core::PCWSTR = windows_core::w!("MSIME.Japan");
pub const szImeKorea: windows_core::PCWSTR = windows_core::w!("MSIME.Korea");
pub const szImeTaiwan: windows_core::PCWSTR = windows_core::w!("MSIME.Taiwan");
pub const wchPrivate1: u32 = 57344u32;
